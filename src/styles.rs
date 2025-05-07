use crate::prelude::*;

pub(crate) mod prelude {
    pub(crate) use super::{AppColors, AppTableStyles};
}

pub struct AppColors {
    pub border_fg: Color,
    pub header_bg: Color,
    pub header_fg: Color,
    pub row_fg: Color,
    pub row_bg: Color,
    pub selected_row_fg: Color,
}

impl AppColors {
    pub const ACTIVE: Self = Self {
        border_fg: Color::Reset,
        header_bg: Color::Magenta,
        header_fg: Color::Black,
        row_fg: Color::Reset,
        row_bg: Color::Reset,
        selected_row_fg: Color::Red,
    };

    pub const INACTIVE: Self = Self {
        border_fg: Color::DarkGray,
        header_bg: Color::DarkGray,
        header_fg: Color::Black,
        row_fg: Color::DarkGray,
        row_bg: Color::Reset,
        selected_row_fg: Color::DarkGray,
    };
    pub fn get(active: bool) -> Self {
        match active {
            true => AppColors::ACTIVE,
            false => AppColors::INACTIVE,
        }
    }
}

pub struct AppTableStyles {
    pub header_style: Style,
    pub row_style: Style,
    pub highlight_style: Style,
}

impl From<AppColors> for AppTableStyles {
    fn from(value: AppColors) -> Self {
        Self {
            header_style: Style::default()
                .fg(value.header_fg)
                .bg(value.header_bg)
                .bold(),
            row_style: Style::default().fg(value.row_fg).bg(value.row_bg),
            highlight_style: Style::default().fg(value.selected_row_fg),
        }
    }
}
