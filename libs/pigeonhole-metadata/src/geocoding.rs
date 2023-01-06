use async_trait::async_trait;

use crate::types::Location;

pub trait Address {
    fn display_name(&self) -> &str;
}

#[async_trait]
pub trait GeocodingProvider {
    type Error: std::error::Error;
    type Address: Address;

    async fn lookup(&self, location: Location) -> Result<Self::Address, Self::Error>;
}
