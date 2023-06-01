use refraction_rdp::*;

fn main() {
    let pulse_path = &pulse_path();
    let wg_name = &"wg-refraction";
    let wg_addr = &"10.10.10.1/24";

    get_wireguard('s', wg_name, wg_addr);
    exec_sunshine(pulse_path);
}
