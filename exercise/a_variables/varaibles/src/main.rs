const STARTING_MISSILES: u8 = 8;
const READY_AMOUNT: u8 = 2;
fn main() {
    let (mut missiles, ready): (u8, u8) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles);
}
