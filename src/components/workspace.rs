use std::sync::{Arc, RwLock};

use leptos::prelude::*;
use nary_tree::{NodeId, RemoveBehavior, Tree, TreeBuilder};

use crate::components::Window;
use crate::components::window::WindowData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SplitDirection {
    Vertical,
    #[default]
    Horizontal,
}

impl SplitDirection {
    pub fn inverted(&self) -> SplitDirection {
        match self {
            SplitDirection::Vertical => SplitDirection::Horizontal,
            SplitDirection::Horizontal => SplitDirection::Vertical,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct WorkspaceNodeData {
    pub direction: SplitDirection,
    pub window_data: Option<WindowData>,
}

impl WorkspaceNodeData {
    pub fn new(direction: SplitDirection) -> Self {
        Self {
            direction,
            window_data: None,
        }
    }
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
                direction: SplitDirection::default(),
                window_data: None,
            })
            .build();
        Self {
            name,
            focused_window: None,
            windows: Arc::new(RwLock::new(tree)),
        }
    }

    pub fn add_window(&mut self, split_direction: SplitDirection, window_data: WindowData) {
        let new_node = WorkspaceNodeData {
            direction: split_direction,
            window_data: Some(window_data),
        };

        // select position to insert: if there is a focused window, split that node
        // otherwise, add to the first level under the root

        let tree = &mut self
            .windows
            .write()
            .expect("Failed to acquire write lock on window tree");

        if let Some(focused_id) = self.focused_window
            && let Some(mut to_node) = tree.get_mut(focused_id)
        {
            if split_direction == to_node.data().direction {
                let mut to_node_parent = to_node.parent().unwrap(); // safe since focused node cannot
                // be root
                to_node_parent.append(new_node);
            } else {
                let curr_window = to_node
                    .data()
                    .clone()
                    .window_data
                    .expect("Focused node has no window data!");
                let curr_new_node = to_node.append(WorkspaceNodeData {
                    direction: split_direction,
                    window_data: Some(curr_window),
                });
                self.focused_window = Some(curr_new_node.node_id());

                to_node.append(new_node);
                to_node.data().window_data = None;
                to_node.data().direction = split_direction;
            }
        } else {
            let root = tree.root_id().expect("Window tree root does not exist!");
            tree.get_mut(root).unwrap().append(new_node);
        };
    }

    pub fn remove_window(&mut self, node_id: NodeId) {
        let (node_data, replace_parent) = {
            let tree = self
                .windows
                .read()
                .expect("Failed to acquire read lock on window tree");
            let node = tree.get(node_id).expect("Node to remove does not exist!");
            let only_child = node
                .parent()
                .expect("Node to remove has no parent!")
                .children()
                .count()
                == 2;
            let parent_is_root = node
                .parent()
                .expect("Node to remove has no parent!")
                .node_id()
                == tree.root_id().unwrap();
            let prev_sibling_data = node.prev_sibling().map(|sibling| sibling.data().clone());
            let next_sibling_data = node.next_sibling().map(|sibling| sibling.data().clone());
            (
                prev_sibling_data.or(next_sibling_data),
                only_child && !parent_is_root,
            )
        };

        let mut tree = self
            .windows
            .write()
            .expect("Failed to acquire write lock on window tree");

        if replace_parent {
            let mut node = tree
                .get_mut(node_id)
                .expect("Node to remove does not exist!");
            let mut parent = node.parent().expect("Node to remove has no parent!");

            let node_data = node_data.expect("Node to remove has no sibling!");
            parent.data().window_data = node_data.window_data;
            parent.data().direction = node_data.direction;
        }
        tree.remove(node_id, RemoveBehavior::DropChildren);
    }
}

fn workspace_render_helper(node_id: NodeId, workspace_data: RwSignal<WorkspaceData>) -> AnyView {
    let base_div_style = "w-full h-full flex overflow-hidden";

    let wd = workspace_data.get();
    let tree = wd
        .windows
        .read()
        .expect("Failed to acquire read lock on window tree");

    let node = tree.get(node_id).unwrap();
    let window_content = &node.data().window_data;
    let focused_id = wd.focused_window;

    if let Some(wd) = window_content {
        let class = "w-full h-full p-4 overflow-hidden";
        view! {
            <div class=class>
                <Window
                    content=wd.clone().content
                    blur_overlay=wd.clone().blur_overlay
                    focused=Signal::derive(move || focused_id == Some(node_id))
                    on_is_focused=move || {
                        workspace_data.update(|ws| ws.focused_window = Some(node_id));
                    }
                    on_close=move || {
                        workspace_data.update(|ws| ws.remove_window(node_id));
                    }
                />
            </div>
        }
        .into_any()
    } else {
        let flex_direction = match node.data().direction {
            SplitDirection::Vertical => "flex-col",
            SplitDirection::Horizontal => "flex-row",
        };
        let class = format!("{base_div_style} {flex_direction}");

        view! {
            <div class=class>
                {node
                    .children()
                    .map(move |child_id| workspace_render_helper(
                        child_id.node_id(),
                        workspace_data,
                    ))
                    .collect_view()}
            </div>
        }
        .into_any()
    }
}

#[component]
pub fn Workspace(
    workspace_data: impl Fn() -> RwSignal<WorkspaceData> + Send + Sync + 'static,
) -> impl IntoView {
    view! {
        {move || {
            let (root_id, is_empty) = {
                let data = workspace_data().get();
                let windows = data.windows.read().unwrap();
                let root_id = windows.root_id().expect("Window tree root does not exist!");
                let root = windows.get(root_id).unwrap();
                (root_id, root.children().next().is_none())
            };
            if is_empty {

                view! {
                    <div class="flex flex-col items-center justify-center h-full w-full text-gray-500">
                        <p class="text-lg italic">
                            "No windows open. Use the navbar to add windows."
                        </p>
                    </div>
                }
                    .into_any()
            } else {
                workspace_render_helper(root_id, workspace_data()).into_any()
            }
        }}
    }
}
