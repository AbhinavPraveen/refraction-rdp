use std::{process::{Command,ExitStatus}, sync::mpsc::Receiver};
use libc;

pub fn create_wg(name: &str) -> ExitStatus {
    Command::new("ip")
        .args(["link", "add", "dev", name, "type", "wireguard"])
        .spawn().expect("Failed run ip, do you have it installed?")
        .wait().expect("Failed to wait on child, something went very wrong.")
}


pub fn stay_behind_orig_netns(wg_name: &str, rx: Receiver<()>, pid: u32) {
    println!("Thread will wait in original namespace.");
    rx.recv().expect("Channel broken to original namespace thread.");
    println!("Moving wireguard interface {} to netns of pid: {}", wg_name, pid);
    let move_wg_status = move_wg(wg_name, pid);
    if move_wg_status.success() {
        println!("Moved wireguard interface {} to netns of pid: {}.", wg_name, pid);
    } else {
        exitmsg(format!("Failed to move wireguard interface {} to netns of pid: {}.", wg_name, pid), move_wg_status);
    }
}

pub fn unshare_netns() -> i32{
    unsafe { libc::unshare(libc::CLONE_NEWNET) }
}

pub fn get_err() -> i32 {
    //unsafe {libc::__errno_location()}
    std::io::Error::last_os_error().raw_os_error().unwrap()
}
pub fn pause() {
    unsafe { libc::pause(); }
}

pub fn exitmsg(msg : String, s : ExitStatus) {
    if let Some(n) = s.code() {
        panic!("{} {})", msg, n);
    } else {
        panic!("{} No exit code, terminated by a signal?)", msg)
    }
}

pub fn move_wg(name: &str, pid: u32) -> ExitStatus {
    Command::new("ip")
        .args(["link", "set", "dev", name, "netns", &pid.to_string()])
        .spawn().expect("Failed run ip, do you have it installed?")
        .wait().expect("Failed to wait on child, something went very wrong.")
}
