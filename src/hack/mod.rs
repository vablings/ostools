use hook::prehook;
use log::{info, warn};
use std::{ptr::null_mut, time::Duration};
use windows_sys::Win32::{
    System::LibraryLoader::{FreeLibraryAndExitThread, GetModuleHandleA},
    UI::{Input::KeyboardAndMouse::{GetAsyncKeyState, VK_DELETE}, WindowsAndMessaging::{UnhookWindowsHookEx, HHOOK}},
};

pub fn start() {
    let base = get_module_handle(null_mut());
    info!("[+] Base module: {:#x}", base);

    unsafe { 
        let mouse_hook: *mut HHOOK = std::mem::transmute(base + 0x29ca0c8);
        UnhookWindowsHookEx(*(mouse_hook));
        info!("[+] Unhooked mouse hook HHK @ {:?}", mouse_hook);
    }

    loop {
        unsafe {
            match GetAsyncKeyState(VK_DELETE as i32) {
                0 => (),
                _ => unload(),
            };
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

pub fn unload() {
    unsafe {
 
        warn!("[+] Unloading module ");
        FreeLibraryAndExitThread(get_module_handle(b"ostools\0".as_ptr() as _) as _, 0)
    };
}

fn get_module_handle(name: *const u8) -> isize {
    unsafe { GetModuleHandleA(name as _) }
}
