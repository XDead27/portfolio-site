use leptos::prelude::*;
use once_cell::sync::Lazy;
use reactive_stores::Store;

use crate::app::{GlobalState, GlobalStateStoreFields};
use crate::data::WindowContent;
use crate::data::defaults::{DEFAULT_WORKSPACES, NUM_WORKSPACES};

static WINDOWS_BY_WORKSPACE: Lazy<[Vec<WindowContent>; NUM_WORKSPACES]> = Lazy::new(|| {
    DEFAULT_WORKSPACES
        .iter()
        .map(|wde| wde.contents.clone())
        .collect::<Vec<Vec<WindowContent>>>()
        .try_into()
        .unwrap_or_else(|v: Vec<Vec<WindowContent>>| {
            panic!(
                "Expected a Vec of length {} but got {}",
                NUM_WORKSPACES,
                v.len()
            )
        })
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
                            {
                                if WINDOWS_BY_WORKSPACE[idx].is_empty() {
                                    ().into_any()
                                } else {
                                    view! {
                                        <div class="absolute bottom-full flex flex-col bg-green rounded-sm mx-auto px-4 py-2 hidden group-hover:flex hover:flex min-w-max">
                                            {WINDOWS_BY_WORKSPACE[idx].iter().map({
                                                move |w| {
                                                    let w_name = w.title();
                                                    view! {
                                                        <div class="flex flex-row space-x-2 group/inner items-center">
                                                            <img
                                                                src="/icons/crosshair.svg"
                                                                alt="Add"
                                                                class="w-3 h-3 cursor-pointer opacity-0 group-hover/inner:opacity-100 transition-opacity duration-300"
                                                            />

                                                            <span
                                                                class="cursor-pointer group-hover/inner:underline"
                                                               on:click=move |_| on_add_window_workspace(current_workspace.get(), w.clone())
                                                            >
                                                                {w_name}
                                                            </span>
                                                        </div>
                                                    }
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    }.into_any()
                                }
                            }
                        </div>
                    }
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
