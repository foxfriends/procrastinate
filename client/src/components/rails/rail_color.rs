#![allow(dead_code)]

use yew::prelude::*;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub(crate) enum RailColor {
    Red,
    Cyan,
    Yellow,
    RedToCyan,
    CyanToRed,
    RedToYellow,
    YellowToRed,
    CyanToYellow,
    YellowToCyan,
}

impl RailColor {
    pub fn as_bg(&self) -> Classes {
        match self {
            RailColor::Red => classes!("bg-red"),
            RailColor::Cyan => classes!("bg-cyan"),
            RailColor::Yellow => classes!("bg-yellow"),
            RailColor::RedToCyan => classes!("bg-gradient-to-r", "from-red", "via-yellow", "to-cyan"),
            RailColor::CyanToRed => classes!("bg-gradient-to-r", "from-cyan", "via-yellow", "to-red"),
            RailColor::RedToYellow => classes!("bg-gradient-to-r", "from-red", "to-yellow"),
            RailColor::YellowToRed => classes!("bg-gradient-to-r", "from-yellow", "to-red"),
            RailColor::CyanToYellow => classes!("bg-gradient-to-r", "from-cyan", "to-yellow"),
            RailColor::YellowToCyan => classes!("bg-gradient-to-r", "from-yellow", "to-cyan"),
        }
    }

    pub fn as_border(&self) -> Classes {
        match self {
            RailColor::Red => classes!("border-red"),
            RailColor::Cyan => classes!("border-cyan"),
            RailColor::Yellow => classes!("border-yellow"),
            _ => unimplemented!("Border cannot be gradient (yet)"),
        }
    }

    pub fn as_stroke(&self) -> &'static str {
        match self {
            RailColor::Red => "#D44729",
            RailColor::Cyan => "#57C4B8",
            RailColor::Yellow => "#F5BD47",
            RailColor::RedToCyan => "url(#red-cyan)",
            RailColor::CyanToRed => "url(#cyan-red)",
            RailColor::RedToYellow => "url(#red-yellow)",
            RailColor::YellowToRed => "url(#yellow-red)",
            RailColor::CyanToYellow => "url(#cyan-yellow)",
            RailColor::YellowToCyan => "url(#yellow-cyan)",
        }
    }
}
