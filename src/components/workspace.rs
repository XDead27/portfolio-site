use std::sync::{Arc, RwLock};

use leptos::prelude::*;
use nary_tree::{NodeId, Tree, TreeBuilder};

use crate::components::Window;
use crate::components::modules::WindowContent;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NodeDirection {
    Vertical,
    #[default]
    Horizontal,
}

#[derive(Debug, Clone)]
pub struct WorkspaceNodeData {
    direction: NodeDirection,
    window_content: Option<WindowContent>,
}

#[derive(Debug, Clone)]
pub struct WorkspaceData {
    pub name: String,
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
            windows: Arc::new(RwLock::new(tree)),
        }
    }

    pub fn add_window(&mut self, to_node_id: NodeId, window_content: WindowContent) {
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
            let mut to_node = tree.get_mut(to_node_id).unwrap();
            let curr_window = to_node
                .data()
                .clone()
                .window_content
                .expect("Focused node has no window data!");
            to_node.data().window_content = None;
            to_node.append(WorkspaceNodeData {
                direction: NodeDirection::default(),
                window_content: Some(curr_window),
            });

            new_node.direction = match to_node.data().direction {
                NodeDirection::Vertical => NodeDirection::Horizontal,
                NodeDirection::Horizontal => NodeDirection::Vertical,
            };
            to_node.append(new_node);
        } else {
            let root = tree.root_id().expect("Window tree root does not exist!");
            tree.get_mut(root).unwrap().append(new_node);
        };
    }
}

fn workspace_render_helper(
    node_id: NodeId,
    focused_id: RwSignal<Option<NodeId>>,
    tree: &Tree<WorkspaceNodeData>,
) -> AnyView {
    let base_div_style = "w-full h-full p-4 flex";
    let node = tree.get(node_id).unwrap();
    let window_content = &node.data().window_content;

    if let Some(wd) = window_content {
        let class = "w-full h-full p-4";
        let focused = focused_id.get() == Some(node_id);
        view! {
            <div class=class>
                <Window
                    content=wd.clone()
                    focused=focused
                    on_is_focused=move |b: bool| {
                        if b {
                            focused_id.set(Some(node_id));
                        } else if focused_id.get() == Some(node_id) {
                            focused_id.set(None);
                        }
                    }
                    on_close=||{}
                />
            </div>
        }
        .into_any()
    } else {
        let flex_direction = match node.data().direction {
            NodeDirection::Vertical => "flex-col space-y-6",
            NodeDirection::Horizontal => "flex-row space-x-6",
        };
        let class = format!("{} {}", base_div_style, flex_direction);

        view! {
            <div class=class>
                {node.children().map(move |child_id| workspace_render_helper(child_id.node_id(), focused_id, tree)).collect_view()}
            </div>
        }.into_any()
    }
}

#[component]
pub fn Workspace(
    workspace_data: impl Fn() -> WorkspaceData + Send + Sync + 'static,
) -> impl IntoView {
    let focused_signal = RwSignal::new(workspace_data().focused_window);

    view! {
        {move || workspace_render_helper(
            workspace_data()
                .windows
                .read()
                .expect("Failed to acquire read lock on window tree")
                .root_id()
                .expect("Window tree root does not exist!"),
            focused_signal,
            &workspace_data()
                .windows
                .read()
                .expect("Failed to acquire read lock on window tree"))}
    }
}
