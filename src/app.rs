use crate::components::Navbar;
use crate::components::Window;
use crate::components::Workspace;
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
    let (windows, set_windows) = signal(Vec::<String>::new());

    set_windows.set(vec!["About".into(), "Projects".into()]);

    view! {
        <Title text="Home | Daniel Peter - Portofolio"/>
        <main>
            <div class="flex flex-col h-screen">
                <div class="p-4 text-wm-cyan">"Welcome to my portfolio!"</div>
                <Workspace>
                    {move || windows.get().iter().enumerate().map(|(idx, window_name)| {
                        view! {
                            <Window
                                name=window_name.clone()
                                on_close=move || {
                                    set_windows.update(|windows| {
                                        windows.remove(idx);
                                    });
                                }
                            />
                        }
                    }).collect::<Vec<_>>()}
                </Workspace>
                <Navbar current_workspace=workspace set_workspace=set_workspace/>
            </div>
        </main>
    }
}
