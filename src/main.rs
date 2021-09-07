#[cfg(windows)]
extern crate winapi;
use std::io::Error;
use clap::{App, load_yaml};

const MSGBOX_ERROR:i32 = 0;

enum MsgBoxIcon{
    INFO,
    STOP,
    QUESTION,
    WARNING
}

//&str -> win32 lpstr, u8 ->  u16 -> terminate string with '\0'
#[cfg(windows)]
fn w32string(s: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;

    std::ffi::OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

/*
    Parse and sets the message box icon type
 */
#[cfg(windows)]
fn get_icon_type(icon: &str) -> MsgBoxIcon{
    if let Ok(icon_type) = icon.parse(){
        match icon_type{
            1 => return MsgBoxIcon::INFO,
            2 => return MsgBoxIcon::STOP,
            3 => return MsgBoxIcon::QUESTION,
            4 => return MsgBoxIcon::WARNING,
            _ => return MsgBoxIcon::INFO
        };
    }else{
        return MsgBoxIcon::INFO;
    }
}

/*
    Win32: Message Box function
 */
#[cfg(windows)]
fn message_box(msg: &str, caption: &str, icon: &str) -> Result<i32, Error> {
    //raw pointer
    use std::ptr::null_mut;
    //check MSDN: https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messagebox
    use winapi::um::winuser::{
    MessageBoxW, 
    MB_ICONINFORMATION,
    MB_ICONWARNING,
    MB_ICONSTOP,
    MB_ICONQUESTION,
    MB_OK};

    let w32_txt = w32string(msg);
    let w32_caption = w32string(caption);

    //Use MessageBox win32 function
    let msg_box_result = unsafe {
        //Translates the MsgBoxType to Win32 Messagebox Flags
        let icon_type = match get_icon_type(icon){
            MsgBoxIcon::INFO => MB_ICONINFORMATION,
            MsgBoxIcon::STOP => MB_ICONSTOP,
            MsgBoxIcon::QUESTION => MB_ICONQUESTION,
            MsgBoxIcon::WARNING => MB_ICONWARNING,
        };

        MessageBoxW(
            null_mut(),
            w32_txt.as_ptr(),
            w32_caption.as_ptr(),
            MB_OK | icon_type, // | = piping flags
        )
    };

    if msg_box_result == MSGBOX_ERROR{
        Err(Error::last_os_error())
    }else{
        Ok(msg_box_result)
    }
}


#[cfg(windows)]
fn main() {
    let yaml = load_yaml!("../cmd.yaml");
    let matches = App::from(yaml).get_matches();
    let mut message: &str ="";
    let mut caption: &str ="";
    let mut box_icon: &str ="";

    if let Some(msg) = matches.value_of("MSG"){
        message = msg;
    };

    if let Some(cap) = matches.value_of("CAPTION"){
        caption = cap;
    };

    if let Some(icon) = matches.value_of("ICON"){
        box_icon = icon;
    };

    if let Ok(_) = message_box(message, caption, box_icon){
        println!("Messagebox shown successfully!");
    }
}

#[cfg(not(windows))]
println!("This program only works on Windows!");