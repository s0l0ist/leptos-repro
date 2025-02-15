use leptos::prelude::*;

#[component]
pub fn Page500() -> impl IntoView {
    let error_desc = "Internal server error";
    let sorry = "Sorry, we experienced an unknown error.";
    let go_back = "Go back";

    view! {
        <div class="grid place-items-center py-24 px-6 min-h-full bg-white sm:py-32 lg:px-8">
            <div class="text-center">
                <p class="text-base font-semibold text-indigo-600">500</p>
                <h1 class="mt-4 text-3xl font-bold tracking-tight text-gray-900 sm:text-5xl">
                    {error_desc}
                </h1>
                <p class="mt-6 text-base leading-7 text-gray-600">{sorry}</p>
                <div class="flex gap-x-6 justify-center items-center mt-10">
                    <button
                        class="py-2.5 px-3.5 text-sm font-semibold text-white bg-indigo-600 rounded-md shadow-xs hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                        on:click=move |_| {
                            window().history().unwrap().back().unwrap();
                        }
                    >

                        {go_back}
                    </button>
                </div>
            </div>
        </div>
    }
}
