use crate::app::Message;
use iced::widget::{Row, Space, button, column, container, image, row, text};
use iced::{Element, Length, alignment};
use iced::theme::Text as TextStyle;
use crate::ui::theme;

pub fn view(
    preview: &Option<iced::widget::image::Handle>,
    zoom_factor: f32,
    is_gif: bool,
    is_playing: bool,
    current_frame: usize,
    total_frames: usize,
) -> Element<'static, Message> {
    
    let zoom_controls = container(
        row![
            button("−")
                .on_press(Message::ZoomOutPressed)
                .padding([8, 16])
                .style(iced::theme::Button::Secondary),
            button("+")
                .on_press(Message::ZoomInPressed)
                .padding([8, 16])
                .style(iced::theme::Button::Secondary),
            button("Reset")
                .on_press(Message::ZoomResetPressed)
                .padding([8, 16])
                .style(iced::theme::Button::Secondary),
            Space::with_width(15),
            container(
                text(format!("{:.0}%", zoom_factor * 100.0))
                    .size(14)
                    .style(iced::theme::Text::Color(theme::TEXT_SECONDARY))
            )
            .padding([8, 12])
            .style(theme::inline_panel()),
        ]
        .spacing(8)
        .align_items(iced::Alignment::Center)
    )
    .padding(12)
    .style(theme::card());

    let gif_controls = if is_gif {
        container(
            row![
                if is_playing {
                    button("⏸ Pause")
                        .on_press(Message::ToggleGifPlayback)
                        .padding([8, 16])
                        .style(iced::theme::Button::Primary)
                } else {
                    button("▶ Play")
                        .on_press(Message::ToggleGifPlayback)
                        .padding([8, 16])
                        .style(iced::theme::Button::Primary)
                },
                Space::with_width(15),
                container(
                    text(format!("Frame {}/{}", current_frame + 1, total_frames))
                        .size(14)
                        .style(iced::theme::Text::Color(theme::TEXT_PRIMARY))
                )
                .padding([8, 12])
                .style(theme::inline_panel()),
                container(
                    text("GIF Animation")
                        .size(12)
                        .style(iced::theme::Text::Color(theme::ACCENT))
                )
                .padding([6, 10])
                .style(theme::badge()),
            ]
            .spacing(8)
            .align_items(iced::Alignment::Center)
        )
        .padding(12)
        .style(theme::card())
    } else {
        container(column![])
    };

    let preview_element = if let Some(handle) = preview {
        let scaled_width = (512.0 * zoom_factor) as u16;
        let scaled_height = (512.0 * zoom_factor) as u16;
        
        container(
            container(
                image(handle.clone())
                    .width(scaled_width)
                    .height(scaled_height)
            )
            .padding(20)
            .style(theme::preview_backdrop())
        )
        .center_x()
        .center_y()
        .width(Length::Fill)
        .height(Length::Fill)
    } else {
        container(
            column![
                text("")
                    .size(48)
                    .style(iced::theme::Text::Color(theme::ACCENT)),
                Space::with_height(10),
                text("Generating preview...")
                    .size(16)
                    .style(iced::theme::Text::Color(theme::TEXT_SECONDARY)),
            ]
            .align_items(iced::Alignment::Center)
        )
        .center_x()
        .center_y()
        .width(Length::Fill)
        .height(Length::Fill)
    };

    let content = column![
        zoom_controls,
        if is_gif {
            Into::<Element<'static, Message>>::into(column![
                Space::with_height(12),
                gif_controls,
            ])
        } else {
            Into::<Element<'static, Message>>::into(column![])
        },
        Space::with_height(20),
        preview_element,
    ]
    .spacing(0)
    .padding(20)
    .width(Length::Fill)
    .height(Length::Fill);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}