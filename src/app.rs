use crate::components::Navbar;
use crate::components::Workspace;
use crate::components::workspace::WorkspaceData;
use crate::data::WindowContent;
use crate::data::defaults::{DEFAULT_WORKSPACES, NUM_WORKSPACES};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};
use reactive_stores::Store;

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
                <Route path=StaticSegment("") view=Home />
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    let workspaces = DEFAULT_WORKSPACES.clone().map(|wde| {
        if let Some(windows) = wde.windows {
            RwSignal::new(WorkspaceData {
                name: wde.name.to_string(),
                focused_window: None,
                windows: windows.clone(),
            })
        } else {
            RwSignal::new(WorkspaceData::new(wde.name.to_string()))
        }
    });

    let current_workspace_idx = expect_context::<Store<GlobalState>>().current_workspace();
    let current_workspace = move || workspaces[current_workspace_idx.get()];

    let on_add_window_workspace = move |workspace_idx: usize, window_content: WindowContent| {
        if workspace_idx < NUM_WORKSPACES {
            let ws = workspaces[workspace_idx];
            ws.update(|ws| ws.add_window(window_content.clone()));
        }
    };

    let workspace_names = workspaces
        .iter()
        .map(|ws| ws.get_untracked().name.clone())
        .collect::<Vec<String>>();

    view! {
        <Title text="Home | Daniel Peter - Portofolio" />
        <main>
            <div class="flex flex-col h-screen overflow-hidden">
                <Workspace workspace_data=current_workspace />
                <Navbar
                    workspace_names=workspace_names
                    on_add_window_workspace=on_add_window_workspace
                />
            </div>
        </main>
    }
}
