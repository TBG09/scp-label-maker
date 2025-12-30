use crate::models::{ClassType, Hazard, LABEL_SIZE};
use crate::utils::LabelError;
use image::{RgbaImage, ImageBuffer, DynamicImage};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use zip::ZipArchive;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableRgbaImage {
    pub width: u32,
    pub height: u32,
    #[serde(with = "serde_bytes")]
    pub pixels: Vec<u8>,
}

impl From<RgbaImage> for SerializableRgbaImage {
    fn from(img: RgbaImage) -> Self {
        let (width, height) = img.dimensions();
        Self {
            width,
            height,
            pixels: img.into_raw(),
        }
    }
}

impl From<SerializableRgbaImage> for RgbaImage {
    fn from(s_img: SerializableRgbaImage) -> Self {
        ImageBuffer::from_raw(s_img.width, s_img.height, s_img.pixels)
            .expect("Failed to create RgbaImage from raw data")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetManager {
    pub templates: HashMap<ClassType, (SerializableRgbaImage, SerializableRgbaImage)>,
    pub hazard_icons: HashMap<(ClassType, Hazard), SerializableRgbaImage>,
    pub texture_overlay: SerializableRgbaImage,
    pub placeholder: SerializableRgbaImage,
}

impl AssetManager {
    pub fn load_all() -> Result<Self, LabelError> {
        log::info!("Initializing AssetManager (Auto-detecting texture packs)...");

        let mut archives = Self::get_all_texture_packs();
        
        let mut templates = HashMap::new();
        let mut hazard_icons = HashMap::new();
        
        let placeholder_rgba = image::RgbaImage::from_pixel(1, 1, image::Rgba([0, 0, 0, 0]));
        let placeholder = SerializableRgbaImage::from(placeholder_rgba);

        for class in ClassType::all() {
            let primary = Self::load_asset(&class.label_path(false), &mut archives, true)?;
            let alternate = Self::load_asset(&class.label_path(true), &mut archives, true)
                .unwrap_or_else(|_| primary.clone());
            
            templates.insert(class, (primary, alternate));

            for hazard in Hazard::all() {
                if let Ok(icon) = Self::load_asset(&hazard.icon_path(&class), &mut archives, false) {
                    hazard_icons.insert((class, hazard), icon);
                }
            }
        }

        let texture_path = "resources/materials/textures/dirty_overlay.png";
        let texture_overlay = Self::load_asset(texture_path, &mut archives, true)
            .unwrap_or_else(|_| {
                log::warn!("Texture overlay not found, using transparent placeholder.");
                placeholder.clone()
            });

        log::info!(
            "Asset loading complete. Loaded from {} texture packs and local resources.", 
            archives.len()
        );

        Ok(Self {
            templates,
            hazard_icons,
            texture_overlay,
            placeholder,
        })
    }

    fn get_all_texture_packs() -> Vec<ZipArchive<File>> {
        let mut archives = Vec::new();
        let pack_dir = Path::new("texturepacks");

        if !pack_dir.exists() {
            let _ = fs::create_dir_all(pack_dir);
            return archives;
        }

        if let Ok(entries) = fs::read_dir(pack_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("zip") {
                    if let Ok(file) = File::open(&path) {
                        if let Ok(archive) = ZipArchive::new(file) {
                            log::info!("Detected texture pack: {:?}", path.file_name().unwrap());
                            archives.push(archive);
                        }
                    }
                }
            }
        }
        archives
    }

    fn load_asset(
        path: &str, 
        archives: &mut [ZipArchive<File>], 
        should_resize: bool
    ) -> Result<SerializableRgbaImage, LabelError> {
        
        for archive in archives.iter_mut().rev() {
            if let Ok(mut file) = archive.by_name(path) {
                let mut buffer = Vec::new();
                if file.read_to_end(&mut buffer).is_ok() {
                    if let Ok(img) = image::load_from_memory(&buffer) {
                        return Ok(Self::finalize_image(img, should_resize));
                    }
                }
            }
        }

        let img = image::open(path)
            .map_err(|e| LabelError::ImageLoading(format!("Asset '{}' not found in ZIPs or Disk: {}", path, e)))?;
        
        Ok(Self::finalize_image(img, should_resize))
    }

    fn finalize_image(img: DynamicImage, should_resize: bool) -> SerializableRgbaImage {
        let rgba = if should_resize && (img.width() != LABEL_SIZE || img.height() != LABEL_SIZE) {
            image::imageops::resize(
                &img, 
                LABEL_SIZE, 
                LABEL_SIZE, 
                image::imageops::FilterType::Lanczos3
            )
        } else {
            img.to_rgba8()
        };
        SerializableRgbaImage::from(rgba)
    }


    pub fn get_template(&self, class: &ClassType, alternate: bool) -> &SerializableRgbaImage {
        self.templates
            .get(class)
            .map(|(p, a)| if alternate { a } else { p })
            .unwrap_or(&self.placeholder)
    }

    pub fn get_hazard_icon(&self, class: &ClassType, hazard: &Hazard) -> &SerializableRgbaImage {
        self.hazard_icons
            .get(&(*class, *hazard))
            .unwrap_or(&self.placeholder)
    }

    pub fn get_texture(&self) -> &SerializableRgbaImage {
        &self.texture_overlay
    }
}