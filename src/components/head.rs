use crate::ColumnSort;
use leptos::ev::MouseEvent;
use leptos::*;

/// Event emitted when a table head cell is clicked.
#[derive(Debug)]
pub struct TableHeadEvent<C: 'static> {
    /// The index of the column. Starts at 0 for the first column. The order of the columns is the same as the order of the fields in the struct.
    pub index: usize,
    /// The column enum variant. It is auto generated from the struct.
    pub column: C,
    /// The mouse event that triggered the event.
    pub mouse_event: MouseEvent,
}

/// The default table header renderer. Renders roughly
/// ```html
/// <th>
///    <span>Title</span>
/// </th>
/// ```
#[component]
pub fn DefaultTableHeaderRenderer<C, F>(
    /// The class attribute for the head element. Generated by the classes provider.
    #[prop(into)]
    class: Signal<String>,
    /// The class attribute for the inner element. Generated by the classes provider.
    #[prop(into)]
    inner_class: String,
    /// The index of the column. Starts at 0 for the first column. The order of the columns is the same as the order of the fields in the struct.
    index: usize,
    /// The column enum variant. It is auto generated from the struct.
    column: C,
    /// The sort priority of the column. `None` if the column is not sorted. `0` means the column is the primary sort column.
    #[prop(into)]
    sort_priority: Signal<Option<usize>>,
    /// The sort direction of the column. See [`ColumnSort`].
    #[prop(into)]
    sort_direction: Signal<ColumnSort>,
    /// The event handler for the click event. Has to be called with [`TableHeadEvent`].
    on_click: F,
    children: Children,
) -> impl IntoView
where
    F: Fn(TableHeadEvent<C>) + 'static,
    C: 'static + Copy,
{
    let style = move || {
        let sort = match sort_direction() {
            ColumnSort::Ascending => "--sort-icon: '▲';",
            ColumnSort::Descending => "--sort-icon: '▼';",
            ColumnSort::None => "--sort-icon: '';",
        };

        let priority = match sort_priority() {
            Some(priority) => format!("--sort-priority: '{}';", priority + 1),
            None => "--sort-priority: '';".to_string(),
        };

        format!("{} {}", sort, &priority)
    };

    view! {
        <th class=class
            on:click=move |mouse_event| on_click(TableHeadEvent {
                index,
                column,
                mouse_event,
            })
            style=style
        >
            <span class=inner_class>
                {children()}
            </span>
        </th>
    }
}
