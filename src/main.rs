use std::io::{self, Write};
use std::process::exit;

mod vmath;
mod tunnel;
mod edc;
mod file_explorer;

fn main() {
    loop {
        let input = {
            print!("select demo (1..3): ");
            io::stdout().flush().expect("Failed to flush stdout");
            let mut buffer = String::new(); // тут я создаю буфер для ввода

            io::stdin().read_line(&mut buffer).expect("Failed to read user line"); // ввод
            buffer.trim().to_string() // очистка от пробелов
        };
        if input == "1"
        {
            vmath::do_work();
            exit(0);
        }
        if input == "2"
        {
            tunnel::do_work();
            exit(0);
        }
        if input == "3"
        {
            file_explorer::do_work();
            exit(0);
        }
    }
}
