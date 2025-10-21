use std::collections::HashSet;
use std::time::Duration;

use crate::components::Navbar;
use crate::components::Workspace;
use crate::components::workspace::WorkspaceData;
use crate::components::workspace::WorkspaceNodeData;
use crate::data::WindowContent;
use crate::data::defaults::{DEFAULT_WORKSPACES, NUM_WORKSPACES};
use crate::utils::tree::tree_any;
use leptos::prelude::*;
use leptos::reactive::spawn_local;
use leptos_meta::*;
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};
use reactive_stores::Store;

#[derive(Clone, Debug, Store, Default)]
pub struct GlobalState {
    current_workspace: usize,
    blinking_windows: HashSet<WindowContent>,
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
    let blinking_windows = expect_context::<Store<GlobalState>>().blinking_windows();

    let on_new_window_workspace = move |workspace_idx: usize, window_content: WindowContent| {
        if workspace_idx < NUM_WORKSPACES {
            let ws = workspaces[workspace_idx];
            ws.update(|ws| ws.add_window(window_content.clone()));
        }
    };

    let on_add_window_workspace = move |workspace_idx: usize, window_content: WindowContent| {
        if workspace_idx < NUM_WORKSPACES {
            let ws = workspaces[workspace_idx];
            let contains = {
                let ws = ws.get();
                let tree = ws.windows.read().expect("Failed to read workspace tree");
                tree_any(&tree.root().unwrap(), &|w: &WorkspaceNodeData| {
                    w.window_content == Some(window_content.clone())
                })
            };

            if contains {
                // If the window is already present, make it blink
                blinking_windows.update(|bw| {
                    bw.insert(window_content.clone());
                });

                // Stop blinking after 2 seconds
                spawn_local({
                    async move {
                        set_timeout(
                            move || {
                                blinking_windows.update(|bw| {
                                    bw.remove(&window_content);
                                });
                            },
                            Duration::from_secs(2),
                        );
                    }
                });
            } else {
                ws.update(|ws| ws.add_window(window_content.clone()));
            }
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
                    on_new_window_workspace=on_new_window_workspace
                    on_add_window_workspace=on_add_window_workspace
                />
            </div>
        </main>
    }
}
