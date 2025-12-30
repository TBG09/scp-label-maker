use super::{ClassType, Hazard};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use rand::{thread_rng, Rng};
use iced::Color;


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SerializableColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl From<iced::Color> for SerializableColor {
    fn from(color: iced::Color) -> Self {
        Self {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        }
    }
}

impl From<SerializableColor> for iced::Color {
    fn from(color: SerializableColor) -> Self {
        Self {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelConfig {
    pub scp_number: String,
    pub object_class_text: String,
    pub class_type: ClassType,
    pub use_alternate_style: bool,
    #[serde(skip)]
    pub image_path: Option<PathBuf>,
    pub resize_method: ResizeMethod,
    pub selected_hazard: Option<Hazard>,
    pub apply_texture: bool,
    pub texture_opacity: f32,
    pub output_resolution: u32,
    pub output_format: OutputFormat,
    pub output_quality: u8,
    pub brightness: f32,
    pub contrast: f32,
    pub grayscale: bool,
    pub scp_number_font_size: f32,
    pub object_class_font_size: f32,
    pub scp_text_offset: (f32, f32),
    pub class_text_offset: (f32, f32),
    pub scp_text_color: SerializableColor,
    pub class_text_color: SerializableColor,
    pub scp_line_spacing: f32,   
    pub class_line_spacing: f32,
    
}

impl Default for LabelConfig {
    fn default() -> Self {
        let mut rng = thread_rng();
        let random_scp_number = rng.gen_range(1..=1000);
        Self {
            scp_number: format!("{:03}", random_scp_number),
            object_class_text: String::from("SAFE"),
            class_type: ClassType::Safe,
            use_alternate_style: false,
            image_path: None,
            resize_method: ResizeMethod::CropToFit,
            selected_hazard: None,
            apply_texture: false,
            texture_opacity: 0.3,
            output_resolution: 512,
            output_format: OutputFormat::Png,
            output_quality: 95,
            brightness: 0.0,
            contrast: 1.0,
            grayscale: false,
            scp_number_font_size: 60.0,
            object_class_font_size: 60.0,
            scp_text_offset: (2.0, -7.0),
            class_text_offset: (2.0, -7.0),
            scp_text_color: Color::BLACK.into(),
            class_text_color: Color::BLACK.into(),
            scp_line_spacing: 1.2,
            class_line_spacing: 1.2,
            
        }
    }
}

impl LabelConfig {
    pub fn save(&self, path: &PathBuf) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load(path: &PathBuf) -> Result<Self, crate::utils::LabelError> {
        let json = std::fs::read_to_string(path)
            .map_err(|e| crate::utils::LabelError::ConfigLoading(format!("Failed to read config file: {}", e)))?;
        let config = serde_json::from_str(&json)
            .map_err(|e| crate::utils::LabelError::ConfigLoading(format!("Failed to parse config file: {}", e)))?;
        Ok(config)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, clap::ValueEnum)]
pub enum ResizeMethod {
    CropToFit,
    Stretch,
    Letterbox,
}

impl std::fmt::Display for ResizeMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResizeMethod::CropToFit => write!(f, "CropToFit"),
            ResizeMethod::Stretch => write!(f, "Stretch"),
            ResizeMethod::Letterbox => write!(f, "Letterbox"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, clap::ValueEnum)]
pub enum OutputFormat {
    Png,
    Jpeg,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Png => write!(f, "Png"),
            OutputFormat::Jpeg => write!(f, "Jpeg"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageValidation {
    pub status: ValidationStatus,
    pub source_dimensions: (u32, u32),
    pub target_dimensions: (u32, u32),
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ValidationStatus {
    PerfectFit,
    WillCrop,
    WillStretch,
    NoImage,
}