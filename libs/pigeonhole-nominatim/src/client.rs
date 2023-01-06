use async_trait::async_trait;
use nominatim::{Client as Nominatim, IdentificationMethod, Place, Status};
use pigeonhole_metadata::{
    geocoding::{Address, GeocodingProvider},
    types::Location,
};
use reqwest::Error;

pub struct Client {
    client: Nominatim,
}

pub struct Addr(Place);

impl Client {
    pub fn new<S: AsRef<str>>(user_agent: S) -> Self {
        Self {
            client: Nominatim::new(IdentificationMethod::from_user_agent(user_agent)),
        }
    }

    pub async fn status(&self) -> Result<Status, Error> {
        self.client.status().await
    }

    pub async fn lookup(&self, location: Location) -> Result<Place, Error> {
        self.client
            .reverse(
                format!("{}", location.lat.0),
                format!("{}", location.lon.0),
                None,
            )
            .await
    }
}

#[async_trait]
impl GeocodingProvider for Client {
    type Error = reqwest::Error;
    type Address = Addr;

    async fn lookup(&self, location: Location) -> Result<Self::Address, Self::Error> {
        Client::lookup(self, location).await.map(Addr)
    }
}

impl Address for Addr {
    fn display_name(&self) -> &str {
        &self.0.display_name
    }
}
