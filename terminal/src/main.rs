mod reader;
use reader::main_loop;

mod parser;
mod command_router;

fn main() {
    main_loop();
}
