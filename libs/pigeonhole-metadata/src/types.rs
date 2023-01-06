#[derive(Copy, Clone, Debug)]
pub struct Degrees(pub f64);

#[derive(Copy, Clone, Debug)]
pub struct Location {
    pub lat: Degrees,
    pub lon: Degrees,
}

#[derive(Copy, Clone, Debug)]
pub enum LongitudeReference {
    East,
    West,
}

#[derive(Copy, Clone, Debug)]
pub enum LatitudeReference {
    North,
    South,
}

impl Degrees {
    pub fn latitude_reference(self, reference: LatitudeReference) -> Self {
        if let LatitudeReference::North = reference {
            self
        } else {
            Self(-self.0)
        }
    }

    pub fn longitude_reference(self, reference: LongitudeReference) -> Self {
        if let LongitudeReference::East = reference {
            self
        } else {
            Self(-self.0)
        }
    }
}
