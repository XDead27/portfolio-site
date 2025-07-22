use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="fixed bottom-16 inset-x-20 bg-red-50">
            I am the Navbar!
        </div>
    }
}
