use leptos::prelude::*;

#[component]
pub fn Window(name: String, on_close: impl Fn() + Clone + 'static) -> impl IntoView {
    let (is_closing, set_is_closing) = signal(false);

    view! {
        <div
            class=move || {
                let mut base = "w-full h-full rounded-sm border-3 border-beige flex flex-col transition-all duration-500 ease-in-out transform".to_string();
                if is_closing.get() {
                    base += " opacity-0 scale-90";
                } else {
                    base += " opacity-100 scale-100";
                }
                base
            }
        >
            <div class="w-full px-2 py-1 flex flex-row justify-between bg-beige text-wm-bg">
                <span class="text-sm font-semibold">
                    {name}
                </span>
                <div class="flex flex-row">
                    <img
                        src="/icons/cross.svg"
                        alt="Close"
                        class="w-4 h-4 cursor-pointer hover:opacity-80 transition-opacity duration-300"
                        on:click=move |_| {
                            set_is_closing.set(true);
                            let on_close = on_close.clone();

                            set_timeout(
                                move || on_close(),
                                std::time::Duration::from_millis(500),
                            );
                        }
                    />
                </div>
            </div>
            <div class="h-full bg-purple-900 mx-2 mb-2"></div>
        </div>
    }
}
