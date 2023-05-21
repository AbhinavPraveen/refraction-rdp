use refraction_rdp::*;
use std::{
    io::{BufRead, Write},
    thread,
};

fn main() {
    let wg_name = "wg-refraction";
    let wg_conf = "/etc/wireguard/wg-refraction.conf";
    let wg_serv_addr = "10.10.10.1/24";
    let wg_cli_addr = "10.10.10.2/24";
    let mut wg_addr = "";
    let sock_path = "/tmp/refraction-rdp.sock";

    let req_sock = create_sock(sock_path)
        .expect(format!("Failed to create Unix Socket at {}", sock_path).as_str());
    println!("Created request socket at {}", sock_path);

    loop {
        {
            //This handling can be changed after rust-lang/rust #42839
            let mut pid = String::new();

            let mut req_stream = req_sock
                .accept()
                .expect(format!("Failed to accept stream at {}", sock_path).as_str())
                .0;

            {
                let reader = req_stream
                    .try_clone()
                    .expect("Could not clone request steam.");
                let mut reader = std::io::BufReader::new(reader);
                reader
                    .read_line(&mut pid)
                    .expect("Failed to get data from request stream.");
                println!("Received request: {}", pid);
                pid.pop();
                if let Some(req_type) = pid.pop(){
                    if req_type == 's' {
                        println!("Request type is server ({})", req_type);
                        wg_addr = wg_serv_addr;
                    } else {
                        println!("Request type is client ({})", req_type);
                        wg_addr = wg_cli_addr;
                    }
                } else {
                    panic!("Recieved invalid request.")
                }
            }

            //.req_stream.read_to_string(&mut pid).expect("Failed to get data from request stream.");
            println!(
                "Received request to provide a wireguard interface to pid: {}",
                pid
            );

            println!("Creating wireguard interface.");
            {
                let create_wg_status = create_wg(&wg_name);
                if create_wg_status.success() {
                    println!("Created wireguard interace {}", wg_name);
                } else {
                    exitmsg(
                        format!("Failed to create wireguard interface {} (ip:", wg_name),
                        create_wg_status,
                    );
                }
            }

            println!("Configuring wireguard interface with wg.");
            {
                let wg_conf_wg_status = wg_conf_wg(&wg_name, &wg_conf);
                if wg_conf_wg_status.success() {
                    println!(
                        "Configured wireguard interface {} with wg and {}.",
                        &wg_name, &wg_conf
                    )
                } else {
                    exitmsg(
                        format!(
                            "Failed to configure wireguard interface {} with wg and {} (wg:",
                            &wg_name, &wg_conf
                        ),
                        wg_conf_wg_status,
                    )
                }
            }

            println!("Moving wiregard interface to pid netns.");
            {
                let move_wg_status = move_wg(&wg_name, &pid);
                if move_wg_status.success() {
                    println!(
                        "Moved wireguard interface {} to netns of pid: {}.",
                        wg_name, pid
                    );
                } else {
                    exitmsg(
                        format!(
                            "Failed to move wireguard interface {} to netns of pid: {}. (ip:",
                            wg_name,
                            &pid.to_string()
                        ),
                        move_wg_status,
                    );
                }
            }

            println!("Configuring wireguard in the pid netns.");
            {
                println!("Spawning netns thread.");
                let netns_conf_handle =
                    thread::spawn(move || netns_conf_ip(&wg_name, &wg_addr, &pid));
                netns_conf_handle
                    .join()
                    .expect("Netns configuration failed.");
            }

            req_stream
                .write("Done".as_bytes())
                .expect("Failed to respond to request.");
        }
    }
}
