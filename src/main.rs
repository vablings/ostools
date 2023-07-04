use log::info;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_DELETE};
use std::{
    io::{Read, Write},
    net::TcpListener,
};
use tracing::metadata::LevelFilter;
use dll_syringe::{Syringe, process::OwnedProcess};

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .init();

    let listener = TcpListener::bind("127.0.0.1:7331")?;

    info!("Starting debug console...");

    let proc = OwnedProcess::find_first_by_name("osclient").unwrap();
    let syringe = Syringe::for_process(proc);
    let injected_payload = syringe.inject("./target/debug/ostools.dll").unwrap();


    let (mut stream, address) = listener.accept()?;
    info!("{address} has connected");
    let mut buf = vec![0u8; 1024];
    let mut stdout = std::io::stdout();
    while let Ok(n) = stream.read(&mut buf[..]) {
        stdout.write_all(&buf[..n])?;
        unsafe { match GetAsyncKeyState(VK_DELETE as i32) {
            0 => (),
            _ => std::process::exit(0),
        };}
    }
    Ok(())
}
