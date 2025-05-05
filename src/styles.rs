use crate::prelude::*;

pub(crate) mod prelude {
    pub(crate) use super::AppColors;
}

pub struct AppColors {
    pub header_bg: Color,
    pub header_fg: Color,
    pub row_fg: Color,
    pub row_bg: Color,
    pub selected_row_fg: Color,
}

impl AppColors {
    pub fn active() -> Self {
        Self {
            header_bg: Color::Magenta,
            header_fg: Color::Black,
            row_fg: Color::Reset,
            row_bg: Color::Reset,
            selected_row_fg: Color::Red,
        }
    }
    pub fn inactive() -> Self {
        Self {
            header_bg: Color::DarkGray,
            header_fg: Color::Black,
            row_fg: Color::DarkGray,
            row_bg: Color::Reset,
            selected_row_fg: Color::DarkGray,
        }
    }
}
