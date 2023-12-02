#![feature(pointer_byte_offsets)]
use glob::glob;
use std::path::PathBuf;
use std::{ffi::c_void, fs, thread};
use winapi::um::libloaderapi::GetModuleHandleA;

#[ctor::ctor]
fn ctor() {
    let mods_dir = create_mods_dirs().expect("Elefrac Mod Loader - failed to create Mods directories");
    println!("mods_dir: {}", mods_dir.display());

    let base_addr = unsafe { GetModuleHandleA(std::ptr::null()) as *const c_void };
    match load_sync_mods(mods_dir.join("dlls"), base_addr) {
        Ok(loaded_sync_mods) => {
            if loaded_sync_mods.len() > 0 {
                println!("Elefrac Mod Loader - Loaded the following synchronous mods:");
                for loaded in loaded_sync_mods {
                    println!("\t\t{}", loaded);
                }
            }
        }

        Err(err) => {
            println!("Elefrac Mod Loader - failed to load synchronous mods: {:?}", err);
        }
    }

    thread::spawn(move || {
        let base_addr = unsafe { GetModuleHandleA(std::ptr::null()) as *const c_void };
        
        match load_mods(mods_dir.join("dlls"), base_addr) {
            Ok(loaded_mods) => {
                if loaded_mods.len() == 0 {
                    println!("Elefrac Mod Loader - No mods to load!");
                } else {
                    println!("Elefrac Mod Loader - Loaded the following mods:");
                    for loaded in loaded_mods {
                        println!("\t\t{}", loaded);
                    }
                }
            }

            Err(err) => {
                println!("Elefrac Mod Loader: Failed to load mods! {:?}", err);
            }
        }
    });
}

fn create_mods_dirs() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut dir = std::env::current_exe()?;
    dir.pop();
    dir.pop();
    dir.pop();
    dir.pop();
    dir.push("Mods");

    fs::create_dir_all(dir.join("dlls"))?;
    fs::create_dir_all(dir.join("packages"))?;
    fs::create_dir_all(dir.join("contents"))?;
    fs::create_dir_all(dir.join("commands"))?;
    fs::create_dir_all(dir.join("servers"))?;
    fs::create_dir_all(dir.join("scripts"))?;

    Ok(dir)
}
fn load_sync_mods(dll_dir: PathBuf, base_addr: *const c_void) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut loaded_mods: Vec<String> = vec![];
    for entry in glob(format!("{}/**/*.dll", dll_dir.display()).as_str())? {
        unsafe {
            let path = entry?;
            let lib = libloading::Library::new(path.clone())?;
            let mod_main: Result<libloading::Symbol<unsafe extern fn(*const c_void) -> u32>, libloading::Error> = lib.get(b"mod_main_sync");

            if mod_main.is_ok() {
                let main_fn = mod_main.unwrap();
                let did_load = std::panic::catch_unwind(|| {
                    main_fn(base_addr);
                });

                if did_load.is_ok() { 
                    loaded_mods.push(path.display().to_string());
                }
            }
        }
    }

    Ok(loaded_mods)
}

fn load_mods(dll_dir: PathBuf, base_addr: *const c_void) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut loaded_mods: Vec<String> = vec![];
    for entry in glob(format!("{}/**/*.dll", dll_dir.display()).as_str())? {
        unsafe {
            let path = entry?;
            let lib = libloading::Library::new(path.clone())?;
            let mod_main: Result<libloading::Symbol<unsafe extern fn(*const c_void) -> u32>, libloading::Error> = lib.get(b"mod_main");

            if mod_main.is_ok() {
                let main_fn = mod_main.unwrap();
                let did_load = std::panic::catch_unwind(|| {
                    main_fn(base_addr);
                });

                if did_load.is_ok() { 
                    loaded_mods.push(path.display().to_string());
                }
            }
        }
    }

    Ok(loaded_mods)
}

// pub fn do_a_thing() {
//     let game_base = GameBase::singleton();
//     let g_objects = game_base.gobjects();

//     for i in 0..(g_objects.num_elements.to_native()-1) {
//         let item = g_objects.item_at_idx(i as usize);
//         let object = if item.is_some() { item.unwrap().object::<UObject>() } else { None };

//         if object.is_some() &&  game_base.get_offset_from_addr(object.unwrap().virtual_funcs()) == 0x5ab1450 {
//             let obj: &UObject = object.unwrap();
//             debug!("GOBJECTS[{:?}]: {:?} ({:?})", i, obj.full_name(), obj.class().full_name());
//         }
//     }
// }