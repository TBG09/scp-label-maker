use crate::models::{Rectangle, ResizeMethod};
use image::{DynamicImage, GenericImageView, RgbaImage};

pub struct ImageProcessor;

impl ImageProcessor {
    pub fn process_user_image(image: DynamicImage, method: ResizeMethod, rect: Rectangle) -> RgbaImage {
        let target_width = rect.width;
        let target_height = rect.height;

        match method {
            ResizeMethod::CropToFit => Self::crop_to_fit(image, target_width, target_height),
            ResizeMethod::Stretch => Self::stretch(image, target_width, target_height),
            ResizeMethod::Letterbox => Self::letterbox(image, target_width, target_height),
        }
    }

    fn crop_to_fit(image: DynamicImage, target_w: u32, target_h: u32) -> RgbaImage {
        let (img_w, img_h) = image.dimensions();
        let img_ratio = img_w as f32 / img_h as f32;
        let target_ratio = target_w as f32 / target_h as f32;

        let (crop_w, crop_h) = if img_ratio > target_ratio {
            (img_h * target_w / target_h, img_h)
        } else {
            (img_w, img_w * target_h / target_w)
        };

        let x = (img_w - crop_w) / 2;
        let y = (img_h - crop_h) / 2;

        let cropped = image.crop_imm(x, y, crop_w, crop_h);
        image::imageops::resize(
            &cropped,
            target_w,
            target_h,
            image::imageops::FilterType::Lanczos3,
        )
    }

    fn stretch(image: DynamicImage, target_w: u32, target_h: u32) -> RgbaImage {
        image::imageops::resize(
            &image,
            target_w,
            target_h,
            image::imageops::FilterType::Lanczos3,
        )
    }

    fn letterbox(image: DynamicImage, target_w: u32, target_h: u32) -> RgbaImage {
        let (img_w, img_h) = image.dimensions();
        let img_ratio = img_w as f32 / img_h as f32;
        let target_ratio = target_w as f32 / target_h as f32;

        let (scale_w, scale_h) = if img_ratio > target_ratio {
            (target_w, (target_w as f32 / img_ratio) as u32)
        } else {
            ((target_h as f32 * img_ratio) as u32, target_h)
        };

        let scaled = image::imageops::resize(
            &image,
            scale_w,
            scale_h,
            image::imageops::FilterType::Lanczos3,
        );

        let mut result = RgbaImage::from_pixel(target_w, target_h, image::Rgba([255, 255, 255, 255]));

        let x = (target_w - scale_w) / 2;
        let y = (target_h - scale_h) / 2;

        image::imageops::overlay(&mut result, &scaled, x as i64, y as i64);
        result
    }
}