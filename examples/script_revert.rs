use privacy_sexy::{get_collection, OS::MacOs};

fn main() {
    // Get CollectionData for MacOs
    let coll = get_collection(MacOs).unwrap();

    // Get revert script
    let script = coll.parse(None, true, None).unwrap();

    // Print script
    println!("{script}");
}
