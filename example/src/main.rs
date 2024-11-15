use leptos::{context::Provider, ev, prelude::*};
use leptos_drag_reorder::{
    provide_drag_reorder, use_drag_reorder, DragReorderContext, HoverPosition, UseDragReorderReturn,
};

fn main() {
    mount_to_body(App)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Panel {
    id: u32,
    title: String,
}

#[component]
fn App() -> impl IntoView {
    let column_panels = RwSignal::new(vec![
        Panel {
            id: 1,
            title: "Column Panel #1".to_string(),
        },
        Panel {
            id: 2,
            title: "Column Panel #2".to_string(),
        },
        Panel {
            id: 3,
            title: "Column Panel #3".to_string(),
        },
    ]);
    let column_panel_order = [
        // Column 1
        RwSignal::new(vec!["1".into(), "3".into()]),
        // Column 2
        RwSignal::new(vec!["2".into()]),
    ];
    let (column_refs, col_order_cxt) = provide_drag_reorder(column_panel_order);

    let add_column_panel = {
        move |_: ev::MouseEvent| {
            let mut panels = column_panels.write();
            let next_id = panels.last().map(|item| item.id).unwrap_or(0) + 1;
            panels.push(Panel {
                id: next_id,
                title: format!("Column Panel #{next_id}"),
            });
            column_panel_order[0].update(|order| {
                order.insert(0, next_id.to_string().into());
            });
        }
    };

    let row_panels = RwSignal::new(vec![
        Panel {
            id: 1,
            title: "Row Panel #1".to_string(),
        },
        Panel {
            id: 2,
            title: "Row Panel #2".to_string(),
        },
        Panel {
            id: 3,
            title: "Row Panel #3".to_string(),
        },
    ]);
    let row_panel_order = [
        // Column 1
        RwSignal::new(vec!["1".into(), "3".into()]),
        // Column 2
        RwSignal::new(vec!["2".into()]),
    ];
    let (row_refs, row_order_cxt) = provide_drag_reorder(row_panel_order);

    let add_row_panel = {
        move |_: ev::MouseEvent| {
            let mut panels = row_panels.write();
            let next_id = panels.last().map(|item| item.id).unwrap_or(0) + 1;
            panels.push(Panel {
                id: next_id,
                title: format!("Row Panel #{next_id}"),
            });
            row_panel_order[0].update(|order| {
                order.insert(0, next_id.to_string().into());
            });
        }
    };
    view! {
        <div class="root">
            <button on:click=add_column_panel>"Add Panel to Columns"</button>
            <Provider value=col_order_cxt>
                <div class="row">
                    {column_panel_order
                        .into_iter()
                        .zip(column_refs)
                        .map(|(ordering, column_ref)| {
                            let column_items = move || {
                                ordering
                                    .read()
                                    .iter()
                                    .filter_map(|id| {
                                        column_panels
                                            .read()
                                            .iter()
                                            .find(|panel| &panel.id.to_string() == id)
                                            .cloned()
                                    })
                                    .collect::<Vec<_>>()
                            };
                            view! {
                                <div node_ref=column_ref class="column">
                                    <For each=column_items key=|item| item.id let:panel>
                                        <Panel id=panel.id title=panel.title transpose=false/>
                                    </For>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>
            </Provider>
            <button on:click=add_row_panel>"Add Panel to rows"</button>
            <Provider value=row_order_cxt>
                {row_panel_order
                    .into_iter()
                    .zip(row_refs)
                    .map(|(ordering, row_ref)| {
                        let row_items = move || {
                            ordering
                                .read()
                                .iter()
                                .filter_map(|id| {
                                    row_panels
                                        .read()
                                        .iter()
                                        .find(|panel| &panel.id.to_string() == id)
                                        .cloned()
                                })
                                .collect::<Vec<_>>()
                        };
                        view! {
                            <div node_ref=row_ref class="row">
                                <For each=row_items key=|item| item.id let:panel>
                                    <Panel id=panel.id title=panel.title transpose=true/>
                                </For>
                            </div>
                        }
                    })
                    .collect_view()}
            </Provider>

        </div>
    }
}

#[component]
fn Panel(id: u32, title: String, transpose: bool) -> impl IntoView {
    let UseDragReorderReturn {
        node_ref,
        draggable,
        set_draggable,
        hover_position,
        on_dragstart,
        on_dragend,
        ..
    } = use_drag_reorder(id.to_string(), transpose);

    view! {
        <div
            node_ref=node_ref
            class="panel"
            class=("row-item", move || transpose)

            class=("col-item", move || !transpose)

            class=(
                "panel--above",
                move || matches!(hover_position.get(), Some(HoverPosition::Above)),
            )

            class=(
                "panel--below",
                move || matches!(hover_position.get(), Some(HoverPosition::Below)),
            )

            draggable=move || draggable.get().then_some("true")
            on:dragstart=on_dragstart
            on:dragend=on_dragend
            on:mousedown=move |_| set_draggable(true)
        >
            {title}
        </div>
    }
}
