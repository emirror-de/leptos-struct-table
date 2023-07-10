use async_trait::async_trait;
use chrono::{DateTime, Utc};
use leptos::*;
use leptos_struct_table::*;
use serde::{Deserialize, Serialize};

// This generates the component BookTable
#[derive(TableComponent, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[table(sortable)]
pub struct Book {
    #[table(key, editable)]
    pub id: u32,
    pub title: String,
    #[table(editable)]
    pub author: String,
    #[table(editable, format(string = "%Y-%m-%d %H:%M:%S %z"))]
    pub publish_date: DateTime<Utc>,
    #[table(skip)]
    pub hidden_field: String,
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|cx| {
        let items = create_rw_signal(
            cx,
            vec![
                Book {
                    id: 1,
                    title: "The Great Gatsby".to_string(),
                    author: "F. Scott Fitzgerald".to_string(),
                    publish_date: DateTime::parse_from_rfc3339("1996-12-19T16:39:57+00:00")
                        .unwrap()
                        .into(),
                    hidden_field: "hidden".to_string(),
                },
                Book {
                    id: 2,
                    title: "The Grapes of Wrath".to_string(),
                    author: "John Steinbeck".to_string(),
                    publish_date: DateTime::parse_from_rfc3339("1996-12-19T16:39:57+00:00")
                        .unwrap()
                        .into(),
                    hidden_field: "not visible in the table".to_string(),
                },
                Book {
                    id: 3,
                    title: "Nineteen Eighty-Four".to_string(),
                    author: "George Orwell".to_string(),
                    publish_date: DateTime::parse_from_rfc3339("1996-12-19T16:39:57+00:00")
                        .unwrap()
                        .into(),
                    hidden_field: "hidden".to_string(),
                },
                Book {
                    id: 4,
                    title: "Ulysses".to_string(),
                    author: "James Joyce".to_string(),
                    publish_date: DateTime::parse_from_rfc3339("1996-12-19T16:39:57+00:00")
                        .unwrap()
                        .into(),
                    hidden_field: "hidden".to_string(),
                },
            ],
        );

        view! { cx,
            <BookTable items=items />
            <button on:click= move |_| log::debug!("{:#?}", items.get_untracked())>"Log current state to console"</button>
        }
    })
}
