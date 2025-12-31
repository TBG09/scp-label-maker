use clap::{Parser, Subcommand, Args};
use iced::{Application, Settings, Color};
use anyhow::{Context};
use crate::app::App;
use crate::models::{LabelConfig, ClassType, Hazard, ResizeMethod, OutputFormat, BurnType};
use crate::core::label_composer::generate_and_save_label;
use std::path::PathBuf;
use colored::Colorize;
use crate::utils::CliExitCode;

mod app;
mod core;
mod models;
mod ui;
mod utils;

fn parse_hex_color(hex: &str) -> anyhow::Result<Color> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return Err(anyhow::anyhow!("Invalid hex color string length: {}", hex.len()));
    }
    
    let r = u8::from_str_radix(&hex[0..2], 16).context("Invalid R component")?;
    let g = u8::from_str_radix(&hex[2..4], 16).context("Invalid G component")?;
    let b = u8::from_str_radix(&hex[4..6], 16).context("Invalid B component")?;
    
    Ok(Color::from_rgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0))
}

fn parse_float_range(s: &str, min: f32, max: f32) -> anyhow::Result<f32> {
    let value: f32 = s.parse()?;
    if value >= min && value <= max {
        Ok(value)
    } else {
        Err(anyhow::anyhow!("value not in range {}-{}", min, max))
    }
}

fn parse_u8_range(s: &str, min: u8, max: u8) -> anyhow::Result<u8> {
    let value: u8 = s.parse()?;
    if value >= min && value <= max {
        Ok(value)
    } else {
        Err(anyhow::anyhow!("value not in range {}-{}", min, max))
    }
}

fn parse_non_empty_string(s: &str) -> anyhow::Result<String> {
    if s.is_empty() {
        Err(anyhow::anyhow!("value cannot be empty"))
    } else {
        Ok(s.to_string())
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Generate SCP Foundation labels with custom images and hazard warnings.", long_about = None)]
struct Cli {
    #[arg(long)]
    cli: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Generate(GenerateArgs),
}

#[derive(Args, Debug)]
struct GenerateArgs {
    #[arg(short, long, default_value_t = LabelConfig::default().scp_number.clone(), value_parser = parse_non_empty_string)]
    scp_number: String,
    #[arg(short = 'c', long, default_value_t = LabelConfig::default().object_class_text.clone(), value_parser = parse_non_empty_string)]
    object_class_text: String,

    #[arg(short = 't', long, value_enum, default_value_t = LabelConfig::default().class_type)]
    class_type: ClassType,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    use_alternate_style: bool,

    #[arg(short, long)]
    image_path: Option<PathBuf>,

    #[arg(long, value_enum, default_value_t = LabelConfig::default().resize_method)]
    resize_method: ResizeMethod,

    #[arg(short = 'z', long, value_enum)]
    hazard: Option<Hazard>,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    apply_texture: bool,

    #[arg(long, default_value_t = LabelConfig::default().texture_opacity, value_parser = |s: &str| parse_float_range(s, 0.0, 1.0))]
    texture_opacity: f32,

    #[arg(short = 'r', long, default_value_t = LabelConfig::default().output_resolution)]
    resolution: u32,

    #[arg(short = 'f', long, value_enum, default_value_t = LabelConfig::default().output_format)]
    output_format: OutputFormat,

    #[arg(short = 'q', long, default_value_t = LabelConfig::default().output_quality, value_parser = |s: &str| parse_u8_range(s, 0, 100))]
    output_quality: u8,

    #[arg(long, default_value_t = LabelConfig::default().brightness, value_parser = |s: &str| parse_float_range(s, -1.0, 1.0))]
    brightness: f32,

    #[arg(long, default_value_t = LabelConfig::default().contrast, value_parser = |s: &str| parse_float_range(s, 0.0, 2.0))]
    contrast: f32,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    grayscale: bool,

    #[arg(long, default_value_t = LabelConfig::default().scp_number_font_size, value_parser = |s: &str| parse_float_range(s, 24.0, 72.0))]
    scp_font_size: f32,

    #[arg(long, default_value_t = LabelConfig::default().object_class_font_size, value_parser = |s: &str| parse_float_range(s, 24.0, 72.0))]
    class_font_size: f32,

    #[arg(long, default_value_t = LabelConfig::default().scp_text_offset.0)]
    scp_offset_x: f32,

    #[arg(long, default_value_t = LabelConfig::default().scp_text_offset.1)]
    scp_offset_y: f32,

    #[arg(long, default_value_t = LabelConfig::default().class_text_offset.0)]
    class_offset_x: f32,

    #[arg(long, default_value_t = LabelConfig::default().class_text_offset.1)]
    class_offset_y: f32,

    #[arg(long, default_value_t = format!("#{:02x}{:02x}{:02x}", (Color::from(LabelConfig::default().scp_text_color).r * 255.0) as u8, (Color::from(LabelConfig::default().scp_text_color).g * 255.0) as u8, (Color::from(LabelConfig::default().scp_text_color).b * 255.0) as u8))]
    scp_color: String,

    #[arg(long, default_value_t = format!("#{:02x}{:02x}{:02x}", (Color::from(LabelConfig::default().class_text_color).r * 255.0) as u8, (Color::from(LabelConfig::default().class_text_color).g * 255.0) as u8, (Color::from(LabelConfig::default().class_text_color).b * 255.0) as u8))]
    class_color: String,

    #[arg(long, default_value_t = LabelConfig::default().scp_line_spacing, value_parser = |s: &str| parse_float_range(s, 0.5, 3.0))]
    scp_line_spacing: f32,

    #[arg(long, default_value_t = LabelConfig::default().class_line_spacing, value_parser = |s: &str| parse_float_range(s, 0.5, 3.0))]
    class_line_spacing: f32,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    apply_burn: bool,

    #[arg(long, value_enum, default_value_t = LabelConfig::default().burn_type)]
    burn_type: BurnType,

    #[arg(long, default_value_t = LabelConfig::default().burn_amount,
        value_parser = |s: &str| parse_float_range(s, 0.0, 1.0))]
    burn_amount: f32,

    #[arg(long, default_value_t = LabelConfig::default().burn_scale,
        value_parser = |s: &str| parse_float_range(s, 0.1, 5.0))]
    burn_scale: f32,

    #[arg(long, default_value_t = LabelConfig::default().burn_detail,
        value_parser = |s: &str| parse_float_range(s, 0.0, 1.0))]
    burn_detail: f32,

    #[arg(long, default_value_t = LabelConfig::default().burn_edge_softness,
        value_parser = |s: &str| parse_float_range(s, 0.0, 1.0))]
    burn_edge_softness: f32,

    #[arg(long, default_value_t = LabelConfig::default().burn_irregularity,
        value_parser = |s: &str| parse_float_range(s, 0.0, 1.0))]
    burn_irregularity: f32,

    #[arg(long, default_value_t = LabelConfig::default().burn_char,
        value_parser = |s: &str| parse_float_range(s, 0.0, 1.0))]
    burn_char: f32,

    #[arg(long, default_value_t = LabelConfig::default().burn_seed)]
    burn_seed: u32,

    #[arg(long, default_value_t = LabelConfig::default().burn_scale_multiplier)]
    burn_scale_multiplier: f32,

    #[arg(long, default_value_t = LabelConfig::default().burn_detail_blend, value_parser = |s: &str| parse_float_range(s, 0.0, 1.0))]
    burn_detail_blend: f32,

    #[arg(long, default_value_t = LabelConfig::default().burn_turbulence_freq)]
    burn_turbulence_freq: f32,

    #[arg(long, default_value_t = LabelConfig::default().burn_turbulence_strength, value_parser = |s: &str| parse_float_range(s, 0.0, 1.0))]
    burn_turbulence_strength: f32,

    #[arg(short, long)]
    output: PathBuf,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let has_cli_args = args.len() > 1;

    if atty::is(atty::Stream::Stdout) || has_cli_args {
        utils::setup_logger().expect("Failed to set up logger");
    }

    let cli = Cli::parse();

    let result = if has_cli_args || cli.cli {
        match cli.command {
            Some(Commands::Generate(args)) => {
                println!("{}", "Running in CLI mode to generate label.".green());
                run_cli(args).context("Failed to generate label via CLI")
            }
            None => {
                Err(anyhow::anyhow!("CLI mode specified but no command given. Use `scp-label-maker --help` for more information."))
            }
        }
    } else {
        println!("{}", "Running in GUI mode.".green());
        App::run(Settings::default()).map_err(|e| anyhow::anyhow!("GUI application error: {}", e))
    };

    if let Err(e) = result {
        eprintln!("{}: {:?}", "Error".red().bold(), e);
        let exit_code = if let Some(label_err) = e.downcast_ref::<crate::utils::LabelError>() {
            label_err.to_exit_code()
        } else {
            CliExitCode::GenericError
        };
        std::process::exit(exit_code as i32);
    }
}

fn run_cli(args: GenerateArgs) -> anyhow::Result<()> {
    let scp_text_color = parse_hex_color(&args.scp_color)
        .context(format!("Invalid SCP number color '{}'", args.scp_color))?
        .into();
    let class_text_color = parse_hex_color(&args.class_color)
        .context(format!("Invalid object class color '{}'", args.class_color))?
        .into();

    let config = LabelConfig {
        scp_number: args.scp_number,
        object_class_text: args.object_class_text,
        class_type: args.class_type,
        use_alternate_style: args.use_alternate_style,
        image_path: args.image_path,
        resize_method: args.resize_method,
        selected_hazard: args.hazard,
        apply_texture: args.apply_texture,
        texture_opacity: args.texture_opacity,
        output_resolution: args.resolution,
        output_format: args.output_format,
        output_quality: args.output_quality,
        brightness: args.brightness,
        contrast: args.contrast,
        grayscale: args.grayscale,
        scp_number_font_size: args.scp_font_size,
        object_class_font_size: args.class_font_size,
        scp_text_offset: (args.scp_offset_x, args.scp_offset_y),
        class_text_offset: (args.class_offset_x, args.class_offset_y),
        scp_text_color,
        class_text_color,
        scp_line_spacing: args.scp_line_spacing,
        class_line_spacing: args.class_line_spacing,
        apply_burn: args.apply_burn,
        burn_type: args.burn_type,
        burn_amount: args.burn_amount,
        burn_scale: args.burn_scale,
        burn_detail: args.burn_detail,
        burn_edge_softness: args.burn_edge_softness,
        burn_irregularity: args.burn_irregularity,
        burn_char: args.burn_char,
        burn_seed: args.burn_seed,
        burn_scale_multiplier: args.burn_scale_multiplier,
        burn_detail_blend: args.burn_detail_blend,
        burn_turbulence_freq: args.burn_turbulence_freq,
        burn_turbulence_strength: args.burn_turbulence_strength,
    };

    println!("{}", format!("Generating label for SCP-{}...", config.scp_number).cyan());
            generate_and_save_label(&config, &args.output)
        .context(format!("Failed to generate and save label to {}", args.output.display()))?;

    println!("{}", format!("Successfully generated label to {}", args.output.display()).green().bold());
    Ok(())
}