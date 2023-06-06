use privacy_sexy::collection::{CollectionData, CollectionError};

fn main() -> Result<(), CollectionError> {
    let filename = "collections/windows.yaml";

    // Get CollectionData from filename
    let coll = CollectionData::from_file(filename)?;

    // Display Collection
    println!("{:#?}", coll);

    Ok(())
}
