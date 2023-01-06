use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImageOpenError {
    #[error("Error reading file")]
    IoError(#[from] io::Error),

    #[error("Error parsing metadata")]
    ExifError(#[from] exif::Error),
}

#[derive(Error, Debug)]
pub enum FieldError {
    #[error("Field does not exist")]
    FieldMissing,

    #[error("Field was incorrect type")]
    TypeError,

    #[error("Error parsing field")]
    ExifError(#[from] exif::Error),
}

#[derive(Error, Debug)]
pub enum GeocodingError<LookupError: std::error::Error> {
    #[error("Error reading field")]
    FieldError(#[from] FieldError),

    #[error("Error making request")]
    LookupError(LookupError),
}
