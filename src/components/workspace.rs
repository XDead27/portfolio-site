use leptos::prelude::*;

#[component]
pub fn Workspace(children: Children) -> impl IntoView {
    view! {
        <div class="w-full h-full p-8 flex flex-row space-x-6">
            {children()}
        </div>
    }
}
