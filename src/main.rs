fn main() {
    match privacy_sexy::get_collection(privacy_sexy::OS::Windows) {
        Ok(cd) => match cd.parse(false) {
            Ok(cd_parsed) => {
                println!("{}", cd_parsed);
                privacy_sexy::run_script(&cd.parse(false).unwrap(), cd.scripting.file_extension, cd.os).unwrap();
            }
            Err(e) => eprintln!("{:?}", e),
        },
        Err(e) => eprintln!("{:?}", e),
    }
}
