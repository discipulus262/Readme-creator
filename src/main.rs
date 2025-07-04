use cursive::traits::*;
use cursive::Cursive;
use cursive::views::{Dialog, TextView, EditView};
use std::{fs, io, io::prelude::*};
fn main() {
    let mut siv = cursive::default();
    let mut name = String::new();
    let mut desc = String::new();
    let mut file = fs::File::create("test.md").expect("failed to create file");
    let mut code = String::new();
    let mut img = String::new();
     siv.add_layer(
        Dialog::new()
            .title("Enter project data")
            // Padding is (left, right, top, bottom)
            .padding_lrtb(1, 1, 1, 0)
            .content(
                EditView::new()
                    // Call `show_popup` when the user presses `Enter`
                    .on_submit(show_popup)
                    // Give the `EditView` a name so we can refer to it later.
                    .with_name("name")
                    // Wrap this in a `ResizedView` with a fixed width.
                    // Do this _after_ `with_name` or the name will point to the
                    // `ResizedView` instead of `EditView`!
                    .fixed_width(20),
            )
            .button("Ok", |s| {
                // This will run the given closure, *ONLY* if a view with the
                // correct type and the given name is found.
                let name = s
                    .call_on_name("name", |view: &mut EditView| {
                        // We can return content from the closure!
                        view.get_content()
                    })
                    .unwrap();

                // Run the next step
                println!("OK")
            }),
    );


    siv.run();
    let finalmd = format!("# {} \n \n{} \n ![Alt text]({})\nhere is some example code: \n``` \n{} \n```", name, desc, img, code);
    
    println!("writing to test.md: \n {}", finalmd);
    file.write_all(finalmd.as_bytes()).expect("failed to write to file");
    file.flush().expect("failed to flush file");

}
fn show_popup(s: &mut Cursive, name: &str) {
    if name.is_empty() {
        // Try again as many times as we need!
        s.add_layer(Dialog::info("Please enter a name!"));
    } else {
        let content = format!("Hello {name}!");
        // Remove the initial popup
        s.pop_layer();
        // And put a new one instead
        s.add_layer(Dialog::around(TextView::new(content)).button("Quit", |s| s.quit()));
    }
}

