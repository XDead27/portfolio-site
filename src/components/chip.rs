use leptos::prelude::*;

#[component]
pub fn Chip(
    #[prop(optional, default = "".to_string())] name: String,
    #[prop(optional, default = "green".to_string())] color: String,
    #[prop(optional, default = None)] content: Option<AnyView>,
) -> impl IntoView {
    view! {
        <span class=format!(
            "inline-block bg-{} text-gray-900 text-sm font-medium mr-2 mb-2 px-3 py-1 rounded-sm",
            color,
        )>
            {name}
            {content
                .map(|content| view! { <span class="inline-block align-middle">{content}</span> })}
        </span>
    }
}
