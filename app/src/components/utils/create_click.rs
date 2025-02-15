use leptos::{ev, html, prelude::*};
use wasm_bindgen::{JsCast, UnwrapThrowExt};

/// Create an node that calls `on_close` when a mouse event (click) is detected
/// outside the element.
pub fn create_click(on_close: impl Fn() + Send + Sync + 'static) -> NodeRef<html::Button> {
    let node_ref = NodeRef::<html::Button>::new();

    node_ref.on_load(move |elem| {
        let listener_handle = window_event_listener(ev::click, move |ev| {
            if let Ok(clicked_element) = ev.target().unwrap_throw().dyn_into::<web_sys::Element>() {
                let elem_any = elem.clone();
                if clicked_element != **elem_any && !elem_any.contains(Some(&clicked_element)) {
                    on_close();
                }
            }
        });
        on_cleanup(move || {
            listener_handle.remove();
        });
    });

    node_ref
}
