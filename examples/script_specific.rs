use privacy_sexy::{collection::Recommend::Strict, get_collection, OS::MacOs};

fn main() {
    // Get CollectionData for MacOs
    let coll = get_collection(MacOs).unwrap();

    // Get only Strict script
    let names = vec![
        "Clear terminal history",
        "Clear browser history",
        "Clear DNS cache",
        "Disable Spotlight indexing",
    ];
    let script = coll.parse(Some(&names), false, Some(Strict)).unwrap();

    // Print script
    println!("{script}");
}
