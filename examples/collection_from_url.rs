use privacy_sexy::collection::{CollectionData, CollectionError};

fn main() -> Result<(), CollectionError> {
    let url = "https://raw.githubusercontent.com/SubconsciousCompute/privacy-sexy-rs/master/collections/macos.yaml";

    // Get CollectionData from url
    let coll = CollectionData::from_url(url)?;

    // Display Collection
    println!("{:#?}", coll);

    Ok(())
}
