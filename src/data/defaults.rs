use std::sync::{Arc, RwLock};

use nary_tree::{Tree, TreeBuilder};
use once_cell::sync::Lazy;

use crate::{
    components::workspace::{NodeDirection, WorkspaceNodeData},
    data::WindowContent,
};

#[derive(Debug, Clone)]
pub struct WorkspaceDefaultsEntry {
    pub name: &'static str,
    pub contents: Vec<WindowContent>,
    pub windows: Option<Arc<RwLock<Tree<WorkspaceNodeData>>>>,
}

pub static NUM_WORKSPACES: usize = 4;
pub static DEFAULT_WORKSPACES: Lazy<[WorkspaceDefaultsEntry; NUM_WORKSPACES]> = Lazy::new(|| {
    [
        WorkspaceDefaultsEntry {
            name: "About",
            contents: vec![
                WindowContent::Bio,
                WindowContent::Education,
                WindowContent::ThisSite,
                WindowContent::Skills,
            ],
            windows: Some(Arc::new(RwLock::new({
                let mut tree = TreeBuilder::new()
                    .with_root(WorkspaceNodeData::default())
                    .build();
                let mut root = tree.root_mut().unwrap();

                {
                    let mut lhs = root.append(WorkspaceNodeData::new(NodeDirection::Vertical));

                    lhs.append(WorkspaceNodeData {
                        direction: NodeDirection::default(),
                        window_content: Some(WindowContent::ThisSite),
                    });
                    lhs.append(WorkspaceNodeData {
                        direction: NodeDirection::default(),
                        window_content: Some(WindowContent::Bio),
                    });
                }

                root.append(WorkspaceNodeData {
                    direction: NodeDirection::Vertical,
                    window_content: Some(WindowContent::Education),
                });

                root.append(WorkspaceNodeData {
                    direction: NodeDirection::Vertical,
                    window_content: Some(WindowContent::Skills),
                });

                tree
            }))),
        },
        WorkspaceDefaultsEntry {
            name: "Projects",
            contents: vec![],
            windows: Some(Arc::new(RwLock::new(
                TreeBuilder::new()
                    .with_root(WorkspaceNodeData {
                        direction: NodeDirection::default(),
                        window_content: None,
                    })
                    .build(),
            ))),
        },
        WorkspaceDefaultsEntry {
            name: "Research",
            contents: vec![],
            windows: Some(Arc::new(RwLock::new(
                TreeBuilder::new()
                    .with_root(WorkspaceNodeData {
                        direction: NodeDirection::default(),
                        window_content: None,
                    })
                    .build(),
            ))),
        },
        WorkspaceDefaultsEntry {
            name: "Freestyle",
            contents: vec![],
            windows: None,
        },
    ]
});
