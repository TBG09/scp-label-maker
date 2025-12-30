use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum LabelError {
    #[error("Failed to load image: {0}")]
    ImageLoading(String),

    #[error("Failed to process image: {0}")]
    ImageProcessing(String),

    #[error("Failed to save image: {0}")]
    ImageSaving(String),

    #[error("Failed to load asset: {0}")]
    AssetLoading(String),

    #[error("Failed to render text: {0}")]
    TextRendering(String),

    #[error("IO error: {0}")]
    Io(String),

    #[error("No image selected")]
    NoImageSelected,

    #[error("Invalid configuration: {0}")]
    ConfigLoading(String),

    #[error("Invalid image format")]
    InvalidImageFormat,
}

#[repr(i32)]
pub enum CliExitCode {
    Success = 0,
    GenericError = 1,
    InvalidInput = 2,
    AssetLoadFailure = 3,
    ImageProcessingFailure = 4,
    IoError = 5,
    ConfigError = 6,
}

impl LabelError {
    pub fn to_exit_code(&self) -> CliExitCode {
        match self {
            LabelError::ImageLoading(_)
            | LabelError::ImageProcessing(_)
            | LabelError::ImageSaving(_) => CliExitCode::ImageProcessingFailure,
            LabelError::AssetLoading(_) => CliExitCode::AssetLoadFailure,
            LabelError::TextRendering(_) => CliExitCode::GenericError,
            LabelError::Io(_) => CliExitCode::IoError,
            LabelError::NoImageSelected => CliExitCode::InvalidInput,
            LabelError::ConfigLoading(_) => CliExitCode::ConfigError,
            LabelError::InvalidImageFormat => CliExitCode::InvalidInput,
        }
    }
}