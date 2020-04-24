extern crate libc;
extern crate termios;
extern crate base64;
extern crate rpassword;
extern crate serde;
extern crate aes_gcm;
extern crate aead;
extern crate rand;
extern crate toml;
extern crate md5;
extern crate ureq;

mod struct_;
mod gist;
mod editor;

use std::{fs, io, env}; 
pub use editor::{RawMode, Row, Editor};


fn main() -> io::Result<()> {
    let config_file = fs::read_to_string("Config.toml")
        .expect("something is wrong with your Config.toml");
    let args: Vec<String> = env::args().collect();

    let is_init: bool = args.contains(&"initialize".to_string());
    if is_init {
        println!("initializing... note: this will delete everything in your gist");
    }

    let config: struct_::Config = toml::from_str(&config_file).unwrap();
    let temp: String = rpassword::prompt_password_stdout("Password (¬‿¬): ").unwrap();
    let pwd: String = format!("{:?}", md5::compute(temp.as_bytes()));

    if is_init {
        let nonce: String = gist::unique_gen();
        let content: String = gist::encrypt(&pwd, &nonce, &"".to_string());
        gist::update_gist(&config, &nonce, &content);
    }

    let values = gist::get_gist(&config);
    let stt = gist::decrypt(&pwd, &values[2], &values[3]);

    let mut editor = Editor::new()?;
    editor.set_password(&pwd, &values[0]);
    editor.open(&stt)?;
    editor.set_status_message(format!("(づ｡◕‿‿◕｡)づ (HELP: Ctrl-S/Ctrl-Q/Ctrl-F/Ctrl-P) [{}]", &values[0]));
    editor.refresh_screen();
    while editor.process_keypress() {
        editor.refresh_screen();
    }
    Ok(())
}
