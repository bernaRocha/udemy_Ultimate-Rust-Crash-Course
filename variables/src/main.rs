const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {

    let (mut missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles);
  
}
// Questão 01
// Compiling variables v2.3.4 (/home/bernardo/Documentos/udemy/udemy_Ultimate-Rust-Crash-Course/variables)
//Finished dev [unoptimized + debuginfo] target(s) in 0.32s
//Running `target/debug/variables`
//Firing 2 of my 8 missiles...