use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    let copyright = "© 2024 BugRepro, Inc. All rights reserved.";

    view! {
        <footer class="relative bg-white" aria-labelledby="footer-heading">
            <h2 id="footer-heading" class="sr-only">
                {"Footer"}
            </h2>
            <div class="px-6 pt-16 pb-8 mx-auto max-w-7xl sm:pt-24 lg:px-8 lg:pt-32">
                <div class="pt-8 mt-16 border-t sm:mt-20 lg:mt-24 border-gray-900/10">
                    <p class="text-xs leading-5 text-center text-gray-500">{copyright}</p>
                </div>
            </div>
        </footer>
    }
}
