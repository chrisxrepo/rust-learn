mod errors;

fn main() {
    if let Err(e) = read_file() {
        println!("{}", e);
    }
}

fn read_file() -> Result<(), errors::Errors> {
    // let c = std::fs::read_to_string("hello.txt").map_err(|e| io_error!(e))?;
    // println!("file:{}", c);

    Err(custom_error!("hello wrold".into()))
}
