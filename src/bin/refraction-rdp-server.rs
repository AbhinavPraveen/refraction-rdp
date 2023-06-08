use refraction_rdp::*;

fn main() {
    let pulse_path = &pulse_path();
    let wg_addr = &"10.10.10.1/24";

    get_wireguard('s', wg_addr);
    exec_sunshine(pulse_path);
}
