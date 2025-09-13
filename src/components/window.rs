use leptos::prelude::*;

use crate::components::modules::WindowContent;

#[component]
pub fn Window(
    content: WindowContent,
    focused: bool,
    on_is_focused: impl Fn(bool) + 'static + Copy,
    on_close: fn(),
) -> impl IntoView {
    let (is_closing, _set_is_closing) = signal(false);

    let name = content.title().to_string();

    view! {
        <div
            class=move || {
                let mut base = "w-full h-full rounded-sm border-3 flex flex-col transition-all duration-500 ease-in-out transform".to_string();
                if is_closing.get() {
                    base += " opacity-0 scale-90";
                } else {
                    base += " opacity-100 scale-100";
                }
                if focused {
                    base += " border-beige-800";
                } else {
                    base += " border-beige";
                }
                base
            }
            on:mouseover=move |_| on_is_focused(true)
            // on:mouseout=move |_| on_is_focused(false)
        >
            <div
                class=move || {
                    let mut base = "w-full px-2 py-1 flex flex-row justify-between text-wm-bg transition-all duration-500 ease-in-out".to_string();
                    if focused {
                        base += " bg-beige-800";
                    } else {
                        base += " bg-beige";
                    }
                    base
                }
            >
                <span class="text-sm font-semibold">
                    {name}
                </span>
                <div class="flex flex-row">
                    <img
                        src="/icons/cross.svg"
                        alt="Close"
                        class="w-4 h-4 cursor-pointer hover:opacity-80 transition-opacity duration-300"
                        on:click=move |_| on_close()
                    />
                </div>
            </div>
            <div class="h-full bg-purple-900 mx-2 mb-2">
                {content.render()}
            </div>
        </div>
    }
}
