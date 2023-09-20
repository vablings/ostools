#![cfg(windows)]
#![feature(abi_thiscall)]
use std::{net::TcpStream, panic};
use std::sync::Mutex;

use log::error;
use windows_sys::Win32::System::SystemServices::{
    DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH,
};

use u32 as DWORD;
type LPVOID = *mut core::ffi::c_void;

mod hack;

unsafe extern "system" fn dll_main(_module: usize) -> u32 {
    let stream = TcpStream::connect("127.0.0.1:7331").unwrap();

    tracing_subscriber::fmt()
        .with_writer(Mutex::new(stream))
        .init();

    let res = std::panic::catch_unwind(|| {
        hack::start();
    });

    match res {
        Err(e) => {
            error!("Error: {:?}", e);
            hack::unload()
        },
        _ => {}
    };
    0
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub unsafe extern "system" fn DllMain(module: usize, reason: DWORD, _: LPVOID) -> u8 {
    match reason {
        DLL_PROCESS_ATTACH => {
            std::thread::spawn(move || dll_main(module));
            1
        }
        DLL_PROCESS_DETACH | DLL_THREAD_ATTACH | DLL_THREAD_DETACH => 1,
        _ => 0,
    }
}

