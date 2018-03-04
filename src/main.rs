extern crate nix;
use nix::sys::termios;
use std::io::Read;

fn main() {
    // Querying original as a separate, since `Termios` does not implement copy
    let orig_term = termios::tcgetattr(0).unwrap();
    let mut term = termios::tcgetattr(0).unwrap();
    // Unset canonical mode, so we get characters immediately
    term.local_flags.remove(termios::LocalFlags::ICANON);
    // Don't generate signals on Ctrl-C and friends
    term.local_flags.remove(termios::LocalFlags::ISIG);
    // Disable local echo
    term.local_flags.remove(termios::LocalFlags::ECHO);
    termios::tcsetattr(0, termios::SetArg::TCSADRAIN, &term).unwrap();
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
    termios::tcsetattr(0, termios::SetArg::TCSADRAIN, &orig_term).unwrap();
}
