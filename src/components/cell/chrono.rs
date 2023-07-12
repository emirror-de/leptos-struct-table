#![allow(unused_variables)]
#![doc(cfg(feature = "chrono"))]

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use leptos::html::Td;
use leptos::*;
use paste::paste;

macro_rules! date_cell_renderer {
    (
        $(#[$outer:meta])*
        $date_type:ident
        $default_date_format:literal
    ) => {
        paste! {
            $(#[$outer])*
            ///
            /// This is only available when the **crate feature `chrono`** is enabled
            #[component]
            pub fn [<Default $date_type TableCellRenderer>]<C> (
                cx: Scope,
                /// The class attribute for the cell element. Generated by the classes provider.
                #[prop(into)] class: MaybeSignal<String>,
                /// The value to display.
                #[prop(into)] value: MaybeSignal<$date_type>,
                /// The index of the column. Starts at 0.
                index: usize,
                /// The format string to use for formatting the date. Provided by the `#[table(format(string="..."))]` attribute of the field.
                /// See [`chrono::format::strftime`] for more information.
                #[prop(optional)] format_string: Option<String>,
                /// Called when the content of the cell has changed.
                on_change: C,
                /// Set this to true to be able to edit the content of the cell.
                editable: bool,
            ) -> impl IntoView
            where
                C: Fn($date_type) + 'static,
            {
                let text = match format_string.clone() {
                    Some(format_string) => create_memo(cx, move |_| value().format(&format_string).to_string()),
                    None => create_memo(cx, move |_| value().to_string()),
                };

                if editable {
                    let td_ref = create_node_ref::<Td>(cx);

                    let on_input = move |_| {
                        if let Some(td) = td_ref.get_untracked() {
                            let value = td.inner_text();
                            let parse_result = match format_string {
                                Some(ref f) => $date_type::parse_from_str(&value, &f.clone()),
                                None => $date_type::parse_from_str(&value, $default_date_format),
                            };
                            if let Ok(v) = parse_result {
                                on_change(v);
                            }
                        }
                    };

                    return view! { cx,
                        <td class={ format!("{} editable", class.get()) } node_ref=td_ref on:input=on_input contenteditable>{text}</td>
                    };
                }

                view! { cx,
                    <td class=class>{text}</td>
                }
            }
        }
    };
}

date_cell_renderer!(
    /// The default cell renderer for [`chrono::NaiveDate`].
    NaiveDate
    "%Y-%m-%d"
);

date_cell_renderer!(
    /// The default cell renderer for [`chrono::NaiveDateTime`].
    NaiveDateTime
    "%Y-%m-%dT%H:%M:%S"
);

date_cell_renderer!(
    /// The default cell renderer for [`chrono::NaiveTime`].
    NaiveTime
    "%H:%M:%S"
);

/// Default cell renderer for [`chrono::DateTime<Utc>`].
///
/// This is only available when the **crate feature `chrono`** is enabled
#[component]
pub fn DefaultDateTimeUtcTableCellRenderer<C>(
    cx: Scope,
    /// The class attribute for the cell element. Generated by the classes provider.
    #[prop(into)]
    class: MaybeSignal<String>,
    /// The value to display.
    #[prop(into)]
    value: MaybeSignal<DateTime<Utc>>,
    /// The index of the column. Starts at 0.
    index: usize,
    /// The format string to use for formatting the date. Provided by the `#[table(format(string="..."))]` attribute of the field.
    /// See [`chrono::format::strftime`] for more information.
    #[prop(optional)]
    format_string: Option<String>,
    /// Called when the content of the cell has changed.
    on_change: C,
    /// Set this to true to be able to edit the content of the cell.
    editable: bool,
) -> impl IntoView
where
    C: Fn(DateTime<Utc>) + 'static,
{
    let text = match format_string.clone() {
        Some(format_string) => create_memo(cx, move |_| value().format(&format_string).to_string()),
        None => create_memo(cx, move |_| value().to_rfc3339()),
    };

    if editable {
        let td_ref = create_node_ref::<Td>(cx);

        let on_input = move |_| {
            if let Some(td) = td_ref.get_untracked() {
                let value = td.inner_text();
                let parse_result = match format_string {
                    Some(ref f) => DateTime::<FixedOffset>::parse_from_str(&value, &f.clone()),
                    None => DateTime::<FixedOffset>::parse_from_str(&value, "%Y-%m-%dT%H:%M:%S%:z"),
                };
                if let Ok(v) = parse_result {
                    on_change(v.into());
                }
            }
        };

        return view! { cx,
            <td class=class node_ref=td_ref on:input=on_input contenteditable>{text}</td>
        };
    }

    view! { cx,
        <td class=class>{text}</td>
    }
}
