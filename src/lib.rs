use std::process::{Command,ExitStatus};

pub fn create_wg(name: &str) -> ExitStatus {
    Command::new("ip")
        .args(["link", "add", "dev", name, "type", "wireguard"])
        .spawn().expect("Failed run ip, do you have it installed?")
        .wait().expect("Failed to wait on child, something went very wrong.")
}

pub fn exitmsg(msg : String, s : ExitStatus) {
    if let Some(n) = s.code() {
        panic!("{} {})", msg, n);
    } else {
        panic!("{} No exit code, terminated by a signal?)", msg)
    }
}
