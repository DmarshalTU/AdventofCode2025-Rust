mod errors_handler;
mod file_reader;
mod puzzle_engine;

fn main() {
    let filename = "input.txt";

    let input = match file_reader::read_input_file(filename) {
        Ok(input) => input,
        Err(e) => {
            errors_handler::handle_file_error(e, filename);
        }
    };

    let password = puzzle_engine::solve_puzzle(&input);
    println!("Password: {}", password);
}
