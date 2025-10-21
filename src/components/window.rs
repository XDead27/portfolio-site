use leptos::prelude::*;
use reactive_stores::Store;

use crate::{
    app::{GlobalState, GlobalStateStoreFields},
    data::WindowContent,
};

#[component]
pub fn Window(
    content: WindowContent,
    focused: bool,
    on_is_focused: impl Fn(bool) + 'static + Copy,
    on_close: impl Fn() + 'static + Copy,
) -> impl IntoView {
    let (is_closing, _set_is_closing) = signal(false);
    let blinking_windows = expect_context::<Store<GlobalState>>().blinking_windows();

    let name = content.title().to_string();
    let content2 = content.clone();

    view! {
        <div
            class=move || {
                let mut base = "w-full h-full rounded-sm border-3 flex flex-col transition-all duration-500 ease-in-out transform group"
                    .to_string();
                if is_closing.get() {
                    base += " opacity-0 scale-90";
                } else {
                    base += " opacity-100 scale-100";
                }
                if blinking_windows.get().contains(&content) {
                    base += " border-red animate-pulse";
                } else if focused {
                    base += " border-beige-800";
                } else {
                    base += " border-beige";
                }
                base
            }
            on:mouseover=move |_| on_is_focused(true)
        >
            // on:mouseout=move |_| on_is_focused(false)
            <div class=move || {
                let mut base = "w-full px-2 py-1 flex flex-row justify-between text-wm-bg transition-all duration-500 ease-in-out cursor-pointer"
                    .to_string();
                if blinking_windows.get().contains(&content2) {
                    base += " bg-red";
                } else if focused {
                    base += " bg-beige-800";
                } else {
                    base += " bg-beige";
                }
                base
            }>
                <span class="text-sm font-semibold">{name}</span>
                <div class="flex flex-row">
                    <img
                        src="/icons/cross.svg"
                        alt="Close"
                        class="w-4 h-4 cursor-pointer opacity-0 group-hover:opacity-100 hover:opacity-80 transition-opacity duration-300"
                        on:click=move |_| on_close()
                    />
                </div>
            </div>
            <div class="h-full bg-purple-900 mx-2 mb-2 overflow-auto">{content.render()}</div>
        </div>
    }
}
