use crate::app::Message;
use crate::models::{ClassType, Hazard, ImageValidation, LabelConfig, ResizeMethod, ValidationStatus};
use iced::widget::{button, checkbox, column, container, pick_list, row, slider, text, text_input};
use iced::{Element, Length, Color};
use crate::ui::theme;

fn parse_hex_color(hex: &str) -> Result<Color, ()> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return Err(());
    }
    
    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| ())?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| ())?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| ())?;
    
    Ok(Color::from_rgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0))
}

pub fn view(config: &LabelConfig, validation: &Option<ImageValidation>) -> Element<'static, Message> {
    let title: iced::widget::Text<iced::Theme> = text("SCP Number Label Maker").size(24);
    let scp_text_color = config.scp_text_color;
    let class_text_color = config.class_text_color;
    let scp_input: iced::widget::Column<'_, Message, iced::Theme> = column![
        text("SCP Number:").size(14),
        row![
            text("SCP-").size(24),
            text_input("001", &config.scp_number)
                .on_input(Message::ScpNumberChanged)
                .on_submit(Message::ScpNumberSubmitted(config.scp_number.clone()))
                .padding(10)
        ]
        .spacing(5),
    ]
    .spacing(5);

    let class_input = column![
        text("Object Class Text:").size(14),
        text_input("SAFE", &config.object_class_text)
            .on_input(Message::ObjectClassChanged)
            .on_submit(Message::ObjectClassSubmitted(config.object_class_text.clone()))
            .padding(10)
    ]
    .spacing(5);

    let class_picker: iced::widget::Column<'_, Message, iced::Theme> = column![
        text("Visual Style:").size(14),
        pick_list(
            ClassType::all(),
            Some(config.class_type),
            Message::ClassTypeSelected,
        )
        .padding(10),
    ]
    .spacing(5);

    let alternate_toggle = checkbox(
        "Use alternate style",
        config.use_alternate_style
    )
    .on_toggle(Message::AlternateStyleToggled);

    let scp_number_font_size_slider: iced::widget::Column<'_, Message, iced::Theme> = column![
        text("SCP Number Font Size:").size(14),
        row![
            slider(24.0..=72.0, config.scp_number_font_size, |v| Message::ScpNumberFontSizeChanged(v)).step(1.0),
            text_input("60.0", &config.scp_number_font_size.to_string())
                .on_input(Message::ScpNumberFontSizeTextChanged)
                .on_submit(Message::ScpNumberFontSizeSubmitted(config.scp_number_font_size.to_string()))
                .padding(5)
                .width(60),
        ]
        .spacing(5),
    ]
    .spacing(5);

    let object_class_font_size_slider = column![
        text("Object Class Font Size:").size(14),
        row![
            slider(24.0..=72.0, config.object_class_font_size, |v| Message::ObjectClassFontSizeChanged(v)).step(1.0),
            text_input("60.0", &config.object_class_font_size.to_string())
                .on_input(Message::ObjectClassFontSizeTextChanged)
                .on_submit(Message::ObjectClassFontSizeSubmitted(config.object_class_font_size.to_string()))
                .padding(5)
                .width(60),
        ]
        .spacing(5),
    ]
    .spacing(5);
    
    let validation_display: iced::widget::Column<'_, Message, iced::Theme> = if let Some(val) = validation {
        let (_icon, color) = match val.status {
            ValidationStatus::PerfectFit => ("", iced::Color::from_rgb(0.0, 0.8, 0.0)),
            ValidationStatus::WillCrop => ("", iced::Color::from_rgb(1.0, 0.6, 0.0)),
            ValidationStatus::WillStretch => ("", iced::Color::from_rgb(0.9, 0.0, 0.0)),
            ValidationStatus::NoImage => ("", iced::Color::WHITE),
        };
        
        column![
            text(format!("{}", val.message))
                .size(12)
                .style(iced::theme::Text::Color(color)),
        ]
    } else {
        column![]
    };

    let resize_method = column![
        text("Resize Method:").size(14),
        pick_list(
            vec![ResizeMethod::CropToFit, ResizeMethod::Stretch, ResizeMethod::Letterbox],
            Some(config.resize_method),
            Message::ResizeMethodChanged
        )
        .padding(10),
    ]
    .spacing(5);

    let hazard_picker: iced::widget::Row<'_, Message, iced::Theme> = row![
        pick_list(
            Hazard::all(),
            config.selected_hazard,
            Message::HazardSelected
        )
        .padding(10),
        button("Clear").on_press(Message::ClearHazard).padding(10),
    ]
    .spacing(5);

    let main_settings = container(column![
        scp_input,
        class_input,
    ].spacing(10)).style(theme::panel());

    let style_section = container(column![
        class_picker,
        alternate_toggle,
    ].spacing(10)).style(theme::panel());
    
    let text_settings_section = container(column![
        column![
            text("Note: Use \\n to create new lines in text fields.")
                .size(12)
                .style(iced::Color::from_rgb(0.7, 0.7, 0.2)),
            iced::widget::horizontal_rule(1),
        ].spacing(5),
        text("Text Customization:").size(14),
        scp_number_font_size_slider,
        
        column![
            text(format!("SCP Number Line Spacing: {:.2}x", config.scp_line_spacing)).size(12),
            row![
                slider(0.5..=3.0, config.scp_line_spacing, Message::ScpLineSpacingChanged).step(0.05),
                text_input("1.2", &format!("{:.2}", config.scp_line_spacing))
                    .on_input(|s| Message::ScpLineSpacingTextChanged(s))
                    .padding(5)
                    .width(60),
            ].spacing(5)
        ].spacing(5),

        object_class_font_size_slider,

        column![
            text(format!("Object Class Line Spacing: {:.2}x", config.class_line_spacing)).size(12),
            row![
                slider(0.5..=3.0, config.class_line_spacing, Message::ClassLineSpacingChanged).step(0.05),
                text_input("1.2", &format!("{:.2}", config.class_line_spacing))
                    .on_input(|s| Message::ClassLineSpacingTextChanged(s))
                    .padding(5)
                    .width(60),
            ].spacing(5)
        ].spacing(5),
        row![
            text("SCP-Number Offset (X, Y):").size(12),
            text_input("2.0", &format!("{:.4}", config.scp_text_offset.0))
                .on_input(|s| Message::ScpTextOffsetXChanged(s))
                .on_submit(Message::ScpTextOffsetXSubmitted(config.scp_text_offset.0.to_string()))
                .padding(5)
                .width(60),
            text_input("-7.0", &format!("{:.4}", config.scp_text_offset.1))
                .on_input(|s| Message::ScpTextOffsetYChanged(s))
                .on_submit(Message::ScpTextOffsetYSubmitted(config.scp_text_offset.1.to_string()))
                .padding(5)
                .width(60),
        ].spacing(5),
        row![
            text("Object Class Offset (X, Y):").size(12),
            text_input("2.0", &format!("{:.4}", config.class_text_offset.0))
                .on_input(|s| Message::ClassTextOffsetXChanged(s))
                .on_submit(Message::ClassTextOffsetXSubmitted(config.class_text_offset.0.to_string()))
                .padding(5)
                .width(60),
            text_input("-7.0", &format!("{:.4}", config.class_text_offset.1))
                .on_input(|s| Message::ClassTextOffsetYChanged(s))
                .on_submit(Message::ClassTextOffsetYSubmitted(config.class_text_offset.1.to_string()))
                .padding(5)
                .width(60),
        ].spacing(5),
        row![
            text("SCP-Number Color (Hex):").size(12),
            text_input("#000000", &format!("#{:02x}{:02x}{:02x}", (Color::from(config.scp_text_color).r * 255.0) as u8, (Color::from(config.scp_text_color).g * 255.0) as u8, (Color::from(config.scp_text_color).b * 255.0) as u8))
                .on_input(move |s| {
                    if let Ok(color) = parse_hex_color(&s) {
                        Message::ScpTextColorChanged(color)
                    } else {
                        Message::ScpTextColorChanged(scp_text_color.into())
                    }
                })
                .on_submit(Message::ScpTextColorSubmitted(config.scp_text_color.into()))
                .padding(5)
                .width(100),
        ].spacing(5),
        row![
            text("Object Class Color (Hex):").size(12),
            text_input("#000000", &format!("#{:02x}{:02x}{:02x}", (Color::from(config.class_text_color).r * 255.0) as u8, (Color::from(config.class_text_color).g * 255.0) as u8, (Color::from(config.class_text_color).b * 255.0) as u8))
                .on_input(move |s| {
                    if let Ok(color) = parse_hex_color(&s) {
                        Message::ClassTextColorChanged(color)
                    } else {
                        Message::ClassTextColorChanged(class_text_color.into())
                    }
                })
                .on_submit(Message::ClassTextColorSubmitted(config.class_text_color.into()))
                .padding(5)
                .width(100),
        ].spacing(5),
        button("Reset Text").on_press(Message::ResetText).padding(10),
    ].spacing(10)).style(theme::panel());

    let image_section = container(column![
        text("User Image:").size(14),
        button("Select Image")
            .on_press(Message::SelectImagePressed)
            .padding(10),
        if let Some(path) = &config.image_path {
            Into::<Element<'static, Message>>::into(text(format!("File: {}", path.file_name().unwrap().to_string_lossy())).size(12))
        } else {
            Into::<Element<'static, Message>>::into(text("No image selected"))
        },
    ]
    .spacing(10)).style(theme::panel());

    let image_options_section = container(column![
        validation_display,
        resize_method,
    ].spacing(10)).style(theme::panel());

    let image_adjustment_section: iced::widget::Container<'_, Message, iced::Theme> = if !config.use_alternate_style {
        container(column![
            text("Image Adjustments:").size(14),
            text(format!("Brightness: {:.2}", config.brightness)).size(12),
            row![
                slider(-1.0..=1.0, config.brightness, |v| Message::BrightnessChanged(v)).step(0.05),
                text_input("0.0", &format!("{:.4}", config.brightness))
                    .on_input(|s| Message::BrightnessTextChanged(s))
                    .on_submit(Message::BrightnessSubmitted(config.brightness.to_string()))
                    .padding(5)
                    .width(60),
            ]
            .spacing(5),
            text(format!("Contrast: {:.2}", config.contrast)).size(12),
            row![
                slider(0.0..=2.0, config.contrast, |v| Message::ContrastChanged(v)).step(0.05),
                text_input("1.0", &format!("{:.4}", config.contrast))
                    .on_input(|s| Message::ContrastTextChanged(s))
                    .on_submit(Message::ContrastSubmitted(config.contrast.to_string()))
                    .padding(5)
                    .width(60),
            ]
            .spacing(5),
            checkbox("Grayscale", config.grayscale).on_toggle(Message::GrayscaleToggled),
        ]
        .spacing(5)).style(theme::panel())
    } else {
        container(column![])
    };

    let hazard_section = container(column![
        text("Hazard Warning:").size(14),
        hazard_picker,
    ].spacing(5)).style(theme::panel());

    let texture_section = container(column![
        checkbox("Apply texture overlay", config.apply_texture)
            .on_toggle(Message::TextureToggled),
        if config.apply_texture {
            Into::<Element<'static, Message>>::into(column![
                text(format!("Opacity: {:.0}%", config.texture_opacity * 100.0)).size(12),
                row![
                    slider(0.0..=1.0, config.texture_opacity, |v| Message::OpacityTextChanged(v.to_string())).step(0.05),
                    text_input("0.3", &format!("{:.4}", config.texture_opacity))
                        .on_input(|s| Message::OpacityTextChanged(s))
                        .on_submit(Message::OpacitySubmitted(config.texture_opacity.to_string()))
                        .padding(5)
                        .width(60),
                ]
                .spacing(5),
            ]
            .spacing(5))
        } else {
            Into::<Element<'static, Message>>::into(column![])
        }
    ]
    .spacing(5)).style(theme::panel());

    let export_section: iced::widget::Container<'_, Message, iced::Theme> = container(column![
        text("Export & Project Management: ").size(14),
        row![
            text("Resolution: ").size(12),
            button("512").on_press(Message::ResolutionChanged(512)).padding(5),
            button("1024").on_press(Message::ResolutionChanged(1024)).padding(5),
            button("2048").on_press(Message::ResolutionChanged(2048)).padding(5),
        ]
        .spacing(5),
        column![
            row![
                button("Save Config").on_press(Message::SaveConfig).padding(10),
                button("Load Config").on_press(Message::LoadConfig).padding(10),
            ]
            .spacing(5),
            row![
                button("Save Project").on_press(Message::SaveProject).padding(10),
                button("Load Project").on_press(Message::LoadProject).padding(10),
            ]
            .spacing(5),
        ]
        .spacing(5),
        button("Export Label")
            .on_press(Message::ExportPressed),
    ]
    .spacing(10)).style(theme::panel());
let content = column![
    title,
    row![
        main_settings,
        style_section,
    ].spacing(15),
    row![
        text_settings_section,
        image_section,
    ].spacing(15),
    row![
        image_options_section,
        image_adjustment_section,
    ].spacing(15),
    row![
        hazard_section,
        texture_section,
    ].spacing(15),
    export_section,
]
.spacing(20)
.padding(10);

iced::widget::scrollable(content)
    .height(Length::Fill)
    .width(Length::Fill)
    .into()

}
