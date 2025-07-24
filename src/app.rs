use crate::components::Navbar;
use crate::components::Window;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Routes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=Home/>
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    let (workspace, set_workspace) = signal::<u32>(0);

    view! {
        <Title text="Home | Daniel Peter - Portofolio"/>
        <main>
            <Navbar current_workspace=workspace set_workspace=set_workspace/>
            <Window/>
            <div class="p-4 bg-wm-bg text-wm-cyan">"Welcome to my portfolio!"</div>
            <p>"This is a simple example of a Leptos application."</p>
        </main>
    }
}
