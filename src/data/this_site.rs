use leptos::prelude::*;

#[component]
pub fn ThisSiteWindow() -> impl IntoView {
    view! {
        <div class="flex flex-col space-y-4">
            <h2 class="text-2xl font-bold">"This Site"</h2>
            <p>
                "This site is built using Leptos, a Rust framework for building web applications. It showcases my projects and skills in a clean and modern design."
            </p>
        </div>
    }
}
