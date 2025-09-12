use leptos::prelude::*;
use nary_tree::{NodeId, Tree};

use crate::components::Window;
use crate::components::window::WindowData;

pub enum NodeDirection {
    Vertical,
    Horizontal,
}

pub struct WorkspaceNodeData {
    direction: NodeDirection,
    data: WindowData,
}

pub struct WorkspaceData {
    name: String,
    focused_window: Option<NodeId>,
    windows: Tree<WorkspaceNodeData>,
}

fn workspace_render_helper(
    node_id: NodeId,
    tree: &Tree<WorkspaceNodeData>,
) -> impl IntoView + use<> {
    let base_div_style = "w-full h-full p-8 flex space-6";
    let node = tree.get(node_id).unwrap();

    if node.children().next().is_none() {
        view! {
            <div class="w-full h-full p-8">
                <Window data=node.data().data on_close=||{}/>
            </div>
        }
    } else {
        let flex_direction = match node.data().direction {
            NodeDirection::Vertical => "flex-col",
            NodeDirection::Horizontal => "flex-row",
        };

        view! {
            <div class=move || format!("{} {}", base_div_style, flex_direction)>
                {node.children().map(move |child_id| workspace_render_helper(child_id.node_id(), tree)).collect::<Vec<_>>()}
            </div>
        }
    }
}

#[component]
pub fn Workspace(workspace_data: WorkspaceData) -> impl IntoView {
    view! {
        {workspace_render_helper(workspace_data.windows.root_id().expect("Window tree root does not exist!"), &workspace_data.windows)}
    }
}
