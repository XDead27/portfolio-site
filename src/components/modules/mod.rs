pub mod about;

// #[derive(Clone, Debug, PartialEq)]
// pub enum WindowData {
//     Bio { id: Uuid, workspace: usize },
//     ThisSite { id: Uuid, workspace: usize },
//     Skills { id: Uuid, workspace: usize },
// }
//
// impl WindowData {
//     /// the id of the window
//     pub fn id(&self) -> Uuid {
//         match self {
//             WindowData::Bio { id, .. } => *id,
//             WindowData::ThisSite { id, .. } => *id,
//             WindowData::Skills { id, .. } => *id,
//         }
//     }
//
//     /// which workspace this lives in
//     pub fn workspace(&self) -> usize {
//         match self {
//             WindowData::Bio { workspace, .. } => *workspace,
//             WindowData::ThisSite { workspace, .. } => *workspace,
//             WindowData::Skills { workspace, .. } => *workspace,
//         }
//     }
//
//     /// render the right component for this variant
//     pub fn render(&self) -> AnyView {
//         match self {
//             WindowData::Bio { id, .. } => view! {
//                 <BioWindow id={*id} />
//             }
//             .into_any(),
//             WindowData::ThisSite { id, .. } => view! {
//                 <ThisSiteWindow id={*id} />
//             }
//             .into_any(),
//             WindowData::Skills { id, .. } => view! {
//                 <SkillsWindow id={*id} />
//             }
//             .into_any(),
//         }
//     }
// }
