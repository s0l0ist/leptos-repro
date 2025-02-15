use crate::components::LoaderIcon;
use leptos::prelude::*;
use web_sys::PointerEvent;

#[component]
pub fn SubmitButton(
    button_text: &'static str,
    #[prop(optional, default = Some("inline-flex items-center w-full text-center justify-center rounded-md px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-xs focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"))]
    button_class: Option<&'static str>,
    #[prop(optional, default = Some("hover:bg-indigo-500"))] button_hover_class: Option<
        &'static str,
    >,
    #[prop(optional, default = Some("bg-indigo-600"))] button_bg_color: Option<&'static str>,
    #[prop(optional, default = Some("bg-indigo-800"))] button_bg_color_down: Option<&'static str>,

    pending_text: &'static str,
    #[prop(into)] pending: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
) -> impl IntoView {
    let down = RwSignal::new(false);

    let button_class = Signal::derive(move || {
        if disabled.get() {
            format!(
                "{} disabled:cursor-not-allowed disabled:bg-indigo-50 disabled:text-indigo-600",
                button_class.unwrap(),
            )
        } else if down.get() && !pending.get() {
            format!(
                "{} {}",
                button_class.unwrap(),
                button_bg_color_down.unwrap()
            )
        } else {
            format!(
                "{} {} {}",
                button_class.unwrap(),
                button_bg_color.unwrap(),
                button_hover_class.unwrap()
            )
        }
    });

    view! {
        <button
            type="submit"
            disabled=move || pending.get() || disabled.get()
            class=button_class
            on:pointer_down=move |_: PointerEvent| down.set(true)
            on:pointer_up=move |_: PointerEvent| down.set(false)
            on:pointer_cancel=move |_: PointerEvent| down.set(false)
        >
            <Show when=move || pending.get() fallback=move || view! { {button_text} }>
                <LoaderIcon attr:class="inline w-4 h-4 text-white animate-spin me-3" />
                {pending_text}
            </Show>
        </button>
    }
}
