use leptos::prelude::*;
use once_cell::sync::Lazy;
use reactive_stores::Store;

use crate::app::{GlobalState, GlobalStateStoreFields};
use crate::data::WindowContentType;
use crate::data::defaults::{DEFAULT_WORKSPACES, NUM_WORKSPACES};
use crate::components::Button;

static WINDOWS_BY_WORKSPACE: Lazy<[Vec<WindowContentType>; NUM_WORKSPACES]> = Lazy::new(|| {
    DEFAULT_WORKSPACES
        .iter()
        .map(|wde| wde.contents.clone())
        .collect::<Vec<Vec<WindowContentType>>>()
        .try_into()
        .unwrap_or_else(|v: Vec<Vec<WindowContentType>>| {
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
    on_new_window_workspace: impl Fn(usize, WindowContentType) + 'static + Copy,
    on_add_window_workspace: impl Fn(usize, WindowContentType) + 'static + Copy,
) -> impl IntoView {
    let current_workspace = expect_context::<Store<GlobalState>>().current_workspace();

    view! {
        <div class="flex flex-row space-x-1 justify-between px-1 rounded-sm bg-green text-white">
            {workspace_names
                .into_iter()
                .enumerate()
                .map({
                    move |(idx, wn)| {
                        view! {
                            <div class="relative group">
                                <Button
                                    selected=Signal::derive(move || current_workspace.get() == idx)
                                    on_click=move || {
                                        current_workspace.set(idx);
                                    }
                                >
                                    {wn}
                                </Button>
                                {if WINDOWS_BY_WORKSPACE[idx].is_empty() {
                                    ().into_any()
                                } else {
                                    view! {
                                        <div class="absolute bottom-full flex flex-col bg-green rounded-sm mx-auto px-4 py-2 hidden group-hover:flex hover:flex min-w-max">
                                            {WINDOWS_BY_WORKSPACE[idx]
                                                .iter()
                                                .map({
                                                    move |w| {
                                                        let w_name = w.title();
                                                        view! {
                                                            <div class="flex flex-row space-x-2 group/inner items-center">
                                                                <img
                                                                    src="/icons/crosshair.svg"
                                                                    alt="Add"
                                                                    class="w-3 h-3 cursor-pointer opacity-0 group-hover/inner:opacity-100 transition-opacity duration-300"
                                                                    on:click=move |_| on_new_window_workspace(
                                                                        current_workspace.get(),
                                                                        w.clone(),
                                                                    )
                                                                />

                                                                <span
                                                                    class="cursor-pointer group-hover/inner:underline"
                                                                    on:click=move |_| on_add_window_workspace(
                                                                        current_workspace.get(),
                                                                        w.clone(),
                                                                    )
                                                                >
                                                                    {w_name}
                                                                </span>
                                                            </div>
                                                        }
                                                    }
                                                })
                                                .collect::<Vec<_>>()}
                                        </div>
                                    }
                                        .into_any()
                                }}
                            </div>
                        }
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
