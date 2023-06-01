use libc;
use std::{
    io::{Read, Write},
    os::unix::{
        net::{UnixListener, UnixStream},
        process::CommandExt,
    },
    process::{id, Command, ExitStatus},
    sync::mpsc::Receiver,
};

pub fn create_wg(name: &str) -> ExitStatus {
    Command::new("ip")
        .args(["link", "add", "dev", name, "type", "wireguard"])
        .spawn()
        .expect("Failed run ip, do you have it installed?")
        .wait()
        .expect("Failed to wait on child, something went very wrong.")
}

pub fn stay_behind_orig_netns(wg_name: &str, rx: Receiver<()>, pid: u32) {
    println!("Thread will wait in original namespace.");
    rx.recv()
        .expect("Channel broken to original namespace thread.");
    println!(
        "Moving wireguard interface {} to netns of pid: {}",
        wg_name, pid
    );
    let move_wg_status = move_wg(wg_name, &pid.to_string());
    if move_wg_status.success() {
        println!(
            "Moved wireguard interface {} to netns of pid: {}.",
            wg_name, pid
        );
    } else {
        exitmsg(
            format!(
                "Failed to move wireguard interface {} to netns of pid: {}.",
                wg_name,
                pid.to_string()
            ),
            move_wg_status,
        );
    }
}

pub fn exec_sunshine(pulse_path: &String) {
    let err = Command::new("sunshine")
        .env("PULSE_SERVER", pulse_path)
        .exec();
    panic!("{}", err)
}

pub fn exec_moonlight(pulse_path: &String) {
    let err = Command::new("moonlight")
        .env("PULSE_SERVER", pulse_path)
        .exec();
    panic!("{}", err)
}

pub fn pulse_path() -> String {
    format!("unix:/run/user/{}/pulse/native", unsafe { libc::getuid() })
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
    unsafe {
        libc::pause();
    }
}

pub fn create_sock(path: &str) -> std::io::Result<UnixListener> {
    UnixListener::bind(path)
}

pub fn exitmsg(msg: String, s: ExitStatus) {
    if let Some(n) = s.code() {
        panic!("{} {})", msg, n);
    } else {
        panic!("{} No exit code, terminated by a signal?)", msg)
    }
}

pub fn setns(pid: &str) -> i32 {
    let pid: i32 = pid.parse().expect("Failed to parse pid.");
    unsafe {
        let pidfd = libc::syscall(libc::SYS_pidfd_open, pid, 0);
        libc::setns(pidfd as i32, libc::CLONE_NEWNET)
    }
}

pub fn move_wg(name: &str, pid: &str) -> ExitStatus {
    Command::new("ip")
        .args(["link", "set", "dev", name, "netns", pid])
        .spawn()
        .expect("Failed run ip, do you have it installed?")
        .wait()
        .expect("Failed to wait on child, something went very wrong.")
}

pub fn wg_conf_wg(name: &str, wgconf: &str) -> ExitStatus {
    Command::new("wg")
        .args(["setconf", name, wgconf])
        .spawn()
        .expect("Failed to run wg, do you have it installed?")
        .wait()
        .expect(
            format!(
                "Failed to configure {} using wg, does {} exist?",
                name, wgconf
            )
            .as_str(),
        )
}

pub fn netns_wg_addr(name: &str, addr: &str) -> ExitStatus {
    Command::new("ip")
        .args(["addr", "add", "dev", name, addr])
        .spawn()
        .expect("Failed run ip, do you have it installed?")
        .wait()
        .expect("Failed to wait on child, something went very wrong.")
}

pub fn netns_lo_up() -> ExitStatus {
    Command::new("ip")
        .args(["link", "set", "dev", "lo", "up"])
        .spawn()
        .expect("Failed run ip, do you have it installed?")
        .wait()
        .expect("Failed to wait on child, something went very wrong.")
}

pub fn netns_wg_up(name: &str) -> ExitStatus {
    Command::new("ip")
        .args(["link", "set", "dev", name, "up"])
        .spawn()
        .expect("Failed run ip, do you have it installed?")
        .wait()
        .expect("Failed to wait on child, something went very wrong.")
}

pub fn get_wireguard(req_type: char) {
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

        println!("Sending req: {}{}", id(), req_type);
        writeln!(req_stream, "{}{}", id(), req_type)
            .expect(format!("Failed to write to {} - something went wrong.", sock_name).as_str());
        //  .write(id().to_string().as_bytes())
        //    .expect(format!("Failed to write to {} - something went wrong.", sock_name).as_str());
        println!("Made request for wireguard interface on {}", sock_name);

        let mut resp = String::new();
        req_stream
            .read_to_string(&mut resp)
            .expect("Failed to read respone.");
        if resp == "Done" {
            println!("Request was completed.")
        } else {
            panic!("Server did not provide the expect response: {}", resp);
        }
    }
}

pub fn netns_conf_ip(wg_name: &str, addr: &str, pid: &str) {
    println!("Begining netns configuration.");
    let s = setns(pid);
    if s == 0 {
        println!("Moved to namespace of pid: {}", pid);
    } else {
        panic!(
            "Failed to setns to that of pid: {}, err: {}",
            pid,
            get_err()
        );
    }

    let netns_wg_addr_status = netns_wg_addr(wg_name, addr);
    if netns_wg_addr_status.success() {
        println!("Set {} addr to {}", wg_name, addr);
    } else {
        exitmsg(
            format!("Failed to set {} addr to {}", wg_name, addr),
            netns_wg_addr_status,
        );
    }

    let netns_lo_up_status = netns_lo_up();
    if netns_lo_up_status.success() {
        println!("Set lo status to up.");
    } else {
        exitmsg("Failed to set lo up.".to_string(), netns_lo_up_status);
    }

    let netns_wg_up_status = netns_wg_up(wg_name);
    if netns_wg_up_status.success() {
        println!("Set {} status to up", wg_name);
    } else {
        exitmsg(
            format!("Failed to set {} status to up", wg_name),
            netns_wg_up_status,
        );
    }
    println!("Completed netns configuration.")
}
