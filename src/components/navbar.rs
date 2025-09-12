use std::rc::Rc;

use leptos::prelude::*;
use reactive_stores::Store;
use uuid::Uuid;

use crate::{
    app::{GlobalState, GlobalStateStoreFields},
    components::modules::WindowData,
};

#[derive(Clone, Debug)]
struct NavbarDefaultWindow {
    name: String,
    window_data_ctor: fn() -> WindowData,
}

#[derive(Clone, Debug)]
struct NavbarWorkspaceInfo {
    name: String,
    default_windows: Vec<NavbarDefaultWindow>,
}

#[component]
pub fn Navbar() -> impl IntoView {
    let workspaces = Rc::new(vec![
        NavbarWorkspaceInfo {
            name: "About".to_string(),
            default_windows: vec![
                NavbarDefaultWindow {
                    name: "Bio".to_string(),
                    window_data_ctor: move || WindowData::Bio {
                        id: Uuid::new_v4(),
                        workspace: 0,
                    },
                },
                NavbarDefaultWindow {
                    name: "This Site".to_string(),
                    window_data_ctor: move || WindowData::ThisSite {
                        id: Uuid::new_v4(),
                        workspace: 0,
                    },
                },
                NavbarDefaultWindow {
                    name: "Skills".to_string(),
                    window_data_ctor: move || WindowData::Skills {
                        id: Uuid::new_v4(),
                        workspace: 0,
                    },
                },
            ],
        },
        NavbarWorkspaceInfo {
            name: "Projects".to_string(),
            default_windows: vec![],
        },
        NavbarWorkspaceInfo {
            name: "Contact".to_string(),
            default_windows: vec![],
        },
    ]);
    let workspace_base_css_class = "inline-block align-middle rounded-sm hover:bg-green-200 transition-colors duration-300 px-2 cursor-pointer";
    let current_workspace = expect_context::<Store<GlobalState>>().current_workspace();
    let current_windows = expect_context::<Store<GlobalState>>().all_windows();

    view! {
        <div class="flex flex-row space-x-2 justify-between px-4 py-2 mx-auto mb-12 rounded-sm bg-green text-white">
            {workspaces.iter().enumerate().map({
                move |(idx, workspace)| {
                    // Clone the Rc into this scope too
                    let workspace_name = workspace.name.clone();
                    let default_windows = workspace.default_windows.clone();

                    view! {
                        <div class="relative group">
                            <span
                                class={workspace_base_css_class}
                                class=("bg-green-100", move || current_workspace.get() == idx)
                                on:click=move |_| {
                                    current_workspace.set(idx);
                                }
                            >
                                {workspace_name}
                            </span>
                            <div class="absolute bottom-full flex flex-col bg-green rounded-sm mx-auto px-4 py-2 hidden group-hover:flex hover:flex">
                                {default_windows.iter().map({
                                    move |w| {
                                        let w_name = w.name.clone();
                                        let ctor = w.window_data_ctor;
                                        view! {
                                            <span
                                                class="cursor-pointer"
                                                on:click=move |_| current_windows.update(move |ws| ws.push((ctor)()))
                                            >
                                                {w_name}
                                            </span>
                                        }
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
                    }
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
