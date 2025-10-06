use leptos::{
    prelude::{AnyView, IntoAny},
    view,
};

use crate::data::bio::BioWindow;
use crate::data::education::EducationWindow;
use crate::data::skills::SkillsWindow;
use crate::data::this_site::ThisSiteWindow;

pub mod bio;
pub mod defaults;
pub mod education;
pub mod hexchess_project;
pub mod imaginary_project;
pub mod skills;
pub mod snake_project;
pub mod this_site;

#[derive(Clone, Debug, PartialEq)]
pub enum WindowContent {
    Bio,
    ThisSite,
    Skills,
    Education,
    SnakeProject,
    ImaginaryProject,
    HexChessProject,
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
        }
    }

    pub fn render(&self) -> AnyView {
        match self {
            WindowContent::Bio => view! { <BioWindow /> }
            .into_any(),
            WindowContent::ThisSite => view! { <ThisSiteWindow /> }
            .into_any(),
            WindowContent::Skills => view! { <SkillsWindow /> }
            .into_any(),
            WindowContent::Education => view! { <EducationWindow /> }
            .into_any(),
            WindowContent::SnakeProject => view! { <crate::data::snake_project::SnakeProject /> }
            .into_any(),
            WindowContent::ImaginaryProject => view! { <crate::data::imaginary_project::ImaginaryProject /> }
            .into_any(),
            WindowContent::HexChessProject => view! { <crate::data::hexchess_project::HexChessProject /> }
            .into_any(),
        }
    }
}
