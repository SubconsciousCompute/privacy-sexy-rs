fn main() {
    match privacy_sexy::get_collection(privacy_sexy::OS::Windows) {
        Ok(cd) => match cd.parse(false) {
            Ok(cd_parsed) => println!("{}", cd_parsed),
            Err(e) => eprintln!("{:?}", e),
        },
        Err(e) => eprintln!("{:?}", e),
    }
}
