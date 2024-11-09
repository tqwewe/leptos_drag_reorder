use leptos::{ev, prelude::*};
use leptos_drag_reorder::{
    provide_drag_reorder, use_drag_reorder, HoverPosition, UseDragReorderReturn,
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
    let panels = RwSignal::new(vec![
        Panel {
            id: 1,
            title: "Panel #1".to_string(),
        },
        Panel {
            id: 2,
            title: "Panel #2".to_string(),
        },
        Panel {
            id: 3,
            title: "Panel #3".to_string(),
        },
    ]);
    let panel_order = [
        // Column 1
        RwSignal::new(vec!["1".into(), "3".into()]),
        // Column 2
        RwSignal::new(vec!["2".into()]),
    ];
    let column_refs = provide_drag_reorder(panel_order);

    let columns = panel_order
        .into_iter()
        .zip(column_refs)
        .map(|(ordering, column_ref)| {
            let column_items = move || {
                ordering
                    .read()
                    .iter()
                    .filter_map(|id| {
                        panels
                            .read()
                            .iter()
                            .find(|panel| &panel.id.to_string() == id)
                            .cloned()
                    })
                    .collect::<Vec<_>>()
            };

            view! {
                <div node_ref=column_ref class="column">
                    <For
                        each=column_items
                        key=|item| item.id
                        let:panel
                    >
                        <Panel id=panel.id title=panel.title />
                    </For>
                </div>
            }
        })
        .collect_view();

    let add_panel = {
        move |_: ev::MouseEvent| {
            let mut panels = panels.write();
            let next_id = panels.last().map(|item| item.id).unwrap_or(0) + 1;
            panels.push(Panel {
                id: next_id,
                title: format!("Panel #{next_id}"),
            });
            panel_order[0].update(|order| {
                order.insert(0, next_id.to_string().into());
            });
        }
    };

    view! {
        <div class="root">
            <button on:click=add_panel>"Add Panel"</button>

            <div class="row">
                {columns}
            </div>
        </div>
    }
}

#[component]
fn Panel(id: u32, title: String) -> impl IntoView {
    let UseDragReorderReturn {
        node_ref,
        draggable,
        set_draggable,
        hover_position,
        on_dragstart,
        on_dragend,
        ..
    } = use_drag_reorder(id.to_string());

    view! {
        <div
            node_ref=node_ref
            class="panel"
            class=("panel--above", move || matches!(hover_position.get(), Some(HoverPosition::Above)))
            class=("panel--below", move || matches!(hover_position.get(), Some(HoverPosition::Below)))
            draggable=move || draggable.get().then_some("true")
            on:dragstart=on_dragstart
            on:dragend=on_dragend
            on:mousedown=move |_| set_draggable(true)
        >
            {title}
        </div>
    }
}
