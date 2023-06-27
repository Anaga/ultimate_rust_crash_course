const STARTING_MISSILES: i8 = 8;
const READY_AMOUNT: i8 = 3;
fn main() {
    let (mut missiles, ready): (i8, i8) = (STARTING_MISSILES, READY_AMOUNT);
    while missiles!=0 {
        println!("Firing {} of my {} missiles...", ready, missiles);
        missiles = missiles - ready;
        println!("{} missiles left", missiles); 
    }
    println!("End of missiles!");
}
