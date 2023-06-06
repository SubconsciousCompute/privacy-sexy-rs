use privacy_sexy::{get_collection, OS::Windows};

fn main() {
    // Get CollectionData for Windows
    let coll = get_collection(Windows).unwrap();

    // Parse CollectionData to string
    let script = coll.parse(None, false, None).unwrap();

    // Print script
    println!("{script}");
}
