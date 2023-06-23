use std::io;

fn get_passcode(callsign: &str) -> u16 {
    let mut tmp_code = 29666;
    if callsign.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
        let callsign = callsign.to_ascii_uppercase();
        for i in (0..callsign.len()).step_by(2) {
            tmp_code ^= callsign.as_bytes()[i] as u16 * 256;
            tmp_code ^= callsign.as_bytes()[i + 1] as u16;
        }
        tmp_code &= 32767;
    } else {
        panic!("Invalid Callsign, Try Again!");
    }
    tmp_code
}

fn main() {
    let mut input = String::new();
    println!("Enter your callsign:");
    io::stdin().read_line(&mut input).unwrap();
    let passcode = get_passcode(input.trim());
    println!("Your passcode is: {}", passcode);
}