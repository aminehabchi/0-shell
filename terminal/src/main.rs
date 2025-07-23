mod reader;
use reader::main_loop;

mod parser;
mod command_router;

fn main() {
    // run main loop
    main_loop();
}
