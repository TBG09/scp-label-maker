
use crate::app::Message;
use iced::widget::{column, container, image, text, row, button, Space};
use iced::{Element, Length, alignment};

pub fn view(
    preview: &Option<iced::widget::image::Handle>,
    zoom_factor: f32,
    is_gif: bool,
    is_playing: bool,
    current_frame: usize,
    total_frames: usize,
) -> Element<'static, Message> {
    
    let zoom_controls = row![
        button("-").on_press(Message::ZoomOutPressed).padding(5),
        button("+").on_press(Message::ZoomInPressed).padding(5),
        button("Reset").on_press(Message::ZoomResetPressed).padding(5),
        text(format!("Zoom: {:.0}%", zoom_factor * 100.0))
            .size(14)
            .vertical_alignment(alignment::Vertical::Center),
    ]
    .spacing(10);

    // let gif_controls = if is_gif {
    //     let play_pause_button = if is_playing {
    //         button("⏸ Pause").on_press(Message::ToggleGifPlayback).padding(5)
    //     } else {
    //         button("▶ Play").on_press(Message::ToggleGifPlayback).padding(5)
    //     };

    //     let frame_info = text(format!("Frame: {}/{}", current_frame + 1, total_frames))
    //         .size(14)
    //         .vertical_alignment(alignment::Vertical::Center);

    //     row![
    //         play_pause_button,
    //         frame_info,
    //         text("(GIF Animation)")
    //             .size(12)
    //             .style(iced::Color::from_rgb(0.7, 0.7, 0.7))
    //             .vertical_alignment(alignment::Vertical::Center),
    //     ]
    //     .spacing(10)
    // } else {
    //     row![].spacing(0)
    // };

    let controls = if is_gif {
        column![
            zoom_controls,
            // gif_controls,
        ]
        .spacing(10)
    } else {
        column![zoom_controls]
    };

    let preview_image_element: Element<'static, Message> = if let Some(handle) = preview {
        let scaled_width = (512.0 * zoom_factor) as u16;
        let scaled_height = (512.0 * zoom_factor) as u16;
        
        image(handle.clone())
            .width(scaled_width)
            .height(scaled_height)
            .into()
    } else {
        text("Generating preview...")
            .size(14)
            .into()
    };

    let content = container(
        column![
            controls,
            Space::with_height(10),
            preview_image_element,
        ]
        .spacing(10)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(20)
    .into();

    content
}