use crate::components::Navbar;
use crate::components::Workspace;
use crate::components::modules::WindowContent;
use crate::components::workspace::WorkspaceData;
use leptos::logging;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};
use reactive_stores::Store;

pub static NUM_WORKSPACES: usize = 4;

#[derive(Clone, Debug, Store, Default)]
pub struct GlobalState {
    current_workspace: usize,
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
    let (workspaces, set_workspaces) = signal([
        WorkspaceData::new("About".to_string()),
        WorkspaceData::new("Projects".to_string()),
        WorkspaceData::new("Contact".to_string()),
        WorkspaceData::new("Freestyle".to_string()),
    ]);

    let current_workspace = expect_context::<Store<GlobalState>>().current_workspace();

    let on_add_window_workspace = move |workspace_idx: usize, window_content: WindowContent| {
        if workspace_idx < NUM_WORKSPACES {
            set_workspaces.update(|workspaces| {
                let mut ws = workspaces[workspace_idx].clone();
                ws.add_window(window_content.clone());
                workspaces[workspace_idx] = ws;
            });
        }
    };

    let workspace_names = workspaces
        .get_untracked()
        .iter()
        .map(|ws| ws.name.clone())
        .collect::<Vec<String>>();

    view! {
        <Title text="Home | Daniel Peter - Portofolio"/>
        <main>
            <div class="flex flex-col h-screen">
                <div class="p-4 text-wm-cyan">"Welcome to my portfolio!"</div>
                <Workspace workspace_data=move || workspaces.get()[current_workspace.get()].clone() />
                <Navbar workspace_names=workspace_names on_add_window_workspace=on_add_window_workspace/>
            </div>
        </main>
    }
}
