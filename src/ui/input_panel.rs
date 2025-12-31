use crate::app::Message;
use crate::models::{ClassType, Hazard, ImageValidation, LabelConfig, ResizeMethod, ValidationStatus, BurnType};
use iced::widget::{button, checkbox, column, container, pick_list, row, slider, text, text_input, Space, radio};
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

fn section_header(title: &str) -> iced::widget::Text<'static, iced::Theme> {
    text(title)
        .size(16)
        .style(iced::theme::Text::Color(theme::ACCENT))
}

fn label_text(title: &str) -> iced::widget::Text<'static, iced::Theme> {
    text(title)
        .size(13)
        .style(iced::theme::Text::Color(theme::TEXT_SECONDARY))
}

pub fn view(config: &LabelConfig, validation: &Option<ImageValidation>, advanced_burn_settings_visible: bool) -> Element<'static, Message> {
    let title = text("SCP Label Maker")
        .size(28)
        .style(iced::theme::Text::Color(Color::WHITE));
    
    let subtitle = text("Create custom SCP Foundation labels")
        .size(14)
        .style(iced::theme::Text::Color(theme::TEXT_SECONDARY));

    let scp_text_color = config.scp_text_color;
    let class_text_color = config.class_text_color;
    
    let scp_input = column![
        label_text("SCP Number"),
        row![
            text("SCP-")
                .size(20)
                .style(iced::theme::Text::Color(theme::ACCENT)),
            text_input("001", &config.scp_number)
                .on_input(Message::ScpNumberChanged)
                .on_submit(Message::ScpNumberSubmitted(config.scp_number.clone()))
                .padding(10)
                .width(120)
        ]
        .spacing(8)
        .align_items(iced::Alignment::Center),
    ]
    .spacing(8);

    let class_input = column![
        label_text("Object Class"),
        text_input("SAFE", &config.object_class_text)
            .on_input(Message::ObjectClassChanged)
            .on_submit(Message::ObjectClassSubmitted(config.object_class_text.clone()))
            .padding(10)
            .width(200)
    ]
    .spacing(8);

    let class_picker = column![
        label_text("Visual Style"),
        pick_list(
            ClassType::all(),
            Some(config.class_type),
            Message::ClassTypeSelected,
        )
        .padding(10)
        .width(200),
    ]
    .spacing(8);

    let alternate_toggle = checkbox(
        "Use alternate style",
        config.use_alternate_style
    )
    .on_toggle(Message::AlternateStyleToggled)
    .text_size(13);

    let basic_settings = container(
        column![
            section_header("Basic Settings"),
            Space::with_height(10),
            row![
                scp_input,
                Space::with_width(20),
                class_input,
            ]
            .spacing(15),
            Space::with_height(15),
            row![
                class_picker,
                Space::with_width(20),
                column![
                    Space::with_height(20),
                    alternate_toggle
                ]
            ]
            .spacing(15),
        ]
        .spacing(12)
        .padding(20)
    )
    .style(theme::card());

    let text_size_controls = row![
        column![
            label_text("SCP Number Size"),
            container(
                row![
                    container(
                        slider(24.0..=72.0, config.scp_number_font_size, Message::ScpNumberFontSizeChanged)
                            .step(1.0)
                            .width(200)
                    )
                    .padding([0, 8]),
                    container(
                        text_input("60", &config.scp_number_font_size.to_string())
                            .on_input(Message::ScpNumberFontSizeTextChanged)
                            .on_submit(Message::ScpNumberFontSizeSubmitted(config.scp_number_font_size.to_string()))
                            .padding(8)
                            .width(65)
                    )
                    .style(theme::input_container()),
                ]
                .spacing(12)
                .align_items(iced::Alignment::Center)
            )
            .padding(10)
            .style(theme::slider_container()),
        ]
        .spacing(8),
        Space::with_width(20),
        column![
            label_text("Object Class Size"),
            container(
                row![
                    container(
                        slider(24.0..=72.0, config.object_class_font_size, Message::ObjectClassFontSizeChanged)
                            .step(1.0)
                            .width(200)
                    )
                    .padding([0, 8]),
                    container(
                        text_input("60", &config.object_class_font_size.to_string())
                            .on_input(Message::ObjectClassFontSizeTextChanged)
                            .on_submit(Message::ObjectClassFontSizeSubmitted(config.object_class_font_size.to_string()))
                            .padding(8)
                            .width(65)
                    )
                    .style(theme::input_container()),
                ]
                .spacing(12)
                .align_items(iced::Alignment::Center)
            )
            .padding(10)
            .style(theme::slider_container()),
        ]
        .spacing(8),
    ]
    .spacing(15);

    let line_spacing_controls = row![
        column![
            label_text("SCP Line Spacing"),
            row![
                slider(0.5..=3.0, config.scp_line_spacing, Message::ScpLineSpacingChanged)
                    .step(0.05)
                    .width(180),
                text_input("1.2", &format!("{:.2}", config.scp_line_spacing))
                    .on_input(Message::ScpLineSpacingTextChanged)
                    .padding(8)
                    .width(70),
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center),
        ]
        .spacing(8),
        Space::with_width(20),
        column![
            label_text("Class Line Spacing"),
            row![
                slider(0.5..=3.0, config.class_line_spacing, Message::ClassLineSpacingChanged)
                    .step(0.05)
                    .width(180),
                text_input("1.2", &format!("{:.2}", config.class_line_spacing))
                    .on_input(Message::ClassLineSpacingTextChanged)
                    .padding(8)
                    .width(70),
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center),
        ]
        .spacing(8),
    ]
    .spacing(15);

    let color_controls = row![
        column![
            label_text("SCP Number Color"),
            text_input(
                "#000000",
                &format!(
                    "#{:02x}{:02x}{:02x}",
                    (Color::from(config.scp_text_color).r * 255.0) as u8,
                    (Color::from(config.scp_text_color).g * 255.0) as u8,
                    (Color::from(config.scp_text_color).b * 255.0) as u8
                )
            )
            .on_input(move |s| {
                if let Ok(color) = parse_hex_color(&s) {
                    Message::ScpTextColorChanged(color)
                } else {
                    Message::ScpTextColorChanged(scp_text_color.into())
                }
            })
            .on_submit(Message::ScpTextColorSubmitted(config.scp_text_color.into()))
            .padding(10)
            .width(120),
        ]
        .spacing(8),
        Space::with_width(20),
        column![
            label_text("Object Class Color"),
            text_input(
                "#000000",
                &format!(
                    "#{:02x}{:02x}{:02x}",
                    (Color::from(config.class_text_color).r * 255.0) as u8,
                    (Color::from(config.class_text_color).g * 255.0) as u8,
                    (Color::from(config.class_text_color).b * 255.0) as u8
                )
            )
            .on_input(move |s| {
                if let Ok(color) = parse_hex_color(&s) {
                    Message::ClassTextColorChanged(color)
                } else {
                    Message::ClassTextColorChanged(class_text_color.into())
                }
            })
            .on_submit(Message::ClassTextColorSubmitted(config.class_text_color.into()))
            .padding(10)
            .width(120),
        ]
        .spacing(8),
    ]
    .spacing(15);

    let offset_controls = row![
        column![
            label_text("SCP Number Offset (X, Y)"),
            row![
                text_input("0.0", &format!("{:.2}", config.scp_text_offset.0))
                    .on_input(Message::ScpTextOffsetXChanged)
                    .on_submit(Message::ScpTextOffsetXSubmitted(config.scp_text_offset.0.to_string()))
                    .padding(8)
                    .width(80),
                text_input("0.0", &format!("{:.2}", config.scp_text_offset.1))
                    .on_input(Message::ScpTextOffsetYChanged)
                    .on_submit(Message::ScpTextOffsetYSubmitted(config.scp_text_offset.1.to_string()))
                    .padding(8)
                    .width(80),
            ]
            .spacing(8),
        ]
        .spacing(8),
        Space::with_width(20),
        column![
            label_text("Object Class Offset (X, Y)"),
            row![
                text_input("0.0", &format!("{:.2}", config.class_text_offset.0))
                    .on_input(Message::ClassTextOffsetXChanged)
                    .on_submit(Message::ClassTextOffsetXSubmitted(config.class_text_offset.0.to_string()))
                    .padding(8)
                    .width(80),
                text_input("0.0", &format!("{:.2}", config.class_text_offset.1))
                    .on_input(Message::ClassTextOffsetYChanged)
                    .on_submit(Message::ClassTextOffsetYSubmitted(config.class_text_offset.1.to_string()))
                    .padding(8)
                    .width(80),
            ]
            .spacing(8),
        ]
        .spacing(8),
    ]
    .spacing(15);

    let text_customization = container(
        column![
            section_header("Text Customization"),
            Space::with_height(5),
            text("Tip: Use \\n to create new lines in text fields")
                .size(12)
                .style(iced::theme::Text::Color(Color::from_rgb(0.5, 0.7, 0.9))),
            Space::with_height(15),
            text_size_controls,
            Space::with_height(15),
            line_spacing_controls,
            Space::with_height(15),
            color_controls,
            Space::with_height(15),
            offset_controls,
            Space::with_height(15),
            button("Reset All Text Settings")
                .on_press(Message::ResetText)
                .padding(10)
                .style(iced::theme::Button::Secondary),
        ]
        .spacing(12)
        .padding(20)
    )
    .style(theme::card());

    let validation_display = if let Some(val) = validation {
        let (icon, color) = match val.status {
            ValidationStatus::PerfectFit => ("✓", theme::SUCCESS),
            ValidationStatus::WillCrop => ("⚠", theme::WARNING),
            ValidationStatus::WillStretch => ("⚠", Color::from_rgb(0.9, 0.3, 0.3)),
            ValidationStatus::NoImage => ("ℹ", theme::TEXT_SECONDARY),
        };
        
        row![
            text(icon).size(16).style(iced::theme::Text::Color(color)),
            text(&val.message)
                .size(13)
                .style(iced::theme::Text::Color(color)),
        ]
        .spacing(8)
        .align_items(iced::Alignment::Center)
    } else {
        row![]
    };

    let image_section = container(
        column![
            section_header("Image"),
            Space::with_height(10),
            button("Select Image")
                .on_press(Message::SelectImagePressed)
                .padding(12)
                .style(iced::theme::Button::Primary),
            if let Some(path) = &config.image_path {
                Into::<Element<'static, Message>>::into(
                    text(format!("{}", path.file_name().unwrap().to_string_lossy()))
                        .size(12)
                        .style(iced::theme::Text::Color(theme::TEXT_SECONDARY))
                )
            } else {
                Into::<Element<'static, Message>>::into(
                    text("No image selected")
                        .size(12)
                        .style(iced::theme::Text::Color(theme::TEXT_SECONDARY))
                )
            },
            Space::with_height(10),
            validation_display,
            Space::with_height(15),
            column![
                label_text("Resize Method"),
                pick_list(
                    vec![ResizeMethod::CropToFit, ResizeMethod::Stretch, ResizeMethod::Letterbox],
                    Some(config.resize_method),
                    Message::ResizeMethodChanged
                )
                .padding(10)
                .width(200),
            ]
            .spacing(8),
        ]
        .spacing(12)
        .padding(20)
    )
    .style(theme::card());

    let image_adjustments = if !config.use_alternate_style {
        container(
            column![
                section_header("Image Adjustments"),
                Space::with_height(10),
                column![
                    label_text(&format!("Brightness: {:.2}", config.brightness)),
                    row![
                        slider(-1.0..=1.0, config.brightness, Message::BrightnessChanged)
                            .step(0.05)
                            .width(250),
                        text_input("0.0", &format!("{:.2}", config.brightness))
                            .on_input(Message::BrightnessTextChanged)
                            .on_submit(Message::BrightnessSubmitted(config.brightness.to_string()))
                            .padding(8)
                            .width(70),
                    ]
                    .spacing(10)
                    .align_items(iced::Alignment::Center),
                ]
                .spacing(8),
                Space::with_height(10),
                column![
                    label_text(&format!("Contrast: {:.2}", config.contrast)),
                    row![
                        slider(0.0..=2.0, config.contrast, Message::ContrastChanged)
                            .step(0.05)
                            .width(250),
                        text_input("1.0", &format!("{:.2}", config.contrast))
                            .on_input(Message::ContrastTextChanged)
                            .on_submit(Message::ContrastSubmitted(config.contrast.to_string()))
                            .padding(8)
                            .width(70),
                    ]
                    .spacing(10)
                    .align_items(iced::Alignment::Center),
                ]
                .spacing(8),
                Space::with_height(10),
                checkbox("Grayscale", config.grayscale)
                    .on_toggle(Message::GrayscaleToggled)
                    .text_size(13),
            ]
            .spacing(12)
            .padding(20)
        )
        .style(theme::card())
    } else {
        container(column![])
    };

    let hazard_section = column![
        label_text("Hazard Warning"),
        row![
            pick_list(
                Hazard::all(),
                config.selected_hazard,
                Message::HazardSelected
            )
            .padding(10)
            .width(200),
            button("Clear")
                .on_press(Message::ClearHazard)
                .padding(10)
                .style(iced::theme::Button::Secondary),
        ]
        .spacing(10),
    ]
    .spacing(8);

    let texture_section = column![
        checkbox("Apply texture overlay", config.apply_texture)
            .on_toggle(Message::TextureToggled)
            .text_size(13),
        if config.apply_texture {
            Into::<Element<'static, Message>>::into(
                column![
                    Space::with_height(8),
                    label_text(&format!("Opacity: {:.0}%", config.texture_opacity * 100.0)),
                    row![
                        slider(0.0..=1.0, config.texture_opacity, |v| Message::OpacityTextChanged(v.to_string()))
                            .step(0.05)
                            .width(180),
                        text_input("0.3", &format!("{:.2}", config.texture_opacity))
                            .on_input(Message::OpacityTextChanged)
                            .on_submit(Message::OpacitySubmitted(config.texture_opacity.to_string()))
                            .padding(8)
                            .width(70),
                    ]
                    .spacing(10)
                    .align_items(iced::Alignment::Center),
                ]
                .spacing(8)
            )
        } else {
            Into::<Element<'static, Message>>::into(column![])
        }
    ]
    .spacing(8);

    let burn_section = column![
        checkbox("Apply burn overlay", config.apply_burn)
            .on_toggle(Message::BurnToggled)
            .text_size(13),
        if config.apply_burn {
            let advanced_burn_controls = if advanced_burn_settings_visible {
                column![
                    Space::with_height(10),
                    label_text(&format!("Scale Multiplier: {:.2}", config.burn_scale_multiplier)),
                    slider(1.0..=20.0, config.burn_scale_multiplier, Message::BurnScaleMultiplierChanged).step(0.1),
                    label_text(&format!("Detail Blend: {:.2}", config.burn_detail_blend)),
                    slider(0.0..=1.0, config.burn_detail_blend, Message::BurnDetailBlendChanged).step(0.05),
                    label_text(&format!("Turbulence Freq: {:.2}", config.burn_turbulence_freq)),
                    slider(0.1..=10.0, config.burn_turbulence_freq, Message::BurnTurbulenceFreqChanged).step(0.1),
                    label_text(&format!("Turbulence Strength: {:.2}", config.burn_turbulence_strength)),
                    slider(0.0..=1.0, config.burn_turbulence_strength, Message::BurnTurbulenceStrengthChanged).step(0.01),
                ].spacing(8)
            } else {
                column![]
            };

            Into::<Element<'static, Message>>::into(
                column![
                    Space::with_height(8),
                    label_text("Burn Style"),
                    pick_list(
                        vec![BurnType::Perlin, BurnType::Patches],
                        Some(config.burn_type),
                        Message::BurnTypeChanged,
                    )
                    .padding(10),
                    Space::with_height(10),
                    label_text(&format!("Burn Amount: {:.0}%", config.burn_amount * 100.0)),
                    row![
                        slider(0.0..=1.0, config.burn_amount, |v| Message::BurnAmountChanged(v.to_string()))
                            .step(0.01)
                            .width(180),
                        text_input("0.35", &format!("{:.2}", config.burn_amount))
                            .on_input(Message::BurnAmountChanged)
                            .padding(8)
                            .width(70),
                    ]
                    .spacing(10)
                    .align_items(iced::Alignment::Center),

                    label_text(&format!("Burn Scale: {:.2}", config.burn_scale)),
                    slider(0.1..=10.0, config.burn_scale, Message::BurnScaleChanged)
                        .step(0.05)
                        .width(250),

                    label_text(&format!("Burn Detail: {:.2}", config.burn_detail)),
                    slider(0.0..=1.0, config.burn_detail, Message::BurnDetailChanged)
                        .step(0.05)
                        .width(250),

                    label_text(&format!("Edge Softness: {:.2}", config.burn_edge_softness)),
                    slider(0.0..=1.0, config.burn_edge_softness, Message::BurnEdgeSoftnessChanged)
                        .step(0.05)
                        .width(250),

                    label_text(&format!("Irregularity: {:.2}", config.burn_irregularity)),
                    slider(0.0..=1.0, config.burn_irregularity, Message::BurnIrregularityChanged)
                        .step(0.05)
                        .width(250),

                    label_text(&format!("Edge Darkness (Char): {:.2}", config.burn_char)),
                    slider(0.0..=1.0, config.burn_char, Message::BurnCharChanged)
                        .step(0.05)
                        .width(250),

                    label_text(&format!("Seed: {}", config.burn_seed)),
                    row![
                        text_input("Seed", &config.burn_seed.to_string())
                            .on_input(Message::BurnSeedTextChanged)
                            .on_submit(Message::BurnSeedSubmitted)
                            .padding(8)
                            .width(100),
                        button("Randomize")
                            .on_press(Message::BurnSeedRandomized)
                            .padding(8)
                            .style(iced::theme::Button::Secondary),
                    ]
                    .spacing(10),
                    Space::with_height(15),
                    checkbox("Advanced Burn Settings", advanced_burn_settings_visible)
                        .on_toggle(Message::ToggleAdvancedBurnSettings),
                    advanced_burn_controls,
                ]
                .spacing(8)
            )
        } else {
            Into::<Element<'static, Message>>::into(column![])
        }
    ]
    .spacing(8);

    let effects_section = container(
        column![
            section_header("Effects & Overlays"),
            Space::with_height(10),
            hazard_section,
            Space::with_height(15),
            texture_section,
            Space::with_height(15),
            burn_section,
        ]
        .spacing(12)
        .padding(20)
    )
    .style(theme::card());

    let export_section = container(
        column![
            section_header("Export & Project"),
            Space::with_height(10),
            label_text("Resolution:"),
            row(
                [512, 1024, 2048].iter().map(|&res| {
                    radio(
                        format!("{}px", res),
                        res,
                        Some(config.output_resolution),
                        Message::ResolutionChanged,
                    )
                    .into()
                }).collect::<Vec<_>>()
            ).spacing(10),
            Space::with_height(5),
            text("Note: Increasing resolution interpolates the image, it does not add new detail.")
                .size(12)
                .style(iced::theme::Text::Color(theme::TEXT_SECONDARY)),
            Space::with_height(15),
            row![
                button("Save Config")
                    .on_press(Message::SaveConfig)
                    .padding(10)
                    .style(iced::theme::Button::Secondary),
                button("Load Config")
                    .on_press(Message::LoadConfig)
                    .padding(10)
                    .style(iced::theme::Button::Secondary),
                Space::with_width(10),
                button(" Save Project")
                    .on_press(Message::SaveProject)
                    .padding(10)
                    .style(iced::theme::Button::Secondary),
                button(" Load Project")
                    .on_press(Message::LoadProject)
                    .padding(10)
                    .style(iced::theme::Button::Secondary),
            ]
            .spacing(8),
            Space::with_height(15),
            button("Export Label")
                .on_press(Message::ExportPressed)
                .padding(15)
                .style(iced::theme::Button::Primary),
        ]
        .spacing(12)
        .padding(20)
    )
    .style(theme::card());

    let content = column![
        column![
            title,
            subtitle,
        ]
        .spacing(5)
        .padding(5),
        Space::with_height(20),
        basic_settings,
        Space::with_height(15),
        text_customization,
        Space::with_height(15),
        row![
            image_section,
            Space::with_width(15),
            if !config.use_alternate_style {
                Into::<Element<'static, Message>>::into(image_adjustments)
            } else {
                Into::<Element<'static, Message>>::into(container(column![]))
            }
        ]
        .spacing(15),
        Space::with_height(15),
        effects_section,
        Space::with_height(15),
        export_section,
        Space::with_height(20),
    ]
    .spacing(0)
    .padding(20);

    iced::widget::scrollable(content)
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
}