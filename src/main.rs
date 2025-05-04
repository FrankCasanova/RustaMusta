mod exercises;

use std::env;
use std::fs;
use std::process;

fn main() {
    println!("Welcome to LeetCode 75 in Rust!");
    println!("=================================\n");
    
    let args: Vec<String> = env::args().collect();
    let default_command = String::from("help");
    let command = args.get(1).unwrap_or(&default_command).as_str();
    
    match command {
        "list" => list_exercises(),
        "verify" => verify_exercise(args.get(2)),
        "watch" => watch_exercise(args.get(2)),
        "hint" => show_hint(args.get(2)),
        _ => show_help(),
    }
}

fn list_exercises() {
    println!("Available exercises:");
    println!("  1. two_sum/two_sum_v1 - Two Sum (Classic LeetCode Problem)");
    println!("  2. two_sum/warehouse_weight - Warehouse Box Weight Matcher");
    println!("  3. two_sum/construction_bars - Construction Bar Length Finder");
    println!("  4. two_sum/recipe_ingredients - Recipe Ingredient Calorie Counter");
}

fn verify_exercise(exercise_name: Option<&String>) {
    match exercise_name {
        Some(name) => {
            match name.as_str() {
                "two_sum/two_sum_v1" => run_test("exercises::two_sum::two_sum_v1"),
                "two_sum/warehouse_weight" => run_test("exercises::two_sum::warehouse_weight"),
                "two_sum/construction_bars" => run_test("exercises::two_sum::construction_bars"),
                "two_sum/recipe_ingredients" => run_test("exercises::two_sum::recipe_ingredients"),
                _ => println!("Exercise '{}' not found. Use 'list' to see available exercises.", name),
            }
        },
        None => println!("Please specify an exercise to verify. Use 'list' to see available exercises."),
    }
}

fn run_test(module_path: &str) {
    println!("Running tests for {}...", module_path);
    
    // Check if the exercise is marked as done
    let file_path = format!("src/{}.rs", module_path.replace("::", "/"));
    if let Ok(content) = fs::read_to_string(&file_path) {
        if content.contains("I AM NOT DONE") {
            println!("This exercise is still marked as 'I AM NOT DONE'. Remove this comment to submit the exercise.");
            process::exit(1);
        }
    }
    
    // This is a simple simulation - in a real app, you'd use Cargo to run the tests
    println!("Running: cargo test --lib {}", module_path);
    println!("Hint: Use actual 'cargo test --lib {}' command to run tests for real.", module_path);
}

fn watch_exercise(exercise_name: Option<&String>) {
    match exercise_name {
        Some(name) => {
            println!("Watching exercise '{}'...", name);
            println!("(In a real implementation, this would watch for file changes and automatically verify.)");
            println!("Hint: Implement file watching using the 'notify' crate.");
        },
        None => println!("Please specify an exercise to watch. Use 'list' to see available exercises."),
    }
}

fn show_hint(exercise_name: Option<&String>) {
    match exercise_name {
        Some(name) => {
            match name.as_str() {
                "two_sum/two_sum_v1" => {
                    println!("Hint for Two Sum (Classic LeetCode Problem):");
                    println!("  - Try using a data structure to store values you've seen while iterating.");
                    println!("  - For each number, calculate what value would complement it to reach the target.");
                },
                "two_sum/warehouse_weight" => {
                    println!("Hint for Warehouse Box Weight Matcher:");
                    println!("  - This problem is structurally similar to the classic Two Sum problem.");
                    println!("  - Think about what data structure could help you find matching boxes efficiently.");
                },
                "two_sum/construction_bars" => {
                    println!("Hint for Construction Bar Length Finder:");
                    println!("  - For each bar, what other length would you need to reach the target?");
                    println!("  - How can you quickly determine if that complementary length exists?");
                },
                "two_sum/recipe_ingredients" => {
                    println!("Hint for Recipe Ingredient Calorie Counter:");
                    println!("  - Think about the target calories minus each ingredient's calories.");
                    println!("  - Is there a way to store ingredient information for quick lookups?");
                },
                _ => println!("Hint not available for '{}'. Use 'list' to see available exercises.", name),
            }
        },
        None => println!("Please specify an exercise for a hint. Use 'list' to see available exercises."),
    }
}

fn show_help() {
    println!("LeetCode 75 in Rust - Commands:");
    println!("  list                   - List all available exercises");
    println!("  verify [exercise]      - Verify your solution for an exercise");
    println!("  watch [exercise]       - Watch an exercise for changes and verify automatically");
    println!("  hint [exercise]        - Show a hint for an exercise");
    println!("  help                   - Show this help message");
    println!("\nExample usage:");
    println!("  cargo run -- list");
    println!("  cargo run -- verify two_sum/warehouse_weight");
}
