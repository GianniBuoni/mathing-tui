use crate::prelude::*;

pub(crate) mod prelude {
    pub(crate) use super::{AppColors, AppStyles};
}

pub struct AppColors {
    pub border_fg: Color,
    pub header_bg: Color,
    pub header_fg: Color,
    pub row_fg: Color,
    pub selected_row_fg: Color,
    pub input_fg: Color,
}

impl AppColors {
    pub const ACTIVE: Self = Self {
        border_fg: Color::Reset,
        header_bg: Color::Magenta,
        header_fg: Color::Black,
        row_fg: Color::Reset,
        selected_row_fg: Color::Red,
        input_fg: Color::Magenta,
    };

    pub const INACTIVE: Self = Self {
        border_fg: Color::DarkGray,
        header_bg: Color::DarkGray,
        header_fg: Color::Black,
        row_fg: Color::DarkGray,
        selected_row_fg: Color::DarkGray,
        input_fg: Color::DarkGray,
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
}

impl From<AppColors> for AppStyles {
    fn from(value: AppColors) -> Self {
        Self {
            header_style: Style::default()
                .fg(value.header_fg)
                .bg(value.header_bg)
                .bold(),
            row_style: Style::default().fg(value.row_fg),
            highlight_style: Style::default().fg(value.selected_row_fg),
            input_style: Style::default().fg(value.input_fg),
            block_style: Style::default().fg(value.border_fg),
        }
    }
}
