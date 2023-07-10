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
    #[table(key, editable)]
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

    mount_to_body(|cx| {
        let items = create_rw_signal(
            cx,
            vec![
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
            ],
        );

        view! { cx,
            <BookTable items=items />
            <button on:click= move |_| log::debug!("{:#?}", items.get_untracked())>"Log current state to console"</button>
        }
    })
}
