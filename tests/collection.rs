use privacy_sexy::collection::CollectionData;

#[test]
fn from_file_test() {
    assert!(CollectionData::from_file("collections/macos.yaml").is_ok());
}

#[test]
fn from_url_test() {
    assert!(CollectionData::from_url(
        "https://raw.githubusercontent.com/SubconsciousCompute/privacy-sexy-rs/master/collections/macos.yaml"
    )
    .is_ok());
}
