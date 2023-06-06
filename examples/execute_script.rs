use privacy_sexy::{get_collection, run_script, OS::Windows};

fn main() {
    // Get CollectionData for Windows
    let coll = get_collection(Windows).unwrap();

    // Parse CollectionData to string
    let script = coll.parse(None, false, None).unwrap();

    // Execute script
    run_script(&script, coll.scripting.file_extension).unwrap();
}
