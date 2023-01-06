use exif::{DateTime, Field, Value};

use crate::{
    error::FieldError,
    types::{Degrees, LatitudeReference, LongitudeReference},
};

pub(crate) trait FieldExt: Sized {
    fn from_field(field: &Field) -> Result<Self, FieldError>;
}

impl FieldExt for u32 {
    fn from_field(field: &Field) -> Result<Self, FieldError> {
        let uint_value = field.value.as_uint()?;
        Ok(uint_value.get(0).unwrap_or(0))
    }
}

impl FieldExt for DateTime {
    fn from_field(field: &Field) -> Result<Self, FieldError> {
        let Value::Ascii(ascii) = &field.value else {
            return Err(FieldError::TypeError)
        };

        if ascii.len() != 1 {
            return Err(FieldError::TypeError);
        }

        Ok(DateTime::from_ascii(&ascii[0])?)
    }
}

impl FieldExt for Degrees {
    fn from_field(field: &Field) -> Result<Self, FieldError> {
        let Value::Rational(rational) = &field.value else {
            return Err(FieldError::TypeError);
        };

        let [degrees, minutes, seconds] = rational[..] else {
            return Err(FieldError::TypeError);
        };

        Ok(Degrees(
            degrees.to_f64() + (minutes.to_f64() / 60.0) + (seconds.to_f64() / 3600.0),
        ))
    }
}

impl FieldExt for LongitudeReference {
    fn from_field(field: &Field) -> Result<Self, FieldError> {
        let Value::Ascii(ascii) = &field.value else {
            return Err(FieldError::TypeError)
        };

        if ascii.len() != 1 {
            return Err(FieldError::TypeError);
        }

        match ascii[0][..] {
            [b'E'] => Ok(LongitudeReference::East),
            [b'W'] => Ok(LongitudeReference::West),
            _ => Err(FieldError::TypeError),
        }
    }
}

impl FieldExt for LatitudeReference {
    fn from_field(field: &Field) -> Result<Self, FieldError> {
        let Value::Ascii(ascii) = &field.value else {
            return Err(FieldError::TypeError)
        };

        if ascii.len() != 1 {
            return Err(FieldError::TypeError);
        }

        match ascii[0][..] {
            [b'N'] => Ok(LatitudeReference::North),
            [b'S'] => Ok(LatitudeReference::South),
            _ => Err(FieldError::TypeError),
        }
    }
}
