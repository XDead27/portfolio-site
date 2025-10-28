use std::collections::HashSet;
use std::time::Duration;

use crate::components::Button;
use crate::components::Navbar;
use crate::components::Tooltip;
use crate::components::Workspace;
use crate::components::window::WindowData;
use crate::components::workspace::SplitDirection;
use crate::components::workspace::WorkspaceData;
use crate::components::workspace::WorkspaceNodeData;
use crate::data::WindowContentType;
use crate::data::defaults::{DEFAULT_WORKSPACES, NUM_WORKSPACES};
use crate::utils::tree::tree_any;
use leptos::logging;
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
    let blinking_windows = expect_context::<Store<GlobalState>>().blinking_windows();
    let current_direction = expect_context::<Store<GlobalState>>().current_direction();

    let on_new_window_workspace = move |workspace_idx: usize, window_content: WindowContentType| {
        if workspace_idx < NUM_WORKSPACES {
            let ws = workspaces[workspace_idx];
            ws.update(|ws| {
                ws.add_window(
                    current_direction.get(),
                    WindowData {
                        content: window_content.clone(),
                        blur_overlay: false,
                    },
                )
            });
        }
    };

    let on_add_window_workspace = move |workspace_idx: usize, window_content: WindowContentType| {
        if workspace_idx < NUM_WORKSPACES {
            let ws = workspaces[workspace_idx];
            let contains = {
                let ws = ws.get();
                let tree = ws.windows.read().expect("Failed to read workspace tree");
                tree_any(&tree.root().unwrap(), &|w: &WorkspaceNodeData| {
                    w.window_data
                        .clone()
                        .is_some_and(|wd| wd.content == window_content.clone())
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
                ws.update(|ws| {
                    ws.add_window(
                        current_direction.get(),
                        WindowData {
                            content: window_content.clone(),
                            blur_overlay: false,
                        },
                    )
                });
            }
        }
    };

    let workspace_names = workspaces
        .iter()
        .map(|ws| ws.get_untracked().name.clone())
        .collect::<Vec<String>>();

    // Sliding transition state
    let (transitioning, set_transitioning) = signal(false);
    let (prev_workspace_idx, set_prev_workspace_idx) =
        signal(current_workspace_idx.get_untracked());
    let animation_duration_ms: u64 = 250; // ms

    // Listen for workspace changes
    Effect::new(move |_| {
        let idx = current_workspace_idx.get();
        if idx != prev_workspace_idx.get() {
            set_transitioning.set(true);
            set_timeout(
                move || {
                    set_prev_workspace_idx.set(idx);
                    set_transitioning.set(false);
                },
                Duration::from_millis(animation_duration_ms),
            );
        }
    });

    // Determine which workspace to show
    let shown_workspace = move || {
        if transitioning.get() {
            workspaces[prev_workspace_idx.get()]
        } else {
            workspaces[current_workspace_idx.get()]
        }
    };

    // Animation direction
    let slide_direction = move || {
        let prev = prev_workspace_idx.get();
        let curr = current_workspace_idx.get();
        if prev < curr {
            "-translate-x-full"
        } else {
            "translate-x-full"
        }
    };

    // Tailwind sliding classes
    let slide_classes = move || {
        if transitioning.get() {
            slide_direction()
        } else {
            ""
        }
    };

    view! {
        <Title text="Home | Daniel Peter - Portofolio" />
        <main>
            <div class="flex flex-col h-screen overflow-hidden">
                <div class=move || {
                    format!(
                        "w-full h-full relative overflow-hidden transform transition-transform duration-{} ease-in-out {}",
                        animation_duration_ms.saturating_div(2),
                        slide_classes(),
                    )
                }>
                    <Workspace workspace_data=shown_workspace />
                </div>
                <div class="flex flex-row justify-center mb-8 space-x-4">
                    <Navbar
                        workspace_names=workspace_names
                        on_new_window_workspace=on_new_window_workspace
                        on_add_window_workspace=on_add_window_workspace
                    />
                    <Tooltip
                        text="Change split direction"
                        delay=200
                    >
                        <Button on_click=move || {
                            let new_direction = current_direction.get().inverted();
                            current_direction.set(new_direction);
                        }>
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
                    </Tooltip>
                </div>
            </div>
        </main>
    }
}
