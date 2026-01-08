use brainfuckers::*;

fn main() {
    let code: String = match std::env::args().nth(1) {
        Some(filename) => {
            let code: Vec<u8> = match std::fs::read(&filename) {
                Ok(c) => c,
                Err(error) => {
                    panic!("Failed to read file `{filename}` {error}")
                }
            };
            String::from_utf8(code).unwrap()
        },
        None => {
            let mut buf = String::new();
            match std::io::stdin().read_line(&mut buf) {
                Ok(_) => buf,
                Err(error) => panic!("{error}"),
            }
        },
    };

    let code = code.as_str();

    let mut state = new_bf_state(code);
    while step_bf(&mut state) {};
}
