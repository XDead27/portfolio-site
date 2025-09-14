use std::sync::{Arc, RwLock};

use leptos::{logging, prelude::*};
use nary_tree::{NodeId, RemoveBehavior, Tree, TreeBuilder};

use crate::components::Window;
use crate::components::modules::WindowContent;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NodeDirection {
    Vertical,
    #[default]
    Horizontal,
}

impl NodeDirection {
    pub fn inverted(&self) -> NodeDirection {
        match self {
            NodeDirection::Vertical => NodeDirection::Horizontal,
            NodeDirection::Horizontal => NodeDirection::Vertical,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WorkspaceNodeData {
    direction: NodeDirection,
    window_content: Option<WindowContent>,
}

#[derive(Debug, Clone)]
pub struct WorkspaceData {
    pub name: String,
    pub focused_window: Option<NodeId>,
    pub windows: Arc<RwLock<Tree<WorkspaceNodeData>>>,
}

impl WorkspaceData {
    pub fn new(name: String) -> Self {
        let tree = TreeBuilder::new()
            .with_root(WorkspaceNodeData {
                direction: NodeDirection::default(),
                window_content: None,
            })
            .build();
        Self {
            name,
            focused_window: None,
            windows: Arc::new(RwLock::new(tree)),
        }
    }

    pub fn add_window(&mut self, window_content: WindowContent) {
        let mut new_node = WorkspaceNodeData {
            direction: NodeDirection::default(),
            window_content: Some(window_content),
        };

        // select position to insert: if there is a focused window, split that node
        // otherwise, add to the first level under the root

        let tree = &mut self
            .windows
            .write()
            .expect("Failed to acquire write lock on window tree");

        if let Some(focused_id) = self.focused_window {
            let mut to_node = tree.get_mut(focused_id).unwrap();
            let curr_window = to_node
                .data()
                .clone()
                .window_content
                .expect("Focused node has no window data!");
            to_node.data().window_content = None;
            let curr_direction = to_node.data().direction;
            let curr_new_node = to_node.append(WorkspaceNodeData {
                direction: curr_direction.inverted(),
                window_content: Some(curr_window),
            });
            self.focused_window = Some(curr_new_node.node_id());

            new_node.direction = curr_direction.inverted();
            to_node.append(new_node);
        } else {
            let root = tree.root_id().expect("Window tree root does not exist!");
            tree.get_mut(root).unwrap().append(new_node);
        };
    }

    pub fn remove_window(&mut self, node_id: NodeId) {
        let tree = &mut self
            .windows
            .write()
            .expect("Failed to acquire write lock on window tree");

        let mut only_child_data = None;
        let node = tree
            .get_mut(node_id)
            .expect("Node to remove does not exist!");
        if let Some(parent_node) = node.parent() {
            tree.remove(node_id, RemoveBehavior::DropChildren);
            // if parent has only one child left, promote that child
            if parent_node.first_child().is_some() {
                let only_child_id = parent_node.first_child().unwrap().node_id();
                let only_child = tree.get(only_child_id).unwrap();
                only_child_data = Some(only_child.data().clone());
                tree.remove(only_child_id, RemoveBehavior::DropChildren);
            }
        }

        if self.focused_window == Some(node_id) {
            self.focused_window = None;
        }
    }
}

fn workspace_render_helper(node_id: NodeId, workspace_data: RwSignal<WorkspaceData>) -> AnyView {
    let base_div_style = "w-full h-full flex";

    let wd = workspace_data.get();
    let tree = wd
        .windows
        .read()
        .expect("Failed to acquire read lock on window tree");

    let node = tree.get(node_id).unwrap();
    let window_content = &node.data().window_content;
    let focused_id = wd.focused_window;

    if let Some(wd) = window_content {
        let class = "w-full h-full p-4";
        let focused = focused_id == Some(node_id);
        view! {
            <div class=class>
                <Window
                    content=wd.clone()
                    focused=focused
                    on_is_focused=move |b: bool| {
                        if b {
                            workspace_data.update(|ws| ws.focused_window = Some(node_id));
                        } else if focused_id == Some(node_id) {
                            workspace_data.update(|ws| ws.focused_window = None);
                        }
                    }
                    on_close=||{}
                />
            </div>
        }
        .into_any()
    } else {
        let flex_direction = match node.data().direction {
            NodeDirection::Vertical => "flex-col",
            NodeDirection::Horizontal => "flex-row",
        };
        let class = format!("{base_div_style} {flex_direction}");

        view! {
            <div class=class>
                {node.children().map(move |child_id| workspace_render_helper(child_id.node_id(), workspace_data)).collect_view()}
            </div>
        }.into_any()
    }
}

#[component]
pub fn Workspace(
    workspace_data: impl Fn() -> RwSignal<WorkspaceData> + Send + Sync + 'static,
) -> impl IntoView {
    view! {
        {move || workspace_render_helper(
            workspace_data().get()
                .windows
                .read()
                .expect("Failed to acquire read lock on window tree")
                .root_id()
                .expect("Window tree root does not exist!"),
            workspace_data()
        )}
    }
}
