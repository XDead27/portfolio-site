use once_cell::sync::Lazy;

use crate::data::WindowContent;

pub static NUM_WORKSPACES: usize = 4;
pub static DEFAULT_WORKSPACES: Lazy<[(&str, Vec<WindowContent>); NUM_WORKSPACES]> =
    Lazy::new(|| {
        [
            (
                "About",
                vec![
                    WindowContent::Bio,
                    WindowContent::Education,
                    WindowContent::ThisSite,
                    WindowContent::Skills,
                ],
            ),
            ("Projects", vec![]),
            ("Contact", vec![]),
            ("Freestyle", vec![]),
        ]
    });
