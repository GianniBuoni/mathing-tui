use crate::prelude::*;

pub mod prelude {
    pub use super::{AppColors, AppStyles};
}

pub struct AppColors {
    pub base: Color,
    pub ground: Color,
    pub primary: Color,
    pub secondary: Color,
    pub warning: Color,
}

impl AppColors {
    pub const ACTIVE: Self = Self {
        base: Color::Reset,
        ground: Color::Magenta,
        primary: Color::Black,
        secondary: Color::Green,
        warning: Color::Red,
    };

    pub const INACTIVE: Self = Self {
        base: Color::DarkGray,
        ground: Color::DarkGray,
        primary: Color::Reset,
        secondary: Color::DarkGray,
        warning: Color::Red,
    };
    pub fn get(active: bool) -> Self {
        match active {
            true => AppColors::ACTIVE,
            false => AppColors::INACTIVE,
        }
    }
}

pub struct AppStyles {
    pub header_style: Style,
    pub row_style: Style,
    pub highlight_style: Style,
    pub input_style: Style,
    pub block_style: Style,
    pub error_style: Style,
}

impl From<AppColors> for AppStyles {
    fn from(value: AppColors) -> Self {
        Self {
            header_style: Style::default()
                .fg(value.primary)
                .bg(value.ground)
                .bold(),
            row_style: Style::default().fg(value.base),
            highlight_style: Style::default().fg(value.secondary).bold(),
            input_style: Style::default().fg(value.ground),
            block_style: Style::default().fg(value.base),
            error_style: Style::default().fg(value.warning),
        }
    }
}
