#![allow(dead_code)]

use yew::prelude::*;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub(crate) struct RailColor {
    from: Color,
    via: Option<Color>,
    to: Color,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Color {
    Red,
    Cyan,
    Yellow,
}

impl RailColor {
    pub fn as_bg(&self) -> Classes {
        if self.from == self.to {
            match self.from {
                Color::Red => classes!("bg-red"),
                Color::Cyan => classes!("bg-cyan"),
                Color::Yellow => classes!("bg-yellow"),
            }
        } else {
            classes!(
                "bg-gradient-to-r",
                match self.from {
                    Color::Red => classes!("from-red"),
                    Color::Cyan => classes!("from-cyan"),
                    Color::Yellow => classes!("from-yellow"),
                },
                self.via.map(|color| match color {
                    Color::Red => classes!("via-red"),
                    Color::Cyan => classes!("via-cyan"),
                    Color::Yellow => classes!("via-yellow"),
                }),
                match self.to {
                    Color::Red => classes!("to-red"),
                    Color::Cyan => classes!("to-cyan"),
                    Color::Yellow => classes!("to-yellow"),
                },
            )
        }
    }

    pub fn as_border(&self) -> Classes {
        match self.from {
            Color::Red => classes!("border-red"),
            Color::Cyan => classes!("border-cyan"),
            Color::Yellow => classes!("border-yellow"),
        }
    }

    const fn solid(color: Color) -> Self {
        Self {
            from: color,
            via: None,
            to: color,
        }
    }

    pub const fn red() -> Self {
        Self::solid(Color::Red)
    }

    pub const fn yellow() -> Self {
        Self::solid(Color::Yellow)
    }

    pub const fn cyan() -> Self {
        Self::solid(Color::Cyan)
    }

    pub const fn cyan_to_red() -> Self {
        Self {
            from: Color::Cyan,
            via: Some(Color::Yellow),
            to: Color::Red,
        }
    }

    pub const fn red_to_cyan() -> Self {
        Self {
            from: Color::Red,
            via: Some(Color::Yellow),
            to: Color::Cyan,
        }
    }

    pub const fn red_to_yellow() -> Self {
        Self {
            from: Color::Red,
            via: None,
            to: Color::Yellow,
        }
    }

    pub const fn yellow_to_red() -> Self {
        Self {
            from: Color::Yellow,
            via: None,
            to: Color::Red,
        }
    }
}
