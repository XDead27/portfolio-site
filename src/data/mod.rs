use leptos::{
    prelude::{AnyView, IntoAny},
    view,
};

use crate::data::bio::BioWindow;
use crate::data::education::EducationWindow;
use crate::data::skills::SkillsWindow;
use crate::data::this_site::ThisSiteWindow;

pub mod bachelor_thesis;
pub mod bio;
pub mod defaults;
pub mod drone_project;
pub mod education;
pub mod hexchess_project;
pub mod imaginary_project;
pub mod master_thesis;
pub mod skills;
pub mod snake_project;
pub mod this_site;
pub mod tilde_project;
pub mod tohoku_paper;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum WindowContentType {
    Bio,
    ThisSite,
    Skills,
    Education,
    SnakeProject,
    ImaginaryProject,
    HexChessProject,
    TildeProject,
    DroneProject,
    BachelorThesis,
    TohokuPaper,
    MasterThesis,
}

impl WindowContentType {
    pub fn title(&self) -> &str {
        match self {
            WindowContentType::Bio => "About Me",
            WindowContentType::ThisSite => "This Site",
            WindowContentType::Skills => "Skills",
            WindowContentType::Education => "Education",
            WindowContentType::SnakeProject => "Snake Game",
            WindowContentType::ImaginaryProject => "GraphlexJS",
            WindowContentType::HexChessProject => "Hexagonal Chess",
            WindowContentType::TildeProject => "Tilde",
            WindowContentType::DroneProject => "Drone",
            WindowContentType::BachelorThesis => "Bachelor Thesis",
            WindowContentType::TohokuPaper => "Tohoku Paper",
            WindowContentType::MasterThesis => "Master Thesis",
        }
    }

    pub fn render(&self) -> AnyView {
        match self {
            WindowContentType::Bio => view! { <BioWindow /> }.into_any(),
            WindowContentType::ThisSite => view! { <ThisSiteWindow /> }.into_any(),
            WindowContentType::Skills => view! { <SkillsWindow /> }.into_any(),
            WindowContentType::Education => view! { <EducationWindow /> }.into_any(),
            WindowContentType::SnakeProject => {
                view! { <crate::data::snake_project::SnakeProject /> }.into_any()
            }
            WindowContentType::ImaginaryProject => {
                view! { <crate::data::imaginary_project::ImaginaryProject /> }.into_any()
            }
            WindowContentType::HexChessProject => {
                view! { <crate::data::hexchess_project::HexChessProject /> }.into_any()
            }
            WindowContentType::TildeProject => {
                view! { <crate::data::tilde_project::TildeProject /> }.into_any()
            }
            WindowContentType::DroneProject => {
                view! { <crate::data::drone_project::DroneProject /> }.into_any()
            }
            WindowContentType::BachelorThesis => {
                view! { <crate::data::bachelor_thesis::BachelorThesis /> }.into_any()
            }
            WindowContentType::TohokuPaper => {
                view! { <crate::data::tohoku_paper::TohokuPaper /> }.into_any()
            }
            WindowContentType::MasterThesis => {
                view! { <crate::data::master_thesis::MasterThesis /> }.into_any()
            }
        }
    }
}
