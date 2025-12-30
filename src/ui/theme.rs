use iced::{Border, Color};
use iced::widget::container;

pub fn panel() -> container::Appearance {
    container::Appearance {
        background: Some(Color::from_rgb(0.15, 0.15, 0.15).into()),
        border: Border {
            color: Color::from_rgb(0.23, 0.23, 0.23),
            width: 1.0,
            ..Default::default()
        },
        ..Default::default()
    }
}

pub const BACKGROUND: Color = Color::from_rgb(0.1, 0.1, 0.1);
pub const PANEL_BG: Color = Color::from_rgb(0.15, 0.15, 0.15);
pub const TEXT_PRIMARY: Color = Color::from_rgb(0.88, 0.88, 0.88);
pub const TEXT_SECONDARY: Color = Color::from_rgb(0.62, 0.62, 0.62);
pub const ACCENT: Color = Color::from_rgb(0.18, 0.49, 0.2);
pub const BORDER: Color = Color::from_rgb(0.23, 0.23, 0.23);
pub const SUCCESS: Color = Color::from_rgb(0.0, 0.8, 0.0);
pub const WARNING: Color = Color::from_rgb(1.0, 0.6, 0.0);