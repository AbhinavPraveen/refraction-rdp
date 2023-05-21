use std::{process::{Command,ExitStatus,id}, sync::mpsc::Receiver, os::unix::{net::{UnixListener,UnixStream},process::CommandExt}, io::{Write,Read}};
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
    let move_wg_status = move_wg(wg_name, &pid.to_string());
    if move_wg_status.success() {
        println!("Moved wireguard interface {} to netns of pid: {}.", wg_name, pid);
    } else {
        exitmsg(format!("Failed to move wireguard interface {} to netns of pid: {}.", wg_name, pid.to_string()), move_wg_status);
    }
}


pub fn exec_sunshine() {
    let err = Command::new("sunshine")
        .env("PULSE_SERVER", sunshine_path())
        .exec();
    panic!("{}", err)
}

pub fn exec_moonlight() {
    let err = Command::new("flatpak")
        .args(["run", "com.moonlight_stream.Moonlight"])
        .env("PULSE_SERVER", sunshine_path())
        .exec();
    panic!("{}", err)
}

pub fn sunshine_path() -> String {
    format!("unix:/run/user/{}/pulse/native", unsafe {libc::getuid()})
}

pub fn unshare_netns() -> i32 {
    unsafe { libc::unshare(libc::CLONE_NEWNET) }
}

pub fn unshare_user_netns() -> i32 {
    unsafe { libc::unshare(libc::CLONE_NEWUSER | libc::CLONE_NEWNET) }
}

pub fn get_err() -> i32 {
    std::io::Error::last_os_error().raw_os_error().unwrap()
}
pub fn pause() {
    unsafe { libc::pause(); }
}

pub fn create_sock(path: &str) -> std::io::Result<UnixListener> {
    UnixListener::bind(path)
}

pub fn exitmsg(msg : String, s : ExitStatus) {
    if let Some(n) = s.code() {
        panic!("{} {})", msg, n);
    } else {
        panic!("{} No exit code, terminated by a signal?)", msg)
    }
}

pub fn move_wg(name: &str, pid: &str) -> ExitStatus {
    Command::new("ip")
        .args(["link", "set", "dev", name, "netns", pid])
        .spawn().expect("Failed run ip, do you have it installed?")
        .wait().expect("Failed to wait on child, something went very wrong.")
}

pub fn get_wireguard() {
    let sock_name = "/tmp/refraction-rdp.sock";
    {
        let s = unshare_user_netns();
        if s == 0 {
            println!("Moved main thread to new netns");
        } else {
            panic!("Failed to unshare, err: {}", get_err());
        }
    }

    {
        println!("Making request for wireguard interface on {}", sock_name);
        let mut req_stream = UnixStream::connect(sock_name)
            .expect(format!("Failed to connect to {} has the privileged service been started and do you have permission to connect to the socket?", sock_name).as_str());

        writeln!(req_stream, "{}", id())
            .expect(format!("Failed to write to {} - something went wrong.", sock_name).as_str());
        //  .write(id().to_string().as_bytes())
        //    .expect(format!("Failed to write to {} - something went wrong.", sock_name).as_str());
        println!("Made request for wireguard interface on {}", sock_name);

        let mut resp = String::new();
        req_stream.read_to_string(&mut resp).expect("Failed to read respone.");
        if resp == "Done" {
            println!("Request was completed.")
        } else {
            panic!("Server did not provide the expect response: {}", resp);
        }
    }
}
