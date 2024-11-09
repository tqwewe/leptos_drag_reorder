# Leptos Drag Reorder

Leptos hook for draggable panels which can be rearranged.

This library uses the browsers drag APIs, so it should be very stable.

**Only supports Leptos 0.7**

![Preview GIF](https://github.com/tqwewe/leptos_drag_reorder/blob/main/preview.gif?raw=true)

### Example

Provide drag order context.

```rust
let panel_order = [
    // Column 1
    RwSignal::new(vec!["1".into(), "3".into()]),
    // Column 2
    RwSignal::new(vec!["2".into()]),
];
let column_refs = provide_drag_reorder(panel_order);
```

Use drag reorder in panel component.

```rust
let UseDragReorderReturn {
    node_ref,
    draggable,
    set_draggable,
    hover_position,
    on_dragstart,
    on_dragend,
    ..
} = use_drag_reorder(id.to_string());

// apply node ref, on_dragstart/end, etc.
```

A full example is available in the example directory.
