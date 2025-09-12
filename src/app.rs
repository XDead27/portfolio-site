use crate::components::Navbar;
use crate::components::Workspace;
use crate::components::workspace::WorkspaceData;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};
use reactive_stores::Store;
use uuid::Uuid;

#[derive(Clone, Debug, Store)]
pub struct GlobalState {
    current_workspace: usize,
    workspaces: Vec<WorkspaceData>,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            current_workspace: 0,
            all_windows: vec![
                WindowData::Bio {
                    id: Uuid::new_v4(),
                    workspace: 0,
                },
                WindowData::ThisSite {
                    id: Uuid::new_v4(),
                    workspace: 0,
                },
                WindowData::Skills {
                    id: Uuid::new_v4(),
                    workspace: 0,
                },
            ],
            focused_window: None,
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_context(Store::new(GlobalState::default()));

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
    let current_workspace = expect_context::<Store<GlobalState>>().current_workspace();
    let windows = expect_context::<Store<GlobalState>>().all_windows();

    let ws_windows = Memo::new(move |_| {
        windows
            .get()
            .iter()
            .filter(|&w| w.workspace() == current_workspace.get())
            .cloned()
            .collect::<Vec<_>>()
    });

    view! {
        <Title text="Home | Daniel Peter - Portofolio"/>
        <main>
            <div class="flex flex-col h-screen">
                <div class="p-4 text-wm-cyan">"Welcome to my portfolio!"</div>
                <Workspace>
                    {move || {
                        ws_windows
                            .get()
                            .into_iter()
                            .map(|w| w.render())
                            .collect_view()
                    }}
                </Workspace>
                <Navbar/>
            </div>
        </main>
    }
}
