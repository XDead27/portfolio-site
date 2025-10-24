use leptos::prelude::*;

#[derive(Clone, Default)]
pub struct ButtonClassSlots {
    outer: Option<&'static str>,
    inner: Option<&'static str>,
}
    

#[component]
pub fn Button(
    on_click: impl Fn() + 'static + Copy,
    #[prop(optional)] 
    class: ButtonClassSlots,
    #[prop(optional, into)]
    selected: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let default_outer_class = "p-1 bg-green text-white rounded-sm group cursor-pointer transition-colors duration-300";
    let default_inner_class = "bg-green-200 group-hover:bg-green-100 rounded-sm p-1";

    let outer_class = class.outer.unwrap_or(default_outer_class);
    let inner_class = class.inner.unwrap_or(default_inner_class);

    view! {
        <button
            class=outer_class
            on:click=move |_| {
                on_click();
            }
        >
            <div 
                class=inner_class
                class=(["bg-green-100", "border"], selected)
            >
                {children()}
            </div>
        </button>
    }
}
