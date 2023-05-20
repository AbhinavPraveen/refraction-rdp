mod lib;

use crate::lib::*;
use std::{thread::spawn,sync::mpsc::{sync_channel, SyncSender, Receiver}, process::id};

fn main() {
    let wg_name = "test";

    {
        println!("Creating wireguard interface.");
        let create_wg_status = create_wg(&wg_name);
    
        if create_wg_status.success() {
            println!("Created wireguard interace {}", wg_name);
        } else {
            exitmsg(format!("Failed to create wireguard interface {} (ip:", wg_name), create_wg_status)
        }
    }


    {
        let (stay_behind_tx, stay_behind_rx): (SyncSender<()>, Receiver<()>) = sync_channel(0);
        let pid = id();
        let stay_behind_handle = spawn(move || {stay_behind_orig_netns(&wg_name, stay_behind_rx, pid)});

        let s = unshare_netns();
        if s == 0 {
            println!("Moved main thread to new netns")
        } else {
            panic!("Failed to unshare, err: {}", get_err())
        }

        stay_behind_tx.send(()).expect("Channel from main thread to original namespace broken.");

        stay_behind_handle.join().expect("Thread in original namespace failed to move wireguard interface.");

        pause()
    }
}
