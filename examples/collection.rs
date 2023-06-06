use privacy_sexy::{get_collection, OS::Windows};

fn main() {
    // Get CollectionData for Windows
    let coll = get_collection(Windows).unwrap();

    // Display Collection
    println!("{:#?}", coll);
}
