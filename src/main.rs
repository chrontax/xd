use std::{env::args, fs::read};

fn main() {
    let args = args().collect::<Vec<_>>();
    if args.len() != 3 {
        panic!("no you don't");
    }
    let name = args[2].replace(|c: char| !c.is_ascii_alphanumeric(), "_");
    print!("const char {}[] = {{", name);
    read(&args[1])
        .expect("couldn't open file")
        .iter()
        .for_each(|b| print!("0x{:02x},", b));
    println!("}};");
    println!("const int {}_len = sizeof({});", name, name);
}
