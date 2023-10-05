#![deny(missing_docs)]
//! Simple showcase example.

use crate::uuid::Uuid;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use leptos::*;
use leptos_struct_table::*;
use serde::{Deserialize, Serialize};

/// This generates the component BookTable
#[derive(TableComponent, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[table(sortable)]
pub struct Book {
    /// Id of the entry.
    #[table(key)]
    pub id: Uuid,
    /// Title of the book.
    pub title: String,
    /// Author of the book.
    #[table(editable)]
    pub author: String,
    /// Date when book has been published.
    #[table(editable, format(string = "%Y-%m-%d %H:%M:%S %z"))]
    pub publish_date: DateTime<Utc>,
    /// Example on hidden member.
    #[table(skip)]
    pub hidden_field: String,
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        let get_current_provider_state =
            create_action(move |provider: &StoredValue<MemoryStorage<Book>>| {
                let p = provider.get_value();
                async move { p.get_rows(0..1000).await.unwrap() }
            });
        let append_row = create_action(move |provider: &StoredValue<MemoryStorage<Book>>| {
            let mut p = provider.get_value();
            async move {
                log::debug!(
                    "{:?}",
                    p.append_row(Book {
                        id: Uuid::default(),
                        title: "The Great Gatsby".to_string(),
                        author: "F. Scott Fitzgerald".to_string(),
                        publish_date: DateTime::parse_from_rfc3339("1996-12-19T16:39:57+00:00")
                            .unwrap()
                            .into(),
                        hidden_field: "hidden".to_string(),
                    })
                    .await
                );
            }
        });
        let current_provider_state = get_current_provider_state.value();
        let log_provider_state = move || {
            log::debug!("Provider state:\n{:#?}", current_provider_state.get());
        };

        let range_to_show = create_rw_signal(0..4);

        let provider = store_value(MemoryStorage::new(vec![
            Book {
                id: Uuid::default(),
                title: "The Great Gatsby".to_string(),
                author: "F. Scott Fitzgerald".to_string(),
                publish_date: DateTime::parse_from_rfc3339("1996-12-19T16:39:57+00:00")
                    .unwrap()
                    .into(),
                hidden_field: "hidden".to_string(),
            },
            Book {
                id: Uuid::default(),
                title: "The Grapes of Wrath".to_string(),
                author: "John Steinbeck".to_string(),
                publish_date: DateTime::parse_from_rfc3339("1996-12-19T16:39:57+00:00")
                    .unwrap()
                    .into(),
                hidden_field: "not visible in the table".to_string(),
            },
            Book {
                id: Uuid::default(),
                title: "Nineteen Eighty-Four".to_string(),
                author: "George Orwell".to_string(),
                publish_date: DateTime::parse_from_rfc3339("1996-12-19T16:39:57+00:00")
                    .unwrap()
                    .into(),
                hidden_field: "hidden".to_string(),
            },
            Book {
                id: Uuid::default(),
                title: "Ulysses".to_string(),
                author: "James Joyce".to_string(),
                publish_date: DateTime::parse_from_rfc3339("1996-12-19T16:39:57+00:00")
                    .unwrap()
                    .into(),
                hidden_field: "hidden".to_string(),
            },
        ]));

        view! {
            <BookTable data_provider=provider range=range_to_show />
            <button on:click=move |_| {
                get_current_provider_state.dispatch(provider);
            }>{"Log current state to console"}</button>
            <button on:click=move |_| {
                append_row.dispatch(provider);
            }>{"Append first row"}</button>
            { log_provider_state }
        }
    })
}
