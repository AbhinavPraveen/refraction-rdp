use refraction_rdp::*;
use std::{process::id,os::unix::net::UnixStream, io::Write};

fn main() {
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
        UnixStream::connect(sock_name)
            .expect(format!("Failed to connect to {} has the privileged service been started and do you have permission to connect to the socket?", sock_name).as_str())
            .write(id().to_string().as_bytes())
            .expect(format!("Failed to write to {} - something went wrong.", sock_name).as_str());
        println!("Made request for wireguard interface on {}", sock_name);
    }
    
    pause()
}
