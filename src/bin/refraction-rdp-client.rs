use refraction_rdp::*;

fn main() {
    let pulse_path = &pulse_path();
    let wg_addr = &"10.10.10.2/24";

    get_wireguard('c', wg_addr);
    exec_moonlight(pulse_path);
}
