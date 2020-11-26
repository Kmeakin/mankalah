use mankalah::grammar::ProtocolGrammar;
use std::io::BufRead;

fn read_line() -> String {
    let mut line = String::new();
    let stdin = std::io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line
}

pub fn main() {
    loop {
        let line = read_line();
        let message = ProtocolGrammar::EngineMessage(&line);
        dbg!(&message);
    }
}
