#![feature(trait_alias)]
use glob::glob;

const KEY: [u8; 32] = [0x1E, 0xF7, 0x62, 0x16, 0xB7, 0xC1, 0xAC, 0x26, 0x91, 0xCE, 0x90, 0xAF, 0x93, 0xB3, 0x71, 0x0F, 0x85, 0x6C, 0xC5, 0x89, 0xD8, 0x7A, 0x02, 0x84, 0x9E, 0x0F, 0x21, 0x3E, 0xED, 0x3C, 0x86, 0xB5];

const PAK_SEARCH_PATH: &str = "../../tmp/**/*.pak";

fn main() {
    export_pak_files();
}

fn export_pak_files() {
    for entry in glob(PAK_SEARCH_PATH).unwrap() {
        match unpak::Pak::new_any(entry.unwrap(), Some(&KEY)) {
            Ok(pak) => pak
                .entries()
                .iter()
                .for_each(|entry| println!("{entry}")),
            Err(e) => eprintln!("{e}"),
        }
    }
}
