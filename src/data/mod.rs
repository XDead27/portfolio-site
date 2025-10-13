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

#[derive(Clone, Debug, PartialEq)]
pub enum WindowContent {
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

impl WindowContent {
    pub fn title(&self) -> &str {
        match self {
            WindowContent::Bio => "About Me",
            WindowContent::ThisSite => "This Site",
            WindowContent::Skills => "Skills",
            WindowContent::Education => "Education",
            WindowContent::SnakeProject => "Snake Game",
            WindowContent::ImaginaryProject => "GraphlexJS",
            WindowContent::HexChessProject => "Hexagonal Chess",
            WindowContent::TildeProject => "Tilde",
            WindowContent::DroneProject => "Drone",
            WindowContent::BachelorThesis => "Bachelor Thesis",
            WindowContent::TohokuPaper => "Tohoku Paper",
            WindowContent::MasterThesis => "Master Thesis",
        }
    }

    pub fn render(&self) -> AnyView {
        match self {
            WindowContent::Bio => view! { <BioWindow /> }.into_any(),
            WindowContent::ThisSite => view! { <ThisSiteWindow /> }.into_any(),
            WindowContent::Skills => view! { <SkillsWindow /> }.into_any(),
            WindowContent::Education => view! { <EducationWindow /> }.into_any(),
            WindowContent::SnakeProject => {
                view! { <crate::data::snake_project::SnakeProject /> }.into_any()
            }
            WindowContent::ImaginaryProject => {
                view! { <crate::data::imaginary_project::ImaginaryProject /> }.into_any()
            }
            WindowContent::HexChessProject => {
                view! { <crate::data::hexchess_project::HexChessProject /> }.into_any()
            }
            WindowContent::TildeProject => {
                view! { <crate::data::tilde_project::TildeProject /> }.into_any()
            }
            WindowContent::DroneProject => {
                view! { <crate::data::drone_project::DroneProject /> }.into_any()
            }
            WindowContent::BachelorThesis => {
                view! { <crate::data::bachelor_thesis::BachelorThesis /> }.into_any()
            }
            WindowContent::TohokuPaper => {
                view! { <crate::data::tohoku_paper::TohokuPaper /> }.into_any()
            }
            WindowContent::MasterThesis => {
                view! { <crate::data::master_thesis::MasterThesis /> }.into_any()
            }
        }
    }
}
