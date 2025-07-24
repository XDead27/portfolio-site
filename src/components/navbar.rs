use leptos::prelude::*;

#[component]
pub fn Navbar(
    current_workspace: ReadSignal<u32>,
    set_workspace: WriteSignal<u32>,
) -> impl IntoView {
    let workspaces = ["About", "Projects", "Contact"];
    let workspace_base_css_class = "inline-block align-middle rounded-sm hover:bg-green-200 transition-colors duration-300 px-2 cursor-pointer";

    view! {
        <div class="flex flex-row space-x-2 justify-between px-4 py-2 fixed bottom-16 left-1/2 -translate-x-1/2 rounded-sm bg-green text-white">
            {workspaces.iter().enumerate().map(|(idx, &workspace)| {
                view! {
                    <span
                        class={workspace_base_css_class}
                        class=("bg-green-100", move || current_workspace.get() == idx as u32)
                        on:click=move |_| {
                            set_workspace.set(idx as u32);
                        }
                    >
                        {workspace}
                    </span>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
