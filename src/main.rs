extern crate nix;
use nix::sys::termios;
use std::io::Read;

fn main() {
    let saved_term = termios::tcgetattr(0).unwrap();
    let mut term = saved_term;
    // Unset canonical mode, so we get characters immediately
    term.c_lflag.remove(termios::ICANON);
    // Don't generate signals on Ctrl-C and friends
    term.c_lflag.remove(termios::ISIG);
    // Disable local echo
    term.c_lflag.remove(termios::ECHO);
    termios::tcsetattr(0, termios::TCSADRAIN, &term).unwrap();
    println!("Press Ctrl-C to quit");
    for byte in std::io::stdin().bytes() {
        let byte = byte.unwrap();
        if byte == 3 {
            break;
        } else {
            println!("You pressed byte {}", byte);
        }
    }
    println!("Goodbye!");
    termios::tcsetattr(0, termios::TCSADRAIN, &saved_term).unwrap();
}
