use leptos::prelude::*;

#[component]
pub fn Landing() -> impl IntoView {
    let heading_text = "Bug ";
    let heading_highlighted_text = "Reproduction";
    let subheading_text = "Testing various bugs!";

    view! {
        <main class="relative px-6 pt-14 lg:px-8 isolate">
            <div class="mx-auto mt-16 max-w-7xl sm:px-6 lg:px-8 sm:mt-26">
                <div class="pt-16 pb-10 sm:px-6 sm:pb-6 lg:px-8 sm:pt-18">
                    <div class="mx-auto max-w-2xl text-center">
                        <h1 class="mt-6 text-4xl font-bold tracking-tight text-gray-900 sm:text-6xl">
                            {heading_text}
                            <span class="text-indigo-600">{heading_highlighted_text}</span>
                        </h1>
                    </div>
                    <h2 class="mx-auto mt-6 max-w-2xl text-lg leading-8 text-center text-gray-600">
                        {subheading_text}
                    </h2>
                </div>
            </div>
        </main>
    }
}
