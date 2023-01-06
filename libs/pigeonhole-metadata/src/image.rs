use std::{fs::File, io::BufReader, path::Path};

use exif::{DateTime, Exif, Field, In, Reader, Tag};

use crate::{
    error::{FieldError, GeocodingError, ImageOpenError},
    field::FieldExt,
    geocoding::GeocodingProvider,
    types::{Degrees, LatitudeReference, Location, LongitudeReference},
};

pub struct Image {
    exif: Exif,
}

macro_rules! define_fields {
    ($($name:ident($tag:expr): $ty:ty)+) => {
        $(pub fn $name(&self) -> Result<$ty, FieldError> {
            self.get_field($tag)
                .ok_or(FieldError::FieldMissing)
                .and_then(|field| <$ty as FieldExt>::from_field(field))
        })+
    };
}

impl Image {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ImageOpenError> {
        let file = File::open(path)?;
        let mut file_reader = BufReader::new(file);

        let exif_reader = Reader::new();
        let exif = exif_reader.read_from_container(&mut file_reader)?;

        Ok(Self { exif })
    }

    pub fn debug_fields(&self) {
        for field in self.exif.fields() {
            println!(
                "{}: {:?}, {}",
                field.tag,
                field.value,
                field.display_value()
            );
        }
    }

    pub fn get_field(&self, tag: Tag) -> Option<&Field> {
        self.exif.get_field(tag, In::PRIMARY)
    }

    pub fn get_location(&self) -> Result<Location, FieldError> {
        Ok(Location {
            lat: self
                .get_latitude()?
                .latitude_reference(self.get_latitude_ref()?),

            lon: self
                .get_longitude()?
                .longitude_reference(self.get_longitude_ref()?),
        })
    }

    pub async fn lookup_location<Client: GeocodingProvider>(
        &self,
        client: &Client,
    ) -> Result<Client::Address, GeocodingError<Client::Error>> {
        client
            .lookup(self.get_location()?)
            .await
            .map_err(GeocodingError::LookupError)
    }

    define_fields! {
        get_width(Tag::ImageWidth): u32
        get_length(Tag::ImageLength): u32
        get_date_time(Tag::DateTime): DateTime
        get_latitude(Tag::GPSLatitude): Degrees
        get_latitude_ref(Tag::GPSLatitudeRef): LatitudeReference
        get_longitude(Tag::GPSLongitude): Degrees
        get_longitude_ref(Tag::GPSLongitudeRef): LongitudeReference
    }
}
