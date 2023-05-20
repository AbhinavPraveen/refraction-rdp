mod lib;

use crate::lib::*;

fn main() {
    let wg_name = "test";

    println!("Creating wireguard interface.");
    let create_wg_status = create_wg(&wg_name);
    
    if create_wg_status.success() {
        println!("Created wireguard interace {}", wg_name);
    } else {
        exitmsg(format!("Failed to create wireguard interface {} (ip:", wg_name), create_wg_status)
    }
}
