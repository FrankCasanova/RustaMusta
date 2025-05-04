use crate::exercise_manager::{Exercise, ExerciseManager};
use std::io::{self, Read, Write};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{channel, Receiver, TryRecvError};
use std::thread;
use std::time::Duration;

pub mod file_watcher;
pub use file_watcher::FileWatcher;

enum TestState {
    Idle,
    Running(Child),
    Success,
    Failure,
}

pub struct InteractiveCLI {
    exercise_manager: ExerciseManager,
    file_watcher: Option<FileWatcher>,
    test_process: TestState,
    hint_shown: bool, // Track if hint was requested for the current state
    test_output_rx: Option<Receiver<String>>, // Channel to receive test output
}

impl InteractiveCLI {
    pub fn new() -> Self {
        let mut manager = ExerciseManager::new();
        manager.refresh_all_statuses(); // Ensure initial statuses are correct
        manager.set_to_first_incomplete();

        InteractiveCLI {
            exercise_manager: manager,
            file_watcher: None,
            test_process: TestState::Idle,
            hint_shown: false,
            test_output_rx: None,
        }
    }

    pub fn run(&mut self) {
        println!("Welcome to Rustlings Enhanced!");
        self.setup_current_exercise();

        loop {
            self.display_header();
            self.display_commands();
            self.process_test_state(); // Check on the running test
            self.check_file_changes(); // Check for file modifications

            // Non-blocking input check (basic example)
            // For a more robust solution, consider crates like `crossterm` for raw input
            // This simple version requires pressing Enter after the command character.
            let mut input = String::new();
            print!("> ");
            io::stdout().flush().unwrap();

            // Simple blocking read for now
            if io::stdin().read_line(&mut input).is_err() {
                eprintln!("Failed to read input.");
                continue;
            }

            match input.trim() {
                "q" => {
                    println!("\nTerminating test process if running...");
                    self.kill_test_process();
                    println!("Goodbye!");
                    break;
                }
                "l" => {
                    self.list_exercises();
                }
                "h" => {
                    self.show_hint();
                    self.hint_shown = true;
                }
                "n" => {
                    if let Some(exercise) = self.exercise_manager.get_current_exercise() {
                        if exercise.is_complete {
                            self.kill_test_process(); // Stop watching the old exercise
                            if self.exercise_manager.move_to_next_exercise() {
                                println!("\nMoving to the next exercise...");
                                self.setup_current_exercise();
                            } else {
                                println!("\nCongratulations! You have completed all exercises!");
                                // Optionally, break or enter a different state
                            }
                        } else {
                            println!("Please complete the current exercise first.");
                        }
                    } else {
                        println!("\nCongratulations! You have completed all exercises!");
                    }
                }
                "c" => {
                    self.check_all_exercises();
                }
                _ => {
                    println!("Unknown command.");
                }
            }
        }
    }

    fn setup_current_exercise(&mut self) {
        self.kill_test_process(); // Ensure previous process is stopped
        self.hint_shown = false;
        self.test_process = TestState::Idle;
        self.file_watcher = None;
        self.test_output_rx = None;

        if let Some(exercise) = self.exercise_manager.get_current_exercise() {
            if !exercise.is_complete {
                let file_path = format!("src/{}.rs", exercise.module_path.replace("::", "/"));
                println!("\nSetting up watcher for: {}", file_path);
                match FileWatcher::new(&file_path) {
                    Ok(watcher) => {
                        self.file_watcher = Some(watcher);
                        println!("Watching for changes... Starting initial test run.");
                        self.run_tests_for_current();
                    }
                    Err(e) => {
                        eprintln!("Error setting up file watcher for {}: {}", file_path, e);
                        eprintln!("Cannot automatically run tests for this exercise.");
                    }
                }
            } else {
                println!(
                    "\nCurrent exercise '{}' is already complete.",
                    exercise.display_name
                );
            }
        } else if self.exercise_manager.all_completed() {
            println!("\nCongratulations! All exercises are completed!");
        } else {
            // This case might occur if the list is empty or index is out of bounds somehow
            println!("\nNo current exercise found.");
        }
    }

    fn display_header(&self) {
        println!("\n========================================================");
        if let Some(exercise) = self.exercise_manager.get_current_exercise() {
            println!(
                "Current Exercise: [{}] {}",
                exercise.status_marker(),
                exercise.display_name
            );
            if !exercise.is_complete {
                let file_path = format!("src/{}.rs", exercise.module_path.replace("::", "/"));
                println!("Edit file: {}", file_path);
            }
        } else {
            println!("All exercises completed!");
        }
        println!(
            "Progress: {} / {}",
            self.exercise_manager.count_completed(),
            self.exercise_manager.get_exercises().len()
        );
        println!("========================================================");
    }

    fn display_commands(&self) {
        print!("Commands: [l] List | [c] Check All | [q] Quit");
        if let Some(exercise) = self.exercise_manager.get_current_exercise() {
            if !exercise.is_complete {
                print!(" | [h] Hint");
            } else {
                if self.exercise_manager.current_index
                    < self.exercise_manager.get_exercises().len() - 1
                {
                    print!(" | [n] Next Exercise");
                } else {
                    print!(" | (All Done!)")
                }
            }
        }
        println!(); // Newline after commands
    }

    /// Runs tests synchronously for a single exercise and returns true if successful.
    fn run_tests_for_exercise_sync(&self, exercise: &Exercise) -> bool {
        println!("  Checking '{}'...", exercise.display_name);
        let test_filter = exercise.module_path.clone();
        let output = Command::new("cargo")
            .args(["test", "--", &test_filter, "--color", "never", "--quiet"])
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    println!("    -> Success");
                    true
                } else {
                    println!("    -> Failed");
                    // Optionally print stderr for debugging:
                    // if !output.stderr.is_empty() {
                    //     println!("      Error: {}", String::from_utf8_lossy(&output.stderr));
                    // }
                    false
                }
            }
            Err(e) => {
                eprintln!("    -> Error running tests for '{}': {}", exercise.name, e);
                false
            }
        }
    }

    /// Checks all exercises, updates their status, and focuses on the first failing one.
    fn check_all_exercises(&mut self) {
        println!("\n--- Checking All Exercises ---");
        self.kill_test_process(); // Stop any current watch/test

        let mut first_failed_index: Option<usize> = None;
        let exercise_count = self.exercise_manager.get_exercises().len();

        for index in 0..exercise_count {
            // Get immutable reference first for running tests
            let exercise_name_display = self
                .exercise_manager
                .get_exercises()
                .get(index)
                .map(|ex| ex.display_name.clone())
                .unwrap_or_else(|| "Unknown Exercise".to_string());

            // Need to clone required info before potentially borrowing mutably
            let exercise_module_path = self
                .exercise_manager
                .get_exercises()
                .get(index)
                .map(|ex| ex.module_path.clone());

            if let Some(module_path) = exercise_module_path {
                // Construct a temporary Exercise-like struct or pass necessary info
                // to avoid borrowing self.exercise_manager immutably and mutably at the same time
                // For simplicity here, let's assume run_tests_for_exercise_sync can be adapted
                // or we fetch the necessary details beforehand.
                // We'll create a temporary Exercise struct for the test function.
                let temp_exercise = Exercise {
                    name: "".to_string(), // Not strictly needed for run_tests_for_exercise_sync
                    display_name: exercise_name_display.clone(),
                    module_path: module_path,
                    is_complete: false, // Status doesn't matter for running the test itself
                };

                let test_success = self.run_tests_for_exercise_sync(&temp_exercise);

                // Now get mutable reference to update status
                if let Some(exercise_mut) = self.exercise_manager.get_exercise_mut_by_index(index) {
                    // First, refresh status based on the 'I AM NOT DONE' marker
                    exercise_mut.refresh_status();
                    // An exercise is truly complete only if tests pass AND the marker is gone
                    let final_is_complete = test_success && exercise_mut.is_complete;

                    // Update the status based on the combined check
                    exercise_mut.is_complete = final_is_complete;

                    if !final_is_complete && first_failed_index.is_none() {
                        first_failed_index = Some(index);
                    }

                    // Refresh status based on marker
                    exercise_mut.refresh_status();
                    if !exercise_mut.is_complete && first_failed_index.is_none() {
                        first_failed_index = Some(index);
                    }
                } else {
                    eprintln!(
                        "Error: Could not get mutable reference to exercise at index {}",
                        index
                    );
                }
            } else {
                eprintln!(
                    "Error: Could not get module path for exercise at index {}",
                    index
                );
            }
        }

        println!("-----------------------------");

        if let Some(failed_index) = first_failed_index {
            println!("Found incomplete exercises. Focusing on the first one.");
            self.exercise_manager.current_index = failed_index;
        } else {
            println!("All exercises are currently complete!");
            // If all are complete, move to the last one.
            if exercise_count > 0 {
                self.exercise_manager.current_index = exercise_count - 1;
            }
        }

        // Reset the watcher and test state for the (potentially new) current exercise
        self.setup_current_exercise();
    }

    fn list_exercises(&self) {
        println!("\n--- Exercises ---");
        for (index, exercise) in self.exercise_manager.get_exercises().iter().enumerate() {
            let status = exercise.status_marker();
            let current_marker = if index == self.exercise_manager.current_index {
                "->"
            } else {
                "  "
            };
            println!("{} [{}] {}", current_marker, status, exercise.display_name);
        }
        println!("-----------------");
    }

    fn show_hint(&self) {
        println!("\n--- Hint ---");
        if let Some(exercise) = self.exercise_manager.get_current_exercise() {
            if exercise.is_complete {
                println!("Exercise already completed, no hints needed!");
            } else if let Some(hints) = self.exercise_manager.get_exercise_hints(&exercise.name) {
                if hints.is_empty() {
                    println!("No hints available for this exercise.");
                } else {
                    // Simple hint display: show the first hint
                    // Could be enhanced to cycle through hints or use self.hint_shown
                    println!("{}", hints[0]);
                }
            } else {
                println!("Could not find hints for this exercise.");
            }
        } else {
            println!("No current exercise selected.");
        }
        println!("------------");
    }

    fn run_tests_for_current(&mut self) {
        if !matches!(self.test_process, TestState::Running(_)) {
            if let Some(exercise) = self.exercise_manager.get_current_exercise() {
                if !exercise.is_complete {
                    // Only run if not complete
                    println!("\nRunning tests for '{}'...", exercise.name);
                    self.hint_shown = false; // Reset hint flag on new test run

                    // Construct the module path argument for `cargo test`
                    // e.g., `exercises::two_sum::recipe_ingredients`
                    let test_filter = exercise.module_path.clone();

                    let (tx, rx) = channel(); // Channel for test output
                    self.test_output_rx = Some(rx);

                    match Command::new("cargo")
                        .args(["test", "--", &test_filter, "--color", "always", "--quiet"])
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn()
                    {
                        Ok(child) => {
                            self.test_process = TestState::Running(child);
                            // Spawn a thread to capture output
                            if let Some(mut stdout) = self.stdout_handle() {
                                let tx_clone = tx.clone();
                                thread::spawn(move || {
                                    let mut buffer = [0; 1024];
                                    loop {
                                        match stdout.read(&mut buffer) {
                                            Ok(0) => break, // EOF
                                            Ok(n) => {
                                                if let Ok(s) =
                                                    String::from_utf8(buffer[..n].to_vec())
                                                {
                                                    if tx_clone.send(s).is_err() {
                                                        break; // Receiver dropped
                                                    }
                                                }
                                            }
                                            Err(_) => break, // Error reading
                                        }
                                    }
                                });
                            }
                            if let Some(mut stderr) = self.stderr_handle() {
                                let tx_clone = tx.clone();
                                thread::spawn(move || {
                                    let mut buffer = [0; 1024];
                                    loop {
                                        match stderr.read(&mut buffer) {
                                            Ok(0) => break, // EOF
                                            Ok(n) => {
                                                if let Ok(s) =
                                                    String::from_utf8(buffer[..n].to_vec())
                                                {
                                                    if tx_clone.send(s).is_err() {
                                                        break; // Receiver dropped
                                                    }
                                                }
                                            }
                                            Err(_) => break, // Error reading
                                        }
                                    }
                                });
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to start test process: {}", e);
                            self.test_process = TestState::Failure; // Mark as failure if spawn fails
                            self.test_output_rx = None;
                        }
                    }
                } else {
                    //println!("Skipping tests for completed exercise '{}'.", exercise.display_name);
                }
            } else {
                println!("No exercise selected to test.");
            }
        }
    }

    fn process_test_state(&mut self) {
        let mut next_state = None;
        let mut output_buffer = String::new();

        // Drain the output channel
        if let Some(rx) = &self.test_output_rx {
            loop {
                match rx.try_recv() {
                    Ok(output) => output_buffer.push_str(&output),
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => {
                        self.test_output_rx = None; // Channel closed
                        break;
                    }
                }
            }
        }

        if !output_buffer.is_empty() {
            // Print the captured output
            print!("{}", output_buffer);
            io::stdout().flush().unwrap(); // Ensure output is displayed
        }

        if let TestState::Running(ref mut child) = &mut self.test_process {
            match child.try_wait() {
                Ok(Some(status)) => {
                    // Process finished
                    println!("\nTest process finished with status: {}", status);
                    if status.success() {
                        println!("✅ Exercise completed successfully!");
                        next_state = Some(TestState::Success);
                        // Mark exercise as complete
                        if let Some(exercise) = self.exercise_manager.get_current_exercise_mut() {
                            if !exercise.is_complete {
                                // TODO: Ideally, we should modify the source file to remove
                                // "I AM NOT DONE", but for simplicity, we just update the state.
                                // This means the status might revert if the app restarts before
                                // the user manually removes the comment.
                                // A more robust solution involves parsing/modifying the file.
                                exercise.is_complete = true; // Update in-memory state
                                println!(
                                    "Marking exercise '{}' as complete in memory.",
                                    exercise.display_name
                                );
                                println!("You can now press 'n' to move to the next exercise.");
                            }
                        }
                    } else {
                        println!("❌ Tests failed. Keep trying!");
                        if !self.hint_shown {
                            println!("Hint: Press 'h' for a hint.");
                        }
                        next_state = Some(TestState::Failure);
                    }
                    self.test_output_rx = None; // Process finished, close channel
                }
                Ok(None) => {
                    // Process still running, do nothing here, output was handled above
                }
                Err(e) => {
                    eprintln!("Error waiting for test process: {}", e);
                    next_state = Some(TestState::Failure);
                    self.test_output_rx = None;
                }
            }
        } else if matches!(self.test_process, TestState::Success | TestState::Failure) {
            // If the state is already Success or Failure, reset to Idle
            // This allows rerunning tests manually or via file watcher
            // Don't reset immediately after success/failure to show the status message
            // Maybe add a small delay or require user input?
            // For now, let file watcher trigger the reset implicitly.
            // next_state = Some(TestState::Idle);
        }

        if let Some(state) = next_state {
            self.test_process = state;
        }
    }

    fn check_file_changes(&mut self) {
        if let Some(watcher) = &self.file_watcher {
            if watcher.check_for_changes() {
                println!("\nFile change detected! Re-running tests...");
                // Reset state to Idle before running tests again
                self.kill_test_process(); // Ensure the old process is gone
                self.test_process = TestState::Idle;
                self.run_tests_for_current();
            }
        }
    }

    fn kill_test_process(&mut self) {
        if let TestState::Running(mut child) =
            std::mem::replace(&mut self.test_process, TestState::Idle)
        {
            println!(
                "Attempting to terminate test process (PID: {})...",
                child.id()
            );
            match child.kill() {
                Ok(_) => {
                    println!("Test process terminated.");
                    // Give it a moment to release resources, then wait
                    thread::sleep(Duration::from_millis(100));
                    match child.wait() {
                        Ok(status) => println!("Terminated process exited with status: {}", status),
                        Err(e) => eprintln!("Error waiting for terminated process: {}", e),
                    }
                }
                Err(e) => {
                    eprintln!(
                        "Failed to terminate test process: {}. It might have already exited.",
                        e
                    );
                }
            }
        }
        // Ensure the output channel is cleared if the process was killed
        self.test_output_rx = None;
    }

    // Helper to get stdout handle from TestState::Running
    fn stdout_handle(&mut self) -> Option<std::process::ChildStdout> {
        if let TestState::Running(child) = &mut self.test_process {
            child.stdout.take()
        } else {
            None
        }
    }

    // Helper to get stderr handle from TestState::Running
    fn stderr_handle(&mut self) -> Option<std::process::ChildStderr> {
        if let TestState::Running(child) = &mut self.test_process {
            child.stderr.take()
        } else {
            None
        }
    }
}
