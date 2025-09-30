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
pub mod skills;
pub mod this_site;

#[derive(Clone, Debug, PartialEq)]
pub enum WindowContent {
    Bio,
    ThisSite,
    Skills,
    Education,
}

impl WindowContent {
    pub fn title(&self) -> &str {
        match self {
            WindowContent::Bio => "About Me",
            WindowContent::ThisSite => "This Site",
            WindowContent::Skills => "Skills",
            WindowContent::Education => "Education",
        }
    }

    pub fn render(&self) -> AnyView {
        match self {
            WindowContent::Bio => view! {
                <BioWindow />
            }
            .into_any(),
            WindowContent::ThisSite => view! {
                <ThisSiteWindow />
            }
            .into_any(),
            WindowContent::Skills => view! {
                <SkillsWindow />
            }
            .into_any(),
            WindowContent::Education => view! {
                <EducationWindow />
            }
            .into_any(),
        }
    }
}
