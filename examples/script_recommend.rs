use privacy_sexy::{collection::Recommend, get_collection, OS::Windows};

fn main() {
    // Get CollectionData for Windows
    let coll = get_collection(Windows).unwrap();

    // Get Strict script
    let script = coll.parse(None, false, Some(Recommend::Strict)).unwrap();

    // Print script
    println!("{script}");
}
