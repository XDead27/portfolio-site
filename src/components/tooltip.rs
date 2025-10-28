use std::time::Duration;

use leptos::prelude::*;

#[derive(Clone, Default)]
pub struct TooltipClassSlots {
    outer: Option<&'static str>,
    inner: Option<&'static str>,
    tooltip: Option<&'static str>,
}

#[component]
pub fn Tooltip(
    text: &'static str,
    #[prop(optional)] class: TooltipClassSlots,
    #[prop(optional)] delay: u64,
    children: Children,
) -> impl IntoView {
    let default_outer_class = "relative inline-block group";
    let default_inner_class = "";
    let default_tooltip_class = "absolute left-1/2 -translate-x-1/2 bottom-full mb-2 px-2 py-1 bg-gray-900 text-white rounded-sm shadow-lg opacity-0 group-hover:opacity-100 transition-opacity duration-300 z-10 whitespace-nowrap";

    let outer_class = class.outer.unwrap_or(default_outer_class);
    let inner_class = class.inner.unwrap_or(default_inner_class);
    let tooltip_class = class.tooltip.unwrap_or(default_tooltip_class);

    // Tooltip visibility signal
    let (visible, set_visible) = signal(false);
    let (timeout_handle, set_timeout_handle) = signal(None::<TimeoutHandle>);

    view! {
        <div
            class=outer_class
            on:mouseenter=move |_| {
                let delay = delay;
                // Clear any previous timeout
                if let Some(handle) = timeout_handle.get() {
                    handle.clear();
                }
                let handle = set_timeout_with_handle(
                    move || set_visible.set(true),
                    Duration::from_millis(delay),
                );
                if let Ok(handle) = handle {
                    set_timeout_handle.set(Some(handle));
                }
            }
            on:mouseleave=move |_| {
                set_visible.set(false);
                if let Some(handle) = timeout_handle.get() {
                    handle.clear();
                    set_timeout_handle.set(None);
                }
            }
        >
            <div class=inner_class>{children()}</div>
            <div
                class=tooltip_class
                style:display=move || if visible.get() { "block" } else { "none" }
                style=move || format!("opacity: {}; transition: opacity 0.3s;", if visible.get() { "1" } else { "0" })
            >
                <div style="position: absolute; left: 50%; top: 100%; transform: translateX(-50%);">
                    <div class="w-0 h-0 border-l-6 border-l-transparent border-r-6 border-r-transparent border-t-6 border-t-beige mt-[-1px]"></div>
                </div>
                {text}
            </div>
        </div>
    }
}
