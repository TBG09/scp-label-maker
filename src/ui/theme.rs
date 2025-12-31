use iced::{Border, Color, Shadow};
use iced::widget::container;

pub const BACKGROUND: Color = Color::from_rgb(0.08, 0.08, 0.12);
pub const PANEL_BG: Color = Color::from_rgb(0.12, 0.13, 0.17);
pub const CARD_BG: Color = Color::from_rgb(0.14, 0.15, 0.19);

pub const TEXT_PRIMARY: Color = Color::from_rgb(0.95, 0.96, 0.98);
pub const TEXT_SECONDARY: Color = Color::from_rgb(0.65, 0.68, 0.75);

pub const ACCENT: Color = Color::from_rgb(0.25, 0.55, 0.95);
pub const ACCENT_HOVER: Color = Color::from_rgb(0.35, 0.65, 1.0);
pub const ACCENT_DARK: Color = Color::from_rgb(0.15, 0.45, 0.85);

pub const BORDER: Color = Color::from_rgb(0.2, 0.22, 0.28);
pub const BORDER_LIGHT: Color = Color::from_rgb(0.25, 0.28, 0.35);

pub const SUCCESS: Color = Color::from_rgb(0.2, 0.8, 0.4);
pub const WARNING: Color = Color::from_rgb(1.0, 0.65, 0.0);
pub const ERROR: Color = Color::from_rgb(0.95, 0.3, 0.3);

pub fn panel() -> container::Appearance {
    card()
}

pub fn card() -> container::Appearance {
    container::Appearance {
        background: Some(CARD_BG.into()),
        border: Border {
            color: BORDER,
            width: 1.0,
            radius: [8.0; 4].into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
            offset: iced::Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
        ..Default::default()
    }
}

pub fn inline_panel() -> container::Appearance {
    container::Appearance {
        background: Some(PANEL_BG.into()),
        border: Border {
            color: BORDER_LIGHT,
            width: 1.0,
            radius: [6.0; 4].into(),
        },
        ..Default::default()
    }
}

pub fn badge() -> container::Appearance {
    container::Appearance {
        background: Some(Color::from_rgba(
            ACCENT.r,
            ACCENT.g,
            ACCENT.b,
            0.15
        ).into()),
        border: Border {
            color: Color::from_rgba(ACCENT.r, ACCENT.g, ACCENT.b, 0.3),
            width: 1.0,
            radius: [12.0; 4].into(),
        },
        ..Default::default()
    }
}

pub fn preview_backdrop() -> container::Appearance {
    container::Appearance {
        background: Some(Color::from_rgb(0.05, 0.05, 0.08).into()),
        border: Border {
            color: BORDER,
            width: 2.0,
            radius: [8.0; 4].into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.5),
            offset: iced::Vector::new(0.0, 4.0),
            blur_radius: 12.0,
        },
        ..Default::default()
    }
}

pub fn slider_container() -> container::Appearance {
    container::Appearance {
        background: Some(Color::from_rgba(0.08, 0.09, 0.12, 0.6).into()),
        border: Border {
            color: BORDER_LIGHT,
            width: 1.0,
            radius: [6.0; 4].into(),
        },
        ..Default::default()
    }
}

pub fn input_container() -> container::Appearance {
    container::Appearance {
        background: Some(Color::from_rgb(0.10, 0.11, 0.14).into()),
        border: Border {
            color: Color::from_rgba(ACCENT.r, ACCENT.g, ACCENT.b, 0.3),
            width: 1.0,
            radius: [4.0; 4].into(),
        },
        ..Default::default()
    }
}

pub fn dropdown_container() -> container::Appearance {
    container::Appearance {
        background: Some(Color::from_rgb(0.10, 0.11, 0.14).into()),
        border: Border {
            color: BORDER_LIGHT,
            width: 1.0,
            radius: [6.0; 4].into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
            offset: iced::Vector::new(0.0, 1.0),
            blur_radius: 4.0,
        },
        ..Default::default()
    }
}