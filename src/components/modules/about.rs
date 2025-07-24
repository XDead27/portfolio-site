use crate::components::workspace::Workspace;
use leptos::prelude::*;

#[component]
fn Bio() -> impl IntoView {
    view! {
        <div class="flex flex-col space-y-4">
            <h2 class="text-2xl font-bold">"About Me"</h2>
            <p>
                "I am a software developer with a passion for building web applications. I enjoy working with modern technologies and frameworks to create efficient and user-friendly experiences."
            </p>
        </div>
    }
}

#[component]
fn ThisSite() -> impl IntoView {
    view! {
        <div class="flex flex-col space-y-4">
            <h2 class="text-2xl font-bold">"This Site"</h2>
            <p>
                "This site is built using Leptos, a Rust framework for building web applications. It showcases my projects and skills in a clean and modern design."
            </p>
        </div>
    }
}

#[component]
fn Skills() -> impl IntoView {
    view! {
        <div class="flex flex-col space-y-4">
            <h2 class="text-2xl font-bold">"Skills"</h2>
            <ul class="list-disc pl-5">
                <li>"Rust"</li>
                <li>"JavaScript"</li>
                <li>"Web Development"</li>
                <li>"UI/UX Design"</li>
            </ul>
        </div>
    }
}

pub fn AboutWorkspace() -> impl IntoView {
    view! {
        <Workspace>
            "Abcd"
        </Workspace>
    }
}
