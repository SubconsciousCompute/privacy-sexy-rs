use std::{fs::File, io::Read};

use privacy_sexy::collection::CollectionData;

fn main() -> Result<(), serde_yaml::Error> {
    let filename = "collections/macos.yaml";

    // Open file
    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            // Read content from file
            file.read_to_string(&mut content).unwrap();

            // Deserialize content into CollectionData
            let coll: CollectionData = serde_yaml::from_str(&content)?;

            // Display Collection
            println!("{:#?}", coll);
        }
        Err(error) => {
            // Print Error
            println!("Error opening file {}: {}", filename, error);
        }
    }

    Ok(())
}
