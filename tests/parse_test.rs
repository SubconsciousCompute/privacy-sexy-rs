use std::fs;

use glob::glob;
use privacy_sexy;

#[allow(dead_code)]
#[test]
fn test() {
    for fpath in glob("tests/collections/*.yml").unwrap() {
        let fpath = fpath.unwrap();

        assert_eq!(
            privacy_sexy::CollectionData::from_file(&fpath)
                .unwrap()
                .parse(None, false, None)
                .unwrap()
                .trim(),
            fs::read_to_string(fpath.with_extension("txt")).unwrap().trim()
        );
    }
}
