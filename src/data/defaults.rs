use std::sync::{Arc, RwLock};

use nary_tree::{Tree, TreeBuilder};
use once_cell::sync::Lazy;

use crate::{
    components::{window::WindowData, workspace::{SplitDirection, WorkspaceNodeData}},
    data::WindowContentType,
};

#[derive(Debug, Clone)]
pub struct WorkspaceDefaultsEntry {
    pub name: &'static str,
    pub contents: Vec<WindowContentType>,
    pub windows: Option<Arc<RwLock<Tree<WorkspaceNodeData>>>>,
}

pub static NUM_WORKSPACES: usize = 4;
pub static DEFAULT_WORKSPACES: Lazy<[WorkspaceDefaultsEntry; NUM_WORKSPACES]> = Lazy::new(|| {
    [
        WorkspaceDefaultsEntry {
            name: "About",
            contents: vec![
                WindowContentType::Bio,
                WindowContentType::Education,
                WindowContentType::ThisSite,
                WindowContentType::Skills,
            ],
            windows: Some(Arc::new(RwLock::new({
                let mut tree = TreeBuilder::new()
                    .with_root(WorkspaceNodeData::default())
                    .build();
                let mut root = tree.root_mut().unwrap();

                {
                    let mut lhs = root.append(WorkspaceNodeData::new(SplitDirection::Vertical));

                    lhs.append(WorkspaceNodeData {
                        direction: SplitDirection::default(),
                        window_data: Some(WindowData::from_content(WindowContentType::ThisSite)),
                    });
                    lhs.append(WorkspaceNodeData {
                        direction: SplitDirection::default(),
                        window_data: Some(WindowData::from_content(WindowContentType::Bio)),
                    });
                }

                root.append(WorkspaceNodeData {
                    direction: SplitDirection::Vertical,
                    window_data: Some(WindowData::from_content(WindowContentType::Education)),
                });

                root.append(WorkspaceNodeData {
                    direction: SplitDirection::Vertical,
                    window_data: Some(WindowData::from_content(WindowContentType::Skills)),
                });

                tree
            }))),
        },
        WorkspaceDefaultsEntry {
            name: "Projects",
            contents: vec![
                WindowContentType::SnakeProject,
                WindowContentType::ImaginaryProject,
                WindowContentType::HexChessProject,
                WindowContentType::TildeProject,
                WindowContentType::DroneProject,
            ],
            windows: Some(Arc::new(RwLock::new({
                let mut tree = TreeBuilder::new()
                    .with_root(WorkspaceNodeData::default())
                    .build();
                let mut root = tree.root_mut().unwrap();

                {
                    let mut lhs = root.append(WorkspaceNodeData::new(SplitDirection::Vertical));

                    lhs.append(WorkspaceNodeData {
                        direction: SplitDirection::default(),
                        window_data: Some(WindowData::new(WindowContentType::SnakeProject, true)),
                    });
                    lhs.append(WorkspaceNodeData {
                        direction: SplitDirection::default(),
                        window_data: Some(WindowData::new(WindowContentType::ImaginaryProject, true)),
                    });
                    lhs.append(WorkspaceNodeData {
                        direction: SplitDirection::default(),
                        window_data: Some(WindowData::new(WindowContentType::DroneProject, true)),
                    });
                }

                {
                    let mut rhs = root.append(WorkspaceNodeData::new(SplitDirection::Vertical));

                    rhs.append(WorkspaceNodeData {
                        direction: SplitDirection::default(),
                        window_data: Some(WindowData::new(WindowContentType::HexChessProject, true)),
                    });

                    rhs.append(WorkspaceNodeData {
                        direction: SplitDirection::default(),
                        window_data: Some(WindowData::new(WindowContentType::TildeProject, true)),
                    });
                }

                tree
            }))),
        },
        WorkspaceDefaultsEntry {
            name: "Research",
            contents: vec![
                WindowContentType::BachelorThesis,
                WindowContentType::TohokuPaper,
                WindowContentType::MasterThesis,
            ],
            windows: Some(Arc::new(RwLock::new({
                let mut tree = TreeBuilder::new()
                    .with_root(WorkspaceNodeData {
                        direction: SplitDirection::default(),
                        window_data: None,
                    })
                    .build();
                let mut root = tree.root_mut().unwrap();
                root.append(WorkspaceNodeData {
                    direction: SplitDirection::default(),
                    window_data: Some(WindowData::from_content(WindowContentType::BachelorThesis)),
                });
                root.append(WorkspaceNodeData {
                    direction: SplitDirection::default(),
                    window_data: Some(WindowData::from_content(WindowContentType::TohokuPaper)),
                });
                root.append(WorkspaceNodeData {
                    direction: SplitDirection::default(),
                    window_data: Some(WindowData::from_content(WindowContentType::MasterThesis)),
                });
                tree
            }))),
        },
        WorkspaceDefaultsEntry {
            name: "Freestyle",
            contents: vec![],
            windows: None,
        },
    ]
});
