fn main() {
    match std::env::current_dir() {
        Ok(current_directory) => {
            println!("{:?}", current_directory.to_str());
        }
        Err(_) => {}
    }
}
