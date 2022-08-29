use std::fs::File;
use std::io::Read;

use privacy_sexy::CollectionData;

fn main() -> Result<(), serde_yaml::Error> {
    let filename = "collections/windows.yaml";

    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();

            let deser: CollectionData = serde_yaml::from_str(&content)?;
            println!("{:#?}", deser);
            Ok(())
        }
        Err(error) => {
            println!("There is an error {}: {}", filename, error);
            Ok(())
        }
    }
}
