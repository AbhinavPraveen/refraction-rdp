use refraction_rdp::*;
use std::{process::id,os::unix::net::UnixStream, io::{Read,Write}};

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
    
    pause()
}
