use leptos::prelude::*;
use uuid::Uuid;

#[derive(Clone)]
pub struct WindowData {
    pub id: Uuid,
    pub name: String,
    pub focused: bool,
    pub content: AnyView,
}

#[component]
pub fn Window(data: WindowData, on_close: fn()) -> impl IntoView {
    let (is_closing, set_is_closing) = signal(false);

    let name = data.name;

    view! {
        <div
            class=move || {
                let mut base = "w-full h-full rounded-sm border-3 flex flex-col transition-all duration-500 ease-in-out transform".to_string();
                if is_closing.get() {
                    base += " opacity-0 scale-90";
                } else {
                    base += " opacity-100 scale-100";
                }
                if data.focused {
                    base += " border-beige-800";
                } else {
                    base += " border-beige";
                }
                base
            }
        >
            <div
                class=move || {
                    let mut base = "w-full px-2 py-1 flex flex-row justify-between text-wm-bg transition-all duration-500 ease-in-out".to_string();
                    if data.focused {
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
                {data.content}
            </div>
        </div>
    }
}
