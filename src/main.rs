use arboard::Clipboard;

fn main() {
    let mut clipboard = Clipboard::new().unwrap();

    println!("Clipboard text was: {}", clipboard.get_text().unwrap());
}
