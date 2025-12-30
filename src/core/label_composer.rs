use super::{AssetManager, ImageProcessor, TextRenderer};
use crate::models::{
    AlternateLayout, CommonLayout, LabelConfig, NormalLayout, LABEL_SIZE,
};
use crate::utils::LabelError;
use image::{imageops, Rgba, RgbaImage};
use iced::Color;
use std::path::{Path, PathBuf};
use image::codecs::jpeg::JpegEncoder;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct LabelComposer {
    #[serde(skip)]
    text_renderer: TextRenderer,
}

impl LabelComposer {
    pub fn new() -> Result<Self, LabelError> {
        Ok(Self {
            text_renderer: TextRenderer::new().map_err(|e| LabelError::TextRendering(e.to_string()))?,
        })
    }

    pub fn compose(
        &self,
        config: &LabelConfig,
        assets: &AssetManager,
    ) -> Result<RgbaImage, LabelError> {
        let mut canvas = assets
            .get_template(&config.class_type, config.use_alternate_style)
            .clone()
            .into();

        self.render_scp_number(&mut canvas, &config);
        
        let object_class_region = if config.use_alternate_style {
            AlternateLayout::OBJECT_CLASS_TEXT
        } else {
            CommonLayout::OBJECT_CLASS_TEXT
        };
        
        self.text_renderer.render_text(
            &mut canvas,
            &config.object_class_text,
            object_class_region,
            Rgba([
                (Color::from(config.class_text_color).r * 255.0) as u8,
                (Color::from(config.class_text_color).g * 255.0) as u8,
                (Color::from(config.class_text_color).b * 255.0) as u8,
                255,
            ]),
            config.object_class_font_size,
            config.class_text_offset,
            config.class_line_spacing,
        );
        
        self.place_user_image(&mut canvas, config)?;
        
        self.place_hazards(&mut canvas, config, assets);
        
        if config.apply_texture {
            self.apply_texture(&mut canvas, &assets.get_texture().clone().into(), config.texture_opacity);
        }
        
        if config.output_resolution != LABEL_SIZE {
            canvas = imageops::resize(
                &canvas,
                config.output_resolution,
                config.output_resolution,
                imageops::FilterType::Lanczos3,
            );
        }
        
        Ok(canvas)
    }
        
    fn render_scp_number(&self, canvas: &mut RgbaImage, config: &LabelConfig) {
        let region = if config.use_alternate_style {
            AlternateLayout::SCP_NUMBER
        } else {
            CommonLayout::SCP_NUMBER
        };
        
        self.text_renderer.render_text(
            canvas,
            &config.scp_number,
            region,
            Rgba([
                (Color::from(config.scp_text_color).r * 255.0) as u8,
                (Color::from(config.scp_text_color).g * 255.0) as u8,
                (Color::from(config.scp_text_color).b * 255.0) as u8,
                255,
            ]),
            config.scp_number_font_size,
            config.scp_text_offset,
            config.class_line_spacing,
        );    
    }

    fn place_user_image(
        &self,
        canvas: &mut RgbaImage,
        config: &LabelConfig,
    ) -> Result<(), LabelError> {
        if config.use_alternate_style {
            return Ok(());
        }

        if let Some(path) = &config.image_path {
            let mut img = image::open(path)
                .map_err(|e| LabelError::ImageLoading(format!("Failed to open user image: {}", e)))?;
                
            if config.grayscale {
                img = img.grayscale();
            }
            img = img.adjust_contrast(config.contrast);
            img = img.brighten((config.brightness * 100.0) as i32);
            
            let processed = ImageProcessor::process_user_image(img, config.resize_method, NormalLayout::USER_IMAGE);
            
            imageops::overlay(
                canvas,
                &processed,
                NormalLayout::USER_IMAGE.x as i64,
                NormalLayout::USER_IMAGE.y as i64,
            );
        }
        Ok(())
    }
        
    fn place_hazards(
        &self,
        canvas: &mut RgbaImage,
        config: &LabelConfig,
        assets: &AssetManager,
    ) {
        if let Some(hazard) = config.selected_hazard {
            let icon: RgbaImage = assets.get_hazard_icon(&config.class_type, &hazard).clone().into();
        
            let (rect, filter) = if config.use_alternate_style {
                (AlternateLayout::HAZARD_ICON, imageops::FilterType::Lanczos3)
            } else {
                (NormalLayout::HAZARD_ICON, imageops::FilterType::Lanczos3)
            };
        
            let resized_icon = imageops::resize(&icon, rect.width, rect.height, filter);
        
            imageops::overlay(
                canvas,
                &resized_icon,
                rect.x as i64,
                rect.y as i64,
            );
        }
    }

    fn apply_texture(&self, canvas: &mut RgbaImage, texture: &RgbaImage, opacity: f32) {
        for (x, y, pixel) in canvas.enumerate_pixels_mut() {
            if let Some(tex_pixel) = texture.get_pixel_checked(x, y) {
                let alpha = (opacity * 255.0) as u8;
                let blend = |c: u8, t: u8| -> u8 {
                    ((c as u16 * (255 - alpha) as u16 + t as u16 * alpha as u16) / 255) as u8
                };

                pixel[0] = blend(pixel[0], tex_pixel[0]);
                pixel[1] = blend(pixel[1], tex_pixel[1]);
                pixel[2] = blend(pixel[2], tex_pixel[2]);
            }
        }
    }
}

pub fn generate_and_save_label(config: &LabelConfig, output_path: &PathBuf) -> Result<(), LabelError> {
    let assets = AssetManager::load_all()?;
    let composer = LabelComposer::new()?;
    let image = composer.compose(config, &assets)?;

    let output_dir = output_path.parent().unwrap_or(Path::new("."));
    std::fs::create_dir_all(output_dir)
        .map_err(|e| LabelError::Io(format!("Failed to create output directory {}: {}", output_dir.display(), e)))?;

    let mut file = std::fs::File::create(output_path)
        .map_err(|e| LabelError::Io(format!("Failed to create output file {}: {}", output_path.display(), e)))?;
    match config.output_format {
        crate::models::OutputFormat::Png => {
            image.write_to(&mut file, image::ImageFormat::Png)
                .map_err(|e| LabelError::ImageSaving(format!("Failed to save PNG image: {}", e)))?;
        }
        crate::models::OutputFormat::Jpeg => {
            let mut buf = std::io::Cursor::new(Vec::new());
            let mut encoder = JpegEncoder::new_with_quality(&mut buf, config.output_quality);
            encoder.encode_image(&image).map_err(|e| LabelError::ImageSaving(format!("Failed to encode JPEG image: {}", e)))?;
            std::fs::write(output_path, buf.into_inner())
                .map_err(|e| LabelError::Io(format!("Failed to write JPEG file: {}", e)))?;
        }
    }
    Ok(())
}