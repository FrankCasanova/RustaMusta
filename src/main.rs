mod exercises;
mod exercise_manager;
mod cli;

use cli::InteractiveCLI;

fn main() {
    let mut interactive_cli = InteractiveCLI::new();
    interactive_cli.run();
}
