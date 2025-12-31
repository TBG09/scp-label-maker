use rand::Rng;
use noise::{NoiseFn, Perlin, Worley};
use crate::models::BurnType;
use image::{GrayImage, Luma};

pub fn generate_burn_mask(config: &crate::models::LabelConfig, width: u32, height: u32) -> GrayImage {
    log::info!("Generating burn mask with type: {:?}", config.burn_type);
    let mut burn = match config.burn_type {
        BurnType::Perlin => {
            let perlin = Perlin::new(config.burn_seed);
            let base_scale = config.burn_scale as f64 * config.burn_scale_multiplier as f64;
            let base = generate_perlin_layer(&perlin, width, height, base_scale, 0);
            let detail_scale = base_scale * config.burn_detail as f64 * config.burn_scale_multiplier as f64;
            let detail = generate_perlin_layer(&perlin, width, height, detail_scale, 1);
            blend_images(&base, &detail, config.burn_detail_blend)
        }
        BurnType::Patches => {
            let worley = Worley::new(config.burn_seed);
            let perlin = Perlin::new(config.burn_seed + 1);
            generate_worley_layer(&worley, &perlin, width, height, config.burn_scale as f64, config.burn_detail, config.burn_turbulence_freq, config.burn_turbulence_strength)
        }
    };

    for y in 0..height {
        for x in 0..width {
            let mut val = burn.get_pixel(x, y)[0] as f32 / 255.0;

            let softness_exponent = 1.0 + config.burn_edge_softness * 4.0;
            val = val.powf(softness_exponent);

            val += (rand::random::<f32>() - 0.5) * config.burn_irregularity;
            val = val.clamp(0.0, 1.0);

            let char_power = 1.0 - config.burn_char * 0.9;
            val = val.powf(char_power);

            val *= config.burn_amount;

            burn.put_pixel(x, y, Luma([(val * 255.0) as u8]));
        }
    }

    burn
}

fn generate_worley_layer(worley: &Worley, perlin: &Perlin, width: u32, height: u32, scale: f64, detail: f32, turbulence_freq: f32, turbulence_strength: f32) -> GrayImage {
    let mut img = GrayImage::new(width, height);
    let detail_strength = detail as f64 * turbulence_strength as f64;
    for y in 0..height {
        for x in 0..width {
            let nx = x as f64 / width as f64 * scale;
            let ny = y as f64 / height as f64 * scale;

            let distortion = if turbulence_freq > 0.001 {
                perlin.get([nx * turbulence_freq as f64, ny * turbulence_freq as f64])
            } else {
                0.0
            };
            let distorted_x = nx + distortion * detail_strength;
            let distorted_y = ny + distortion * detail_strength;

            let val = ((worley.get([distorted_x, distorted_y]) + 1.0) / 2.0 * 255.0).clamp(0.0, 255.0);
            img.put_pixel(x, y, Luma([val as u8]));
        }
    }
    img
}

fn generate_perlin_layer(perlin: &Perlin, width: u32, height: u32, scale: f64, seed_offset: u32) -> GrayImage {
    let mut img = GrayImage::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let nx = x as f64 / width as f64 * scale;
            let ny = y as f64 / height as f64 * scale;
            let val = ((perlin.get([nx, ny, seed_offset as f64]) + 1.0) / 2.0 * 255.0).clamp(0.0, 255.0);
            img.put_pixel(x, y, Luma([val as u8]));
        }
    }
    img
}


pub fn perlin_noise(width: u32, height: u32, scale: f64, seed: u32) -> GrayImage {
    let perlin = Perlin::new(seed);
    let mut img = GrayImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let nx = x as f64 / width as f64 * scale;
            let ny = y as f64 / height as f64 * scale;
            let val = ((perlin.get([nx, ny]) + 1.0) / 2.0 * 255.0).clamp(0.0, 255.0);
            img.put_pixel(x, y, Luma([val as u8]));
        }
    }

    img
}

pub fn random_noise(width: u32, height: u32, intensity: u8) -> GrayImage {
    let mut img = GrayImage::new(width, height);
    let mut rng = rand::thread_rng();

    for y in 0..height {
        for x in 0..width {
            let val: u8 = rng.gen_range(0..=intensity);
            img.put_pixel(x, y, Luma([val]));
        }
    }

    img
}

pub fn blend_images(base: &GrayImage, overlay: &GrayImage, alpha: f32) -> GrayImage {
    let mut out = base.clone();

    for y in 0..base.height() {
        for x in 0..base.width() {
            let b = base.get_pixel(x, y)[0] as f32;
            let o = overlay.get_pixel(x, y)[0] as f32;
            let val = (b * (1.0 - alpha) + o * alpha).clamp(0.0, 255.0);
            out.put_pixel(x, y, Luma([val as u8]));
        }
    }

    out
}
