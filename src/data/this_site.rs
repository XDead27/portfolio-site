use leptos::prelude::*;

#[component]
pub fn ThisSiteWindow() -> impl IntoView {
    view! {
        <div class="py-8 px-6">
            <div class="flex flex-col space-y-4 mx-auto">
                <h1 class="text-2xl font-bold mb-8 text-center text-gray-100">Where am I?</h1>
                <p>
                    "Hello, curious stranger! You happen to have stumbled over my website. Feel free to have a look around and check out what I've been doing with my life lately."
                </p>
                <p>
                    "This website works like a "
                    <a href="https://en.wikipedia.org/wiki/Tiling_window_manager">
                        "tiling window manager"
                    </a>
                    ". You can navigate to different \"workspaces\" using the navbar below. You can also open individual windows by clicking on a sub-topic."
                </p>
            </div>
        </div>
    }
}
