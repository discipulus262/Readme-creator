use std::{fs, io, io::prelude::*};
fn main() {
    let mut name = String::new();
    let mut desc = String::new();
    let mut file = fs::File::create("test.md").expect("failed to create file");
    
    println!("enter the name of your project");
    io::stdin().read_line(&mut name).expect("too bad, you did something wrong");
    println!("enter the description of your project");
    io::stdin().read_line(&mut desc).expect("too bad, you did something wrong");

    let finalmd = format!("#{} {}", name, desc);
    
    println!("writing to test.md: {}", finalmd);
    file.write_all(finalmd.as_bytes()).expect("failed to write to file");
    file.flush().expect("failed to flush file");

}
