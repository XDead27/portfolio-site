use leptos::prelude::*;
use once_cell::sync::Lazy;
use reactive_stores::Store;

use crate::app::{GlobalState, GlobalStateStoreFields, NUM_WORKSPACES};
use crate::components::modules::WindowContent;

static WINDOWS_BY_WORKSPACE: Lazy<[Vec<WindowContent>; NUM_WORKSPACES]> = Lazy::new(|| {
    [
        vec![
            WindowContent::Bio,
            WindowContent::ThisSite,
            WindowContent::Skills,
        ],
        vec![],
        vec![],
        vec![],
    ]
});

#[component]
pub fn Navbar(
    workspace_names: Vec<String>,
    on_add_window_workspace: impl Fn(usize, WindowContent) + 'static + Copy,
) -> impl IntoView {
    let workspace_base_css_class = "inline-block align-middle rounded-sm hover:bg-green-200 transition-colors duration-300 px-2 cursor-pointer";
    let current_workspace = expect_context::<Store<GlobalState>>().current_workspace();

    view! {
        <div class="flex flex-row space-x-2 justify-between px-4 py-2 mx-auto mb-12 rounded-sm bg-green text-white">
            {workspace_names.iter().enumerate().map({
                move |(idx, wn)| {
                    view! {
                        <div class="relative group">
                            <span
                                class={workspace_base_css_class}
                                class=("bg-green-100", move || current_workspace.get() == idx)
                                on:click=move |_| {
                                    current_workspace.set(idx);
                                }
                            >
                                {wn.clone()}
                            </span>
                            <div class="absolute bottom-full flex flex-col bg-green rounded-sm mx-auto px-4 py-2 hidden group-hover:flex hover:flex">
                                {WINDOWS_BY_WORKSPACE[idx].iter().map({
                                    move |w| {
                                        let w_name = w.title();
                                        view! {
                                            <span
                                                class="cursor-pointer hover:underline"
                                                on:click=move |_| on_add_window_workspace(idx, w.clone())
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
