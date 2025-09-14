use leptos::{
    prelude::{AnyView, IntoAny},
    view,
};

use crate::components::modules::about::{BioWindow, SkillsWindow, ThisSiteWindow};

pub mod about;

#[derive(Clone, Debug, PartialEq)]
pub enum WindowContent {
    Bio,
    ThisSite,
    Skills,
}

impl WindowContent {
    pub fn title(&self) -> &str {
        match self {
            WindowContent::Bio => "About Me",
            WindowContent::ThisSite => "This Site",
            WindowContent::Skills => "Skills",
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
        }
    }
}
