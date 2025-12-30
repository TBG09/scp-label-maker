use crate::models::{Alignment, TextRegion};
use crate::utils::LabelError;
use rusttype::{Font, Scale};
use image::{Rgba, RgbaImage};
use imageproc::drawing::{draw_text_mut, text_size};
use serde::{Serialize, Deserialize};
use serde_bytes;

#[derive(Clone)]
pub struct TextRenderer {
    font: Font<'static>,
}

impl TextRenderer {
    pub fn new() -> Result<Self, LabelError> {
        let font_bytes: &'static [u8] = include_bytes!("../../assets/fonts/impact.ttf");
        let font = Font::try_from_bytes(font_bytes)
            .ok_or_else(|| LabelError::TextRendering("Failed to load font".to_string()))?;

        Ok(Self { font })
    }

pub fn render_text(
        &self,
        canvas: &mut RgbaImage,
        text: &str,
        region: TextRegion,
        color: Rgba<u8>,
        font_size: f32,
        offset: (f32, f32),
        line_spacing_multiplier: f32,
    ) {
        if text.is_empty() {
            return;
        }

        let scale = Scale::uniform(font_size);
        
        let processed_text = text.replace("\\n", "\n");
        let lines: Vec<&str> = processed_text.split('\n').collect();
        
        let (_, glyph_height) = text_size(scale, &self.font, "Hg"); 
        let line_spacing = (glyph_height as f32 * line_spacing_multiplier) as i32;
        
        let total_block_height = if lines.len() > 1 {
            (lines.len() as i32 - 1) * line_spacing + glyph_height as i32
        } else {
            glyph_height as i32
        };

        for (i, line) in lines.iter().enumerate() {
            if line.trim().is_empty() && lines.len() > 1 { continue; }

            let (text_w, _) = text_size(scale, &self.font, line);

            let x = match region.alignment {
                Alignment::Left => region.x as i32,
                Alignment::Center => (region.x + region.max_width / 2) as i32 - (text_w / 2) as i32,
                Alignment::Right => (region.x + region.max_width) as i32 - text_w as i32,
                Alignment::CenterLeft => region.x as i32,
            } + offset.0 as i32;

            let y = (region.y as i32 - (total_block_height / 2)) 
                    + (i as i32 * line_spacing) 
                    + offset.1 as i32;

            draw_text_mut(canvas, color, x, y, scale, &self.font, line);
        }
    }
    pub fn render_text_with_stroke(
        &self,
        canvas: &mut RgbaImage,
        text: &str,
        region: TextRegion,
        color: Rgba<u8>,
        stroke_color: Rgba<u8>,
        font_size: f32,
        offset: (f32, f32),
    ) {
        if text.is_empty() {
            return;
        }

        let scale = Scale::uniform(font_size);
        let (text_w, text_h) = text_size(scale, &self.font, text);

        let x = match region.alignment {
            Alignment::Left => region.x as i32,
            Alignment::Center => (region.x + region.max_width / 2) as i32 - (text_w / 2) as i32,
            Alignment::Right => (region.x + region.max_width) as i32 - text_w as i32,
            Alignment::CenterLeft => region.x as i32,
        } + offset.0 as i32;

        let y = (region.y as i32 - text_h as i32 / 2) + offset.1 as i32;

        for dx in -2..=2 {
            for dy in -2..=2 {
                if dx != 0 || dy != 0 {
                    draw_text_mut(
                        canvas,
                        stroke_color,
                        x + dx,
                        y + dy,
                        scale,
                        &self.font,
                        text,
                    );
                }
            }
        }

        draw_text_mut(canvas, color, x, y, scale, &self.font, text);
    }

    pub fn from_font_data(font_data: Vec<u8>) -> Result<Self, LabelError> {
        let leaked_font_data: &'static [u8] = Box::leak(font_data.into_boxed_slice());
        let font = Font::try_from_bytes(leaked_font_data)
            .ok_or_else(|| LabelError::TextRendering("Failed to load font".to_string()))?;
        Ok(Self { font })
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SerializableTextRenderer {
    #[serde(with = "serde_bytes")]
    font_bytes: Vec<u8>,
}

impl From<&TextRenderer> for SerializableTextRenderer {
    fn from(_renderer: &TextRenderer) -> Self {

        let font_data = include_bytes!("../../assets/fonts/impact.ttf").to_vec();
        SerializableTextRenderer {
            font_bytes: font_data,
        }
    }
}

impl SerializableTextRenderer {
    pub fn to_text_renderer(&self) -> Result<TextRenderer, LabelError> {
        TextRenderer::from_font_data(self.font_bytes.clone())
    }
}