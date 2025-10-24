use std::collections::HashSet;
use std::time::Duration;

use crate::components::window::WindowData;
use crate::components::workspace::WorkspaceNodeData;
use crate::components::Navbar;
use crate::components::Workspace;
use crate::components::workspace::SplitDirection;
use crate::components::workspace::WorkspaceData;
use crate::components::Button;
use crate::data::WindowContentType;
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
    blinking_windows: HashSet<WindowContentType>,
    current_direction: SplitDirection,
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
    let current_direction = expect_context::<Store<GlobalState>>().current_direction();

    let on_new_window_workspace = move |workspace_idx: usize, window_content: WindowContentType| {
        if workspace_idx < NUM_WORKSPACES {
            let ws = workspaces[workspace_idx];
            ws.update(|ws| ws.add_window(current_direction.get(), WindowData {
                content: window_content.clone(),
                blur_overlay: false,
            }));
        }
    };

    let on_add_window_workspace = move |workspace_idx: usize, window_content: WindowContentType| {
        if workspace_idx < NUM_WORKSPACES {
            let ws = workspaces[workspace_idx];
            let contains = {
                let ws = ws.get();
                let tree = ws.windows.read().expect("Failed to read workspace tree");
                tree_any(&tree.root().unwrap(), &|w: &WorkspaceNodeData| {
                    w.window_data.clone().is_some_and(|wd| wd.content == window_content.clone())
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
                ws.update(|ws| ws.add_window(current_direction.get(), WindowData {
                    content: window_content.clone(),
                    blur_overlay: false,
                }));
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
                <div class="flex flex-row justify-center mb-8 space-x-4">
                    <Navbar
                        workspace_names=workspace_names
                        on_new_window_workspace=on_new_window_workspace
                        on_add_window_workspace=on_add_window_workspace
                    />
                    <Button 
                        on_click=move || {
                            let new_direction = current_direction.get().inverted();
                            current_direction.set(new_direction);
                        }
                    >
                        <img
                            src=move || {
                                if current_direction.get() == SplitDirection::Vertical {
                                    "/icons/arrow-down.svg"
                                } else {
                                    "/icons/arrow-right.svg"
                                }
                            }
                            alt="Change split direction"
                            class="w-6 h-6 invert-[80%]"
                        />
                    </Button>
                </div>
            </div>
        </main>
    }
}
