use refraction_rdp::*;

fn main() {
    let pulse_path = &pulse_path();
    let wg_name = &"wg_refraction";
    let wg_addr = &"10.10.10.1/24";

    get_wireguard('s');
    exec_sunshine(pulse_path, wg_name, wg_addr);
}
