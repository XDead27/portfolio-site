use leptos::prelude::*;

#[component]
pub fn BioWindow() -> impl IntoView {
    view! {
        <div class="flex flex-col space-y-4 m-2">
            <h2 class="text-2xl font-bold">"About Me"</h2>
            <div class="flex flex-row justify-center space-x-4 items-center">
                <img src="/images/profile.png" alt="Profile Picture" class="w-32 h-32 rounded-full border-4 border-green"/>
                <p>
                    "Hi, I'm Daniel! I am a master’s student in Computer Science at TU Delft with experience in research, embedded systems, GPU programming, web development, and cybersecurity. I am particularily interested in system security and low-level programming."
                </p>
            </div>
        </div>
    }
}
