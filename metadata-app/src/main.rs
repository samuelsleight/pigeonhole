use std::path::PathBuf;

use anyhow::Context;
use pigeonhole_metadata::{geocoding::Address, image::Image};
use pigeonhole_nominatim::client::Client;
use structopt::{paw, StructOpt};

#[derive(StructOpt)]
struct Args {
    path: PathBuf,
}

#[tokio::main]
async fn run(args: Args) -> anyhow::Result<()> {
    let image = Image::from_file(&args.path)
        .with_context(|| format!("Error parsing {}", args.path.display()))?;

    image.debug_fields();

    let client = Client::new("Image Metadata App");
    let place = image
        .lookup_location(&client)
        .await
        .with_context(|| "Error requesting location")?;

    println!("{}", place.display_name());

    Ok(())
}

#[paw::main]
fn main(args: Args) -> anyhow::Result<()> {
    run(args)
}
