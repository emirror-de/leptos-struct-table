use async_trait::async_trait;
use core::fmt::Debug;
use std::ops::Range;
use std::sync::Arc;
use std::sync::RwLock;

/// Provides the main interface for a data storage or backend.
/// Implement this on your struct to get and update your data that needs
/// to be shown on a table. The struct itself is required as [`StoredValue`] in
/// the final component and cloned through the `async` context. Make sure that your actual data is not
/// directly stored in it to keep the amount of clones as minimal as possible.
///
/// A basic memory storage implementation can be found in [MemoryStorage].
///
/// Please note that because of the use of [`async-trait`](https://docs.rs/async-trait/latest/async_trait/) this documentation is a bit cluttered.
#[async_trait(?Send)]
pub trait TableDataStorage<T, K>
where
    T: Debug + PartialEq + Clone,
    K: PartialEq,
    Self: Clone,
{
    /// Get all data rows for the table specified by the range.
    /// The range is determined by the visible rows and used to virtualize the table.
    /// The parameter `range` is only determined by visibility and may be out of bounds. It is the
    /// responsibility of the implementation to handle this case. Use [get_vec_range_clamped] to get a
    /// range that is clamped to the length of the vector.
    async fn get_rows(&self, range: Range<usize>) -> anyhow::Result<Vec<T>>;

    /// Updates the value of the row at `index` to the value of `row` in the implementing storage.
    async fn set_row(&mut self, key: K, row: T) -> anyhow::Result<()>;

    /// Appends the row to the end of the table data.
    async fn append_row(&mut self, row: T) -> anyhow::Result<()>;

    /// Removes the row with the given key from the table data.
    async fn remove_row(&mut self, key: K) -> anyhow::Result<()>;
}

/// Properties of an entry in a table.
pub trait TableDataEntry<K>
where
    K: PartialEq + Clone,
{
    /// Cloned unique identifier of the entry.
    fn key(&self) -> K;
}

/// A basic storage implementation that keeps the given data on initialization
/// in memory.
#[derive(Clone)]
pub struct MemoryStorage<T> {
    data: Arc<RwLock<Vec<T>>>,
}

impl<T> MemoryStorage<T> {
    /// Creates a new storage holding the given data as initial value.
    pub fn new(data: Vec<T>) -> Self {
        Self {
            data: Arc::new(RwLock::new(data)),
        }
    }
}

#[async_trait(?Send)]
impl<T, K> TableDataStorage<T, K> for MemoryStorage<T>
where
    T: Debug + PartialEq + Clone + TableDataEntry<K>,
    K: PartialEq + core::fmt::Display + Clone + 'static,
{
    async fn get_rows(&self, range: Range<usize>) -> anyhow::Result<Vec<T>> {
        let read_lock = self.data.try_read().map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok(get_vec_range_clamped(&read_lock, range))
    }

    async fn set_row(&mut self, key: K, row: T) -> anyhow::Result<()> {
        let mut write_lock = self.data.try_write().map_err(|e| anyhow::anyhow!("{e}"))?;
        match write_lock.iter_mut().find(|it| it.key() == key) {
            Some(r) => {
                *r = row;
            }
            None => log::warn!("Could not find row with identifier {key} to update."),
        }
        Ok(())
    }

    async fn append_row(&mut self, row: T) -> anyhow::Result<()> {
        let mut write_lock = self.data.try_write().map_err(|e| anyhow::anyhow!("{e}"))?;
        write_lock.push(row);
        Ok(())
    }

    async fn remove_row(&mut self, key: K) -> anyhow::Result<()> {
        let mut write_lock = self.data.try_write().map_err(|e| anyhow::anyhow!("{e}"))?;
        let idx = write_lock.iter().position(|it| it.key() == key);
        match idx {
            Some(idx) => {
                write_lock.remove(idx);
            }
            None => log::warn!("Could not find row with identifier {key} to update."),
        }
        Ok(())
    }
}

/// Return `vec[range.start..range.end]` where `range` is clamped to the length of `vec`.
pub fn get_vec_range_clamped<T: Clone>(vec: &Vec<T>, range: Range<usize>) -> Vec<T> {
    if vec.is_empty() {
        return vec![];
    }

    let start = range.start.min(vec.len() - 1);
    let end = range.end.min(vec.len());

    vec[start..end].to_vec()
}
