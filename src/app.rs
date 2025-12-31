
use crate::core::{AssetManager, LabelComposer};
use crate::models::{ClassType, Hazard, ImageValidation, OutputFormat, ResizeMethod, LabelConfig, BurnType};
use crate::ui;
use crate::utils::{validate_user_image, LabelError, load_image_robustly};
use iced::widget::{column, container, text, button, scrollable, row};
use iced::{Application, Command, Element, Length, Theme, Color, Subscription};
use image::codecs::jpeg::JpegEncoder;
use image::DynamicImage;
use std::path::PathBuf;
use std::io::{Read, Write};
fn from_hex(hex: &str) -> Option<Color> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some(Color::from_rgb8(r, g, b))
}

pub struct App {
    config: LabelConfig,
    assets: Option<AssetManager>,
    composer: Option<LabelComposer>,
    preview_handle: Option<iced::widget::image::Handle>,
    validation: Option<ImageValidation>,
    loading: bool,
    modal_error: Option<String>,
    notification_message: Option<String>,
    zoom_factor: f32,
    preview_offset: (f32, f32),
    gif_frames: Option<Vec<image::RgbaImage>>,
    current_frame_index: usize,
    gif_playing: bool,
    gif_frame_delays: Vec<u32>,
    advanced_burn_settings_visible: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    AssetsLoaded(Result<AssetManager, LabelError>),
    ScpNumberChanged(String),
    ObjectClassChanged(String),
    ClassTypeSelected(ClassType),
    AlternateStyleToggled(bool),
    SelectImagePressed,
    ImageSelected(Result<PathBuf, LabelError>),
    ResizeMethodChanged(ResizeMethod),
    HazardSelected(Hazard),
    ClearHazard,
    TextureToggled(bool),
    TextureOpacityChanged(f32),
    BrightnessChanged(f32),
    ContrastChanged(f32),
    GrayscaleToggled(bool),
    ScpNumberFontSizeChanged(f32),
    ScpNumberFontSizeTextChanged(String),
    ObjectClassFontSizeChanged(f32),
    ObjectClassFontSizeTextChanged(String),
    OpacityTextChanged(String),
    BrightnessTextChanged(String),
    ContrastTextChanged(String),
    ScpTextOffsetXChanged(String),
    ScpTextOffsetYChanged(String),
    ClassTextOffsetXChanged(String),
    ClassTextOffsetYChanged(String),
    ScpTextColorChanged(Color),
    ClassTextColorChanged(Color),
    ResetText,
    SaveConfig,
    LoadConfig,
    ConfigLoaded(Result<LabelConfig, LabelError>),
    SaveProject,
    LoadProject,
    ProjectSaved(Result<PathBuf, LabelError>),
    ProjectLoaded(Result<LabelConfig, LabelError>),
    ScpNumberSubmitted(String),
    ObjectClassSubmitted(String),
    ScpNumberFontSizeSubmitted(String),
    ObjectClassFontSizeSubmitted(String),
    OpacitySubmitted(String),
    BrightnessSubmitted(String),
    ContrastSubmitted(String),
    ScpTextOffsetXSubmitted(String),
    ScpTextOffsetYSubmitted(String),
    ClassTextOffsetXSubmitted(String),
    ClassTextOffsetYSubmitted(String),
    ScpTextColorSubmitted(Color),
    ClassTextColorSubmitted(Color),
    AdvanceFrame,
    ScrollZoom(f32),
    ResolutionChanged(u32),
    FormatChanged(OutputFormat),
    ExportPressed,
    RegeneratePreview,
    PreviewGenerated(Result<Vec<u8>, LabelError>),
    ShowNotification(String),
    ZoomInPressed,
    ZoomOutPressed,
    ZoomResetPressed,
    ToggleGifPlayback,
    GifFrameDelayChanged(usize, String),
    ScpLineSpacingChanged(f32),
    ScpLineSpacingTextChanged(String),
    ClassLineSpacingChanged(f32),
    ClassLineSpacingTextChanged(String),
    BurnToggled(bool),
    BurnAmountChanged(String),
    BurnScaleChanged(f32),
    BurnDetailChanged(f32),
    BurnEdgeSoftnessChanged(f32),
    BurnIrregularityChanged(f32),
    BurnCharChanged(f32),
    BurnSeedRandomized,
    BurnSeedTextChanged(String),
    BurnSeedSubmitted,
    BurnTypeChanged(BurnType),
    ToggleAdvancedBurnSettings(bool),
    BurnScaleMultiplierChanged(f32),
    BurnDetailBlendChanged(f32),
    BurnTurbulenceFreqChanged(f32),
    BurnTurbulenceStrengthChanged(f32),
    CloseModal,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                config: LabelConfig::default(),
                assets: None,
                composer: None,
                preview_handle: None,
                validation: None,
                loading: true,
                modal_error: None,
                notification_message: None,
                zoom_factor: 1.0,
                preview_offset: (0.0, 0.0),
                gif_frames: None,
                current_frame_index: 0,
                gif_playing: true,
                gif_frame_delays: Vec::new(),
                advanced_burn_settings_visible: false,
            },
            Command::perform(
                async { crate::core::AssetManager::load_all() },
                Message::AssetsLoaded,
            ),
        )
    }

    fn title(&self) -> String {
        String::from("SCP Label Maker")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::BurnToggled(enabled) => {
                self.config.apply_burn = enabled;
                Command::perform(async {}, |_| Message::RegeneratePreview)
            }
            Message::BurnAmountChanged(s) => {
                if let Ok(value) = s.parse::<f32>() {
                    self.config.burn_amount = value.clamp(0.0, 1.0);
                }
                Command::perform(async {}, |_| Message::RegeneratePreview)
            }
            Message::BurnScaleChanged(value) => {
                self.config.burn_scale = value;
                Command::perform(async {}, |_| Message::RegeneratePreview)
            }
            Message::BurnDetailChanged(value) => {
                self.config.burn_detail = value;
                Command::perform(async {}, |_| Message::RegeneratePreview)
            }
            Message::BurnEdgeSoftnessChanged(value) => {
                self.config.burn_edge_softness = value;
                Command::perform(async {}, |_| Message::RegeneratePreview)
            }
            Message::BurnIrregularityChanged(value) => {
                self.config.burn_irregularity = value;
                Command::perform(async {}, |_| Message::RegeneratePreview)
            }
            Message::BurnCharChanged(value) => {
                self.config.burn_char = value;
                Command::perform(async {}, |_| Message::RegeneratePreview)
            }
            Message::BurnSeedRandomized => {
                self.config.burn_seed = rand::random();
                Command::perform(async {}, |_| Message::RegeneratePreview)
            }
            Message::BurnSeedTextChanged(s) => {
                if let Ok(seed) = s.parse::<u32>() {
                    self.config.burn_seed = seed;
                }
                Command::none()
            }
            Message::BurnSeedSubmitted => {
                Command::perform(async {}, |_| Message::RegeneratePreview)
            }
            Message::BurnTypeChanged(burn_type) => {
                self.config.burn_type = burn_type;
                Command::perform(async {}, |_| Message::RegeneratePreview)
            }
            Message::ToggleAdvancedBurnSettings(visible) => {
                self.advanced_burn_settings_visible = visible;
                Command::none()
            }
            Message::BurnScaleMultiplierChanged(value) => {
                self.config.burn_scale_multiplier = value;
                Command::perform(async {}, |_| Message::RegeneratePreview)
            }
            Message::BurnDetailBlendChanged(value) => {
                self.config.burn_detail_blend = value;
                Command::perform(async {}, |_| Message::RegeneratePreview)
            }
            Message::BurnTurbulenceFreqChanged(value) => {
                self.config.burn_turbulence_freq = value;
                Command::perform(async {}, |_| Message::RegeneratePreview)
            }
            Message::BurnTurbulenceStrengthChanged(value) => {
                self.config.burn_turbulence_strength = value;
                Command::perform(async {}, |_| Message::RegeneratePreview)
            }
            Message::CloseModal => {
                self.modal_error = None;
                Command::none()
            }


            Message::SaveProject => {
                Command::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .set_file_name("project.scp")
                            .add_filter("SCP Project", &["scp", "zip"])
                            .save_file()
                            .await
                            .map(|h| h.path().to_path_buf())
                            .ok_or_else(|| LabelError::Io("Save cancelled".to_string()))
                    },
                    Message::ProjectSaved,
                )
            }

            Message::ProjectSaved(result) => {
                match result {
                    Ok(path) => {
                        if let Err(e) = self.save_project(path.clone()) {
                            log::error!("Failed to save project to {:?}: {}", path, e);
                            self.modal_error = Some(e.to_string());
                        } else {
                            log::info!("Project saved successfully to {:?}", path);
                            self.notification_message = Some("Project Saved!".to_string());
                        }
                    }
                    Err(e) => {
                        log::warn!("Project save cancelled or failed: {}", e);
                        self.notification_message = Some(e.to_string());
                    }
                }
                Command::none()
            }

            Message::LoadProject => {
                return Command::perform(
                    async {
                        let handle = rfd::AsyncFileDialog::new()
                            .add_filter("SCP Project", &["scp", "zip"])
                            .pick_file()
                            .await
                            .ok_or_else(|| LabelError::Io("Load cancelled".to_string()))?;
                        
                        Self::load_project(handle.path().to_path_buf())
                    },
                    Message::ProjectLoaded
                );
            }

            Message::ProjectLoaded(result) => {
                match result {
                    Ok(config) => {
                        log::info!("Project loaded successfully.");
                        self.config = config;
                        return Command::perform(async {}, |_| Message::RegeneratePreview);
                    }
                    Err(e) => {
                        log::error!("Failed to load project: {}", e);
                        self.modal_error = Some(e.to_string());
                    }
                }
                Command::none()
            }
            Message::ScpLineSpacingChanged(value) => {
                self.config.scp_line_spacing = value;
                Command::perform(async {}, |_| Message::RegeneratePreview)
            }
            Message::ScpLineSpacingTextChanged(s) => {
                if let Ok(value) = s.parse::<f32>() {
                    self.config.scp_line_spacing = value;
                }
                Command::perform(async {}, |_| Message::RegeneratePreview)
            }
            Message::ClassLineSpacingChanged(value) => {
                self.config.class_line_spacing = value;
                Command::perform(async {}, |_| Message::RegeneratePreview)
            }
            Message::ClassLineSpacingTextChanged(s) => {
                if let Ok(value) = s.parse::<f32>() {
                    self.config.class_line_spacing = value;
                }
                Command::perform(async {}, |_| Message::RegeneratePreview)
            }


            Message::AssetsLoaded(result) => {
                match result {
                    Ok(assets) => {
                        log::info!("Assets loaded successfully.");
                        self.assets = Some(assets);
                        if let Ok(composer) = LabelComposer::new() {
                            self.composer = Some(composer);
                        }
                        self.loading = false;
                        return Command::perform(async {}, |_| Message::RegeneratePreview);
                    }
                    Err(e) => {
                        log::error!("Failed to load assets: {}", e);
                        self.modal_error = Some(e.to_string());
                        self.loading = false;
                    }
                }
                Command::none()
            }

            Message::ScpNumberChanged(text) => {
                self.config.scp_number = text;
                Command::none()
            }

            Message::ScpNumberSubmitted(text) => {
                if text.is_empty() {
                    self.config.scp_number = "000".to_string();
                    return Command::perform(async {}, move |_| Message::ShowNotification("SCP Number cannot be empty. Defaulted to '000'.".to_string()));
                }
                Command::perform(async {}, |_| Message::RegeneratePreview)
            }

            Message::ObjectClassChanged(text) => {
                self.config.object_class_text = text;
                Command::none()
            }

            Message::ObjectClassSubmitted(text) => {
                if text.is_empty() {
                    self.config.object_class_text = "SAFE".to_string();
                    return Command::perform(async {}, move |_| Message::ShowNotification("Object Class Text cannot be empty. Defaulted to 'SAFE'.".to_string()));
                }
                Command::perform(async {}, |_| Message::RegeneratePreview)
            }

            Message::ClassTypeSelected(class) => {
                self.config.class_type = class;
                return Command::perform(async {}, |_| Message::RegeneratePreview);
            }

            Message::AlternateStyleToggled(enabled) => {
                self.config.use_alternate_style = enabled;
                return Command::perform(async {}, |_| Message::RegeneratePreview);
            }

            Message::SelectImagePressed => {
                return Command::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .add_filter("Images", &["png", "jpg", "jpeg", "webp", "gif", "bmp", "tiff", "tga", "ico", "avif", "pnm", "dds", "farbfeld"])
                            .pick_file()
                            .await
                            .map(|h| h.path().to_path_buf())
                            .ok_or_else(|| crate::utils::LabelError::NoImageSelected)
                    },
                    Message::ImageSelected,
                );
            }

            Message::ImageSelected(result) => {
                match result {
                    Ok(path) => {
                        log::info!("Image selected: {:?}", path);
                        if path.extension().and_then(|s| s.to_str()) == Some("gif") {
                            match self.decode_gif(&path) {
                                Ok(_) => {
                                    self.config.image_path = Some(path);
                                    self.validation = None;
                                    return Command::perform(async {}, |_| Message::RegeneratePreview);
                                }
                                Err(e) => {
                                    log::error!("Failed to load GIF: {}", e);
                                    self.modal_error = Some(format!("Failed to load GIF: {}", e));
                                }
                            }
                        } else {
                            match load_image_robustly(&path) {
                                Ok(img) => {
                                    self.gif_frames = None;
                                    self.gif_frame_delays.clear();
                                    self.current_frame_index = 0;
                                    self.validation = Some(validate_user_image(&img));
                                    self.config.image_path = Some(path);
                                    return Command::perform(async {}, |_| Message::RegeneratePreview);
                                }
                                Err(e) => {
                                    log::error!("Could not open image: {}", e);
                                    self.modal_error = Some(format!("Could not open image: {}", e));
                                }
                            }
                        }
                    }
                    Err(e) => {
                        log::warn!("Image selection failed: {}", e);
                        self.modal_error = Some(e.to_string());
                    }
                }
                Command::none()
            }
            Message::ResizeMethodChanged(method) => {
                self.config.resize_method = method;
                return Command::perform(async {}, |_| Message::RegeneratePreview);
            }

            Message::HazardSelected(hazard) => {
                self.config.selected_hazard = Some(hazard);
                return Command::perform(async {}, |_| Message::RegeneratePreview);
            }

            Message::ClearHazard => {
                self.config.selected_hazard = None;
                return Command::perform(async {}, |_| Message::RegeneratePreview);
            }

            Message::TextureToggled(enabled) => {
                self.config.apply_texture = enabled;
                return Command::perform(async {}, |_| Message::RegeneratePreview);
            }

            Message::TextureOpacityChanged(value) => {
                self.config.texture_opacity = value;
                return Command::perform(async {}, |_| Message::RegeneratePreview);
            }

            Message::BrightnessChanged(value) => {
                self.config.brightness = value;
                return Command::perform(async {}, |_| Message::RegeneratePreview);
            }

            Message::ContrastChanged(value) => {
                self.config.contrast = value;
                return Command::perform(async {}, |_| Message::RegeneratePreview);
            }

            Message::GrayscaleToggled(enabled) => {
                self.config.grayscale = enabled;
                return Command::perform(async {}, |_| Message::RegeneratePreview);
            }

            Message::ScpNumberFontSizeChanged(size) => {
                self.config.scp_number_font_size = size;
                return Command::perform(async {}, |_| Message::RegeneratePreview);
            }

            Message::ScpNumberFontSizeTextChanged(value) => {
                if let Ok(val) = value.parse::<f32>() {
                    let clamped_val = val.clamp(24.0, 72.0);
                    if val != clamped_val {
                        return Command::perform(async {}, move |_| Message::ShowNotification(format!("SCP Number Font Size must be between 24.0 and 72.0. Adjusted to {}.", clamped_val)));
                    }
                    self.config.scp_number_font_size = clamped_val;
                    return Command::perform(async {}, |_| Message::RegeneratePreview);
                } else if value.is_empty() {
                    self.config.scp_number_font_size = 60.0;
                    return Command::perform(async {}, |_| Message::RegeneratePreview);
                }
                Command::none()
            }

            Message::ObjectClassFontSizeChanged(size) => {
                self.config.object_class_font_size = size;
                return Command::perform(async {}, |_| Message::RegeneratePreview);
            }

            Message::ObjectClassFontSizeTextChanged(value) => {
                if let Ok(val) = value.parse::<f32>() {
                    let clamped_val = val.clamp(24.0, 72.0);
                    if val != clamped_val {
                        return Command::perform(async {}, move |_| Message::ShowNotification(format!("Object Class Font Size must be between 24.0 and 72.0. Adjusted to {}.", clamped_val)));
                    }
                    self.config.object_class_font_size = clamped_val;
                    return Command::perform(async {}, |_| Message::RegeneratePreview);
                } else if value.is_empty() {
                    self.config.object_class_font_size = 60.0;
                    return Command::perform(async {}, |_| Message::RegeneratePreview);
                }
                Command::none()
            }
        
            Message::OpacityTextChanged(value) => {
                if let Ok(val) = value.parse::<f32>() {
                    let clamped_val = val.clamp(0.0, 1.0);
                    if val != clamped_val {
                        return Command::perform(async {}, move |_| Message::ShowNotification(format!("Texture Opacity must be between 0.0 and 1.0. Adjusted to {}.", clamped_val)));
                    }
                    self.config.texture_opacity = clamped_val;
                    return Command::perform(async {}, |_| Message::RegeneratePreview);
                } else if value.is_empty() {
                    self.config.texture_opacity = 0.3;
                    return Command::perform(async {}, |_| Message::RegeneratePreview);
                }
                Command::none()
            }

            Message::BrightnessTextChanged(value) => {
                if let Ok(val) = value.parse::<f32>() {
                    let clamped_val = val.clamp(-1.0, 1.0);
                    if val != clamped_val {
                        return Command::perform(async {}, move |_| Message::ShowNotification(format!("Brightness must be between -1.0 and 1.0. Adjusted to {}.", clamped_val)));
                    }
                    self.config.brightness = clamped_val;
                    return Command::perform(async {}, |_| Message::RegeneratePreview);
                } else if value.is_empty() {
                    self.config.brightness = 0.0;
                    return Command::perform(async {}, |_| Message::RegeneratePreview);
                }
                Command::none()
            }

            Message::ContrastTextChanged(value) => {
                if let Ok(val) = value.parse::<f32>() {
                    let clamped_val = val.clamp(0.0, 2.0);
                    if val != clamped_val {
                        return Command::perform(async {}, move |_| Message::ShowNotification(format!("Contrast must be between 0.0 and 2.0. Adjusted to {}.", clamped_val)));
                    }
                    self.config.contrast = clamped_val;
                    return Command::perform(async {}, |_| Message::RegeneratePreview);
                } else if value.is_empty() {
                    self.config.contrast = 1.0;
                    return Command::perform(async {}, |_| Message::RegeneratePreview);
                }
                Command::none()
            }

            Message::ScpTextOffsetXChanged(value) => {
                if let Ok(val) = value.parse::<f32>() {
                    self.config.scp_text_offset.0 = val;
                    return Command::perform(async {}, |_| Message::RegeneratePreview);
                } else if value.is_empty() {
                    self.config.scp_text_offset.0 = 2.0;
                    return Command::perform(async {}, |_| Message::RegeneratePreview);
                }
                Command::none()
            }

            Message::ScpTextOffsetYChanged(value) => {
                if let Ok(val) = value.parse::<f32>() {
                    self.config.scp_text_offset.1 = val;
                    return Command::perform(async {}, |_| Message::RegeneratePreview);
                } else if value.is_empty() {
                    self.config.scp_text_offset.1 = -7.0;
                    return Command::perform(async {}, |_| Message::RegeneratePreview);
                }
                Command::none()
            }

            Message::ClassTextOffsetXChanged(value) => {
                if let Ok(val) = value.parse::<f32>() {
                    self.config.class_text_offset.0 = val;
                    return Command::perform(async {}, |_| Message::RegeneratePreview);
                } else if value.is_empty() {
                    self.config.class_text_offset.0 = 2.0;
                    return Command::perform(async {}, |_| Message::RegeneratePreview);
                }
                Command::none()
            }

            Message::ClassTextOffsetYChanged(value) => {
                if let Ok(val) = value.parse::<f32>() {
                    self.config.class_text_offset.1 = val;
                    return Command::perform(async {}, |_| Message::RegeneratePreview);
                } else if value.is_empty() {
                    self.config.class_text_offset.1 = -7.0;
                    return Command::perform(async {}, |_| Message::RegeneratePreview);
                }
                Command::none()
            }

            Message::ScpTextColorChanged(color) => {
                self.config.scp_text_color = color.into();
                return Command::perform(async {}, |_| Message::RegeneratePreview);
            }

            Message::ClassTextColorChanged(color) => {
                self.config.class_text_color = color.into();
                return Command::perform(async {}, |_| Message::RegeneratePreview);
            }

            Message::ResetText => {
                self.config.scp_text_offset = (2.0, -7.0);
                self.config.class_text_offset = (2.0, -7.0);
                self.config.scp_text_color = Color::BLACK.into();
                self.config.class_text_color = Color::BLACK.into();
                self.config.scp_number_font_size = 60.0;
                self.config.object_class_font_size = 60.0;
                return Command::perform(async {}, |_| Message::RegeneratePreview);
            }

            Message::SaveConfig => {
                let config = self.config.clone();
                return Command::perform(
                    async move {
                        if let Some(path) = rfd::AsyncFileDialog::new().save_file().await {
                            if let Err(e) = config.save(&path.path().to_path_buf()) {
                                log::error!("Failed to save config: {}", e);
                            }
                        }
                    },
                    |_| Message::RegeneratePreview,
                );
            }

            Message::LoadConfig => {
                return Command::perform(
                    async {
                        if let Some(path) = rfd::AsyncFileDialog::new().pick_file().await {
                            LabelConfig::load(&path.path().to_path_buf())
                        } else {
                            Err(crate::utils::LabelError::Io("File selection cancelled.".to_string()))
                        }
                    },
                    Message::ConfigLoaded,
                );
            }

            Message::ConfigLoaded(result) => {
                match result {
                    Ok(config) => {
                        self.config = config;
                        return Command::perform(async {}, |_| Message::RegeneratePreview);
                    }
                    Err(e) => {
                        return Command::perform(async {}, move |_| Message::ShowNotification(format!("Failed to load config: {}", e)));
                    }
                }
            }

            Message::ResolutionChanged(res) => {
                self.config.output_resolution = res;
                Command::none()
            }

            Message::FormatChanged(format) => {
                self.config.output_format = format;
                Command::none()
            }

            Message::ExportPressed => {
                log::info!("Exporting label...");
                if let (Some(assets), Some(composer)) = (&self.assets, &self.composer) {
                    let config = self.config.clone();
                    let assets = assets.clone();
                    let composer = composer.clone();
                    let gif_frames = self.gif_frames.clone();
                    let gif_frame_delays = self.gif_frame_delays.clone();
                    
                    return Command::perform(
                        async move {
                            let dialog = if gif_frames.is_some() {
                                rfd::AsyncFileDialog::new()
                                    .set_file_name("scp_label.gif")
                                    .add_filter("GIF", &["gif"])
                                    .add_filter("PNG", &["png"])
                                    .add_filter("JPEG", &["jpg", "jpeg"])
                            } else {
                                rfd::AsyncFileDialog::new()
                                    .set_file_name("scp_label.png")
                                    .add_filter("PNG", &["png"])
                                    .add_filter("JPEG", &["jpg", "jpeg"])
                            };
                            
                            if let Some(file) = dialog.save_file().await {
                                let path = file.path();
                                let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("png");
                                
                                if extension == "gif" && gif_frames.is_some() {
                                    match Self::export_gif_static(
                                        &gif_frames.unwrap(),
                                        &gif_frame_delays,
                                        &config,
                                        &assets,
                                        &composer,
                                        path
                                    ) {
                                        Ok(_) => Message::ShowNotification("GIF exported successfully!".to_string()),
                                        Err(e) => {
                                            log::error!("GIF export failed: {}", e);
                                            Message::ShowNotification(format!("Export failed: {}", e))
                                        },
                                    }
                                } else {
                                    match composer.compose(&config, &assets, None) {
                                        Ok(img) => {
                                            let output_format = config.output_format;
                                            let output_quality = config.output_quality;

                                            let write_result = match output_format {
                                                OutputFormat::Png => img.save(path).map_err(|e| crate::utils::LabelError::ImageSaving(e.to_string())),
                                                OutputFormat::Jpeg => {
                                                    let mut buf = std::io::Cursor::new(Vec::new());
                                                    let mut encoder = JpegEncoder::new_with_quality(&mut buf, output_quality);
                                                    match encoder.encode_image(&img) {
                                                        Ok(_) => std::fs::write(path, buf.into_inner()).map_err(|e| crate::utils::LabelError::Io(e.to_string())),
                                                        Err(e) => Err(crate::utils::LabelError::ImageSaving(e.to_string()))
                                                    }
                                                }
                                            };

                                            if write_result.is_ok() {
                                                Message::ShowNotification("Label exported successfully!".to_string())
                                            } else {
                                                let err_msg = format!("Failed to save: {}", write_result.unwrap_err());
                                                log::error!("{}", err_msg);
                                                Message::ShowNotification(err_msg)
                                            }
                                        }
                                        Err(e) => {
                                            let err_msg = format!("Generation error: {}", e);
                                            log::error!("{}", err_msg);
                                            Message::ShowNotification(err_msg)
                                        }
                                    }
                                }
                            } else {
                                Message::ShowNotification("Save cancelled".to_string())
                            }
                        },
                        |msg| msg,
                    );
                }
                Command::none()
            }

            Message::RegeneratePreview => {
                if let (Some(assets), Some(composer)) = (&self.assets, &self.composer) {
                    let config = self.config.clone();
                    let assets = assets.clone();
                    let composer = composer.clone();
                    
                    let image_override = self.gif_frames.as_ref().map(|frames| {
                        let frame = &frames[self.current_frame_index % frames.len()];
                        DynamicImage::ImageRgba8(frame.clone())
                    });

                    return Command::perform(
                        async move {
                            let img = composer.compose(&config, &assets, image_override.as_ref())?;

                            let mut buffer = Vec::new();
                            if img.write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageFormat::Png).is_ok() {
                                Ok(buffer)
                            } else {
                                Err(crate::utils::LabelError::ImageProcessing("Failed to encode preview".to_string()))
                            }
                        },
                        Message::PreviewGenerated,
                    );
                }
                Command::none()
            }

            Message::PreviewGenerated(result) => {
                match result {
                    Ok(data) => {
                        self.preview_handle = Some(iced::widget::image::Handle::from_memory(data));
                    }
                    Err(e) => {
                        log::error!("Preview generation failed: {}", e);
                        self.modal_error = Some(e.to_string());
                    }
                }
                Command::none()
            }

            Message::ShowNotification(msg) => {
                self.notification_message = Some(msg);
                Command::none()
            }

            Message::AdvanceFrame => {
                if self.gif_playing {
                    if let Some(frames) = &self.gif_frames {
                        self.current_frame_index = (self.current_frame_index + 1) % frames.len();
                        return Command::perform(async {}, |_| Message::RegeneratePreview);
                    }
                }
                Command::none()
            }

            Message::ToggleGifPlayback => {
                self.gif_playing = !self.gif_playing;
                Command::none()
            }

            Message::GifFrameDelayChanged(index, value) => {
                if let Ok(delay) = value.parse::<u32>() {
                    if index < self.gif_frame_delays.len() {
                        self.gif_frame_delays[index] = delay.clamp(10, 5000);
                    }
                }
                Command::none()
            }

            Message::ZoomInPressed => {
                self.zoom_factor = (self.zoom_factor + 0.1).min(4.0);
                Command::perform(async {}, |_| Message::RegeneratePreview)
            }

            Message::ZoomOutPressed => {
                self.zoom_factor = (self.zoom_factor - 0.1).max(0.5);
                Command::perform(async {}, |_| Message::RegeneratePreview)
            }

            Message::ZoomResetPressed => {
                self.zoom_factor = 1.0;
                Command::perform(async {}, |_| Message::RegeneratePreview)
            }

            // Stubs idk mate
            Message::ScpNumberFontSizeSubmitted(_) | Message::ObjectClassFontSizeSubmitted(_) => Command::none(),
            Message::OpacitySubmitted(_) | Message::BrightnessSubmitted(_) | Message::ContrastSubmitted(_) => Command::none(),
            Message::ScpTextOffsetXSubmitted(_) | Message::ScpTextOffsetYSubmitted(_) => Command::none(),
            Message::ClassTextOffsetXSubmitted(_) | Message::ClassTextOffsetYSubmitted(_) => Command::none(),
            Message::ScpTextColorSubmitted(_) | Message::ClassTextColorSubmitted(_) => Command::none(),
            Message::ScrollZoom(_delta) => Command::none(),

            
        }

        
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.gif_frames.is_some() && self.gif_playing {
            let delay = if self.current_frame_index < self.gif_frame_delays.len() {
                self.gif_frame_delays[self.current_frame_index].max(10)
            } else {
                100
            };
            iced::time::every(std::time::Duration::from_millis(delay as u64))
                .map(|_| Message::AdvanceFrame)
        } else {
            Subscription::none()
        }
    }

    fn view(&self) -> Element<Message> {
        if let Some(error) = &self.modal_error {
            let modal_content = container(
                column![
                    text("Error").size(24).style(Color::from_rgb(0.9, 0.1, 0.1)),
                    text(error).size(16),
                    button("Close").on_press(Message::CloseModal)
                ]
                .spacing(10)
                .padding(20)
                .align_items(iced::Alignment::Center)
            )
            .style(ui::theme::card())
            .max_width(400);

            return container(modal_content)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .style(container::Appearance {
                    background: Some(iced::Background::Color(Color::from_rgba(0.1, 0.1, 0.1, 0.7))),
                    ..Default::default()
                }).into();
        }

        if self.loading {
            return container(text("Loading assets..."))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into();
        }

        let input_panel = ui::input_panel::view(&self.config, &self.validation, self.advanced_burn_settings_visible);
        
        let preview_panel = ui::preview_panel::view(
            &self.preview_handle,
            self.zoom_factor,
            self.gif_frames.is_some(),
            self.gif_playing,
            self.current_frame_index,
            self.gif_frames.as_ref().map(|f| f.len()).unwrap_or(0),
        );

        let content = row![
            container(input_panel)
                .width(Length::FillPortion(1))
                .height(Length::Fill),
            container(preview_panel)
                .width(Length::FillPortion(1))
                .height(Length::Fill),
        ]
        .spacing(20)
        .padding(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}


impl App {
fn decode_gif(&mut self, path: &PathBuf) -> Result<(), LabelError> {
    use std::fs::File;

    let file = File::open(path).map_err(|e| LabelError::Io(e.to_string()))?;
    
    let mut options = gif::DecodeOptions::new();
    options.set_color_output(gif::ColorOutput::RGBA);
    
    let mut decoder = options.read_info(file)
        .map_err(|e| LabelError::ImageProcessing(e.to_string()))?;

    let mut gif_frames = Vec::new();
    let mut gif_delays = Vec::new();

    while let Some(frame) = decoder.read_next_frame()
        .map_err(|e| LabelError::ImageProcessing(e.to_string()))? {
        
        let delay_ms = (frame.delay as u32) * 10;
        gif_delays.push(delay_ms);
        
        let width = frame.width as u32;
        let height = frame.height as u32;
        
        let rgba_image = image::RgbaImage::from_raw(width, height, frame.buffer.to_vec())
            .ok_or_else(|| LabelError::ImageProcessing("Failed to create image from GIF frame".to_string()))?;
        
        gif_frames.push(rgba_image);
    }

    self.gif_frames = Some(gif_frames);
    self.gif_frame_delays = gif_delays;
    self.current_frame_index = 0;
    
    Ok(())
}


    fn export_gif_static(
        frames: &[image::RgbaImage],
        delays: &[u32],
        config: &LabelConfig,
        assets: &AssetManager,
        composer: &LabelComposer,
        path: &std::path::Path,
    ) -> Result<(), LabelError> {
        use std::fs::File;

        if frames.is_empty() {
            return Err(LabelError::ImageProcessing("No frames to export".to_string()));
        }

        let output_size = config.output_resolution as u16;
        
        let mut file = File::create(path).map_err(|e| LabelError::Io(e.to_string()))?;
        
        let mut encoder = gif::Encoder::new(&mut file, output_size, output_size, &[])
            .map_err(|e| LabelError::ImageProcessing(e.to_string()))?;
        
        encoder.set_repeat(gif::Repeat::Infinite)
            .map_err(|e| LabelError::ImageProcessing(e.to_string()))?;

        for (i, gif_frame) in frames.iter().enumerate() {
            let dynamic_frame = DynamicImage::ImageRgba8(gif_frame.clone());
            
            let composed_label = composer.compose(&config, assets, Some(&dynamic_frame))?;
            
            let final_frame = if composed_label.width() != config.output_resolution {
                image::imageops::resize(
                    &composed_label,
                    config.output_resolution,
                    config.output_resolution,
                    image::imageops::FilterType::Lanczos3,
                )
            } else {
                composed_label
            };
            
            let delay_ms = delays.get(i).copied().unwrap_or(100);
            let delay_centisecs = (delay_ms / 10).max(1) as u16;
            
            let mut gif_frame = gif::Frame::from_rgba_speed(
                output_size,
                output_size,
                &mut final_frame.as_raw().to_vec(),
                10,
            );
            
            gif_frame.delay = delay_centisecs;
            gif_frame.dispose = gif::DisposalMethod::Background;
            
            encoder.write_frame(&gif_frame)
                .map_err(|e| LabelError::ImageProcessing(e.to_string()))?;
        }

        Ok(())
    }

    fn save_project(&self, path: PathBuf) -> Result<(), LabelError> {
        let file = std::fs::File::create(&path).map_err(|e| LabelError::Io(e.to_string()))?;
        let mut zip = zip::ZipWriter::new(file);
        
        let options = zip::write::FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        if let Some(img_path) = &self.config.image_path {
            zip.start_file("image", options).map_err(|e| LabelError::Io(e.to_string()))?;
            let img_data = std::fs::read(img_path).map_err(|e| LabelError::Io(e.to_string()))?;
            zip.write_all(&img_data).map_err(|e| LabelError::Io(e.to_string()))?;
        }

        zip.start_file("project.json", options).map_err(|e| LabelError::Io(e.to_string()))?;
        let json = serde_json::to_string_pretty(&self.config).map_err(|e| LabelError::ConfigLoading(e.to_string()))?;
        zip.write_all(json.as_bytes()).map_err(|e| LabelError::Io(e.to_string()))?;

        zip.finish().map_err(|e| LabelError::Io(e.to_string()))?;
        Ok(())
    }

    fn load_project(path: PathBuf) -> Result<LabelConfig, LabelError> {
        let file = std::fs::File::open(&path).map_err(|e| LabelError::Io(e.to_string()))?;
        let mut archive = zip::ZipArchive::new(file).map_err(|e| LabelError::Io(e.to_string()))?;

        let mut json_str = String::new();
        {
            let mut config_file = archive.by_name("project.json")
                .map_err(|_| LabelError::ConfigLoading("Missing project.json".to_string()))?;
            config_file.read_to_string(&mut json_str).map_err(|e| LabelError::Io(e.to_string()))?;
        }
        let mut config: LabelConfig = serde_json::from_str(&json_str).map_err(|e| LabelError::ConfigLoading(e.to_string()))?;

        let mut image_name = None;
        for i in 0..archive.len() {
            let file = archive.by_index(i).map_err(|e| LabelError::Io(e.to_string()))?;
            if file.name() != "project.json" {
                image_name = Some(file.name().to_string());
                break;
            }
        }

        if let Some(name) = image_name {
            let mut buffer = Vec::new();
            let mut image_file = archive.by_name(&name).map_err(|e| LabelError::Io(e.to_string()))?;
            image_file.read_to_end(&mut buffer).map_err(|e| LabelError::Io(e.to_string()))?;

            let format = image::guess_format(&buffer).map_err(|_| LabelError::ImageProcessing("Unknown format".to_string()))?;
            let ext = match format {
                image::ImageFormat::Png => "png",
                image::ImageFormat::Jpeg => "jpg",
                image::ImageFormat::Gif => "gif",
                _ => "bin",
            };

            let temp_path = std::env::temp_dir().join(format!("scp_proj_temp.{}", ext));
            std::fs::write(&temp_path, buffer).map_err(|e| LabelError::Io(e.to_string()))?;
            config.image_path = Some(temp_path);
        }

        Ok(config)
    }

}