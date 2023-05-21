use refraction_rdp::*;
use std::io::Read;

fn main() {
    let wg_name = "wg-refraction";
    let sock_path = "/tmp/refraction-rdp.sock";

    let req_sock = create_sock(sock_path).expect(format!("Failed to create Unix Socket at {}", sock_path).as_str());
    println!("Created request socket at {}", sock_path);

    loop {
        {
            //This handling can be changed after rust-lang/rust #42839
            let mut pid = String::new();

            let mut req_stream = req_sock.accept()
                .expect(format!("Failed to accept stream at {}", sock_path).as_str())
                .0;
            req_stream.read_to_string(&mut pid).expect("Failed to get data from request stream.");
            println!("Received request to provide a wireguard interface to pid: {}", pid);
            

            println!("Creating wireguard interface.");
            {
                let create_wg_status = create_wg(&wg_name);
                if create_wg_status.success() {
                    println!("Created wireguard interace {}", wg_name);
                } else {
                    exitmsg(format!("Failed to create wireguard interface {} (ip:", wg_name), create_wg_status);
                }

            }

            {
                let move_wg_status = move_wg(&wg_name, &pid);
                if move_wg_status.success() {
                    println!("Moved wireguard interface {} to netns of pid: {}.", wg_name, pid);
                } else {
                    exitmsg(format!("Failed to move wireguard interface {} to netns of pid: {}.", wg_name, &pid.to_string()), move_wg_status);
                }
            }

        }
    }
}
