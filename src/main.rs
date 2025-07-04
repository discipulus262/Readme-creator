use std::{fs, io, io::prelude::*};
fn main() {
    let mut name = String::new();
    let mut desc = String::new();
    let mut file = fs::File::create("test.md").expect("failed to create file");
    let mut code = String::new();
    let mut img = String::new();

    let finalmd = format!(
        "# {} \n \n{} \n ![Alt text]({})\nhere is some example code: \n``` \n{} \n```",
        name, desc, img, code
    );

    println!("writing to test.md: \n {}", finalmd);
    file.write_all(finalmd.as_bytes())
        .expect("failed to write to file");
    file.flush().expect("failed to flush file");
}
