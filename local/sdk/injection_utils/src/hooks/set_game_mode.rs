use retour::static_detour;
use ue_types::*;
use game_base::GameBase;
use utils::debug;
use simple_endian::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
struct StringArg {
    pub len: u32le,
    _padding: [u8; 0x4],
    pub s: *const u8
}

impl<'a> std::string::ToString for StringArg {
    fn to_string (&self) -> String {
        let mut bytes: Vec<u8> = vec![];
        for i in 0..self.len.into() {
            unsafe { bytes.push(*self.s.byte_offset(i as isize)) };
        }

        String::from_utf8(bytes).expect("Found invalid UTF-8")
    }
}


static_detour! {
    static FnIntercept: fn(*const UGameInstance, StringArg, StringArg);
}

pub fn intercept() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let hooked: fn(*const UGameInstance, StringArg, StringArg) = std::mem::transmute(GameBase::singleton().at_offset(0x39910C0));
        println!("HOOKED FUNC {:p}", hooked);

        FnIntercept.initialize(hooked, do_intercept)?;
        FnIntercept.enable()?;
    }

    Ok(())
}

fn do_intercept(this: *const UGameInstance, mode: StringArg, variant: StringArg) {
    debug!("INTERCEPTED! {:p} - {:?} - {:?}", this, mode.to_string(), variant.to_string());
    FnIntercept.call(this, mode, variant);
}