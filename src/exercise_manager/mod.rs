use std::collections::HashMap;
use std::fs;

pub struct Exercise {
    pub name: String,
    pub display_name: String,
    pub module_path: String,
    pub is_complete: bool,
}

impl Exercise {
    pub fn new(name: &str, display_name: &str, module_path: &str) -> Self {
        let file_path = format!("src/{}.rs", module_path.replace("::", "/"));
        let is_complete = match fs::read_to_string(&file_path) {
            Ok(content) => !content.contains("I AM NOT DONE"),
            Err(_) => false,
        };

        Exercise {
            name: name.to_string(),
            display_name: display_name.to_string(),
            module_path: module_path.to_string(),
            is_complete,
        }
    }

    pub fn status_marker(&self) -> &str {
        if self.is_complete {
            "O"
        } else {
            "X"
        }
    }

    pub fn refresh_status(&mut self) {
        let file_path = format!("src/{}.rs", self.module_path.replace("::", "/"));
        self.is_complete = match fs::read_to_string(&file_path) {
            Ok(content) => !content.contains("I AM NOT DONE"),
            Err(_) => false,
        };
    }
}

pub struct ExerciseManager {
    exercises: Vec<Exercise>,
    pub current_index: usize,
    hints: HashMap<String, Vec<String>>,
}

impl ExerciseManager {
    pub fn new() -> Self {
        let mut manager = ExerciseManager {
            exercises: Vec::new(),
            current_index: 0,
            hints: HashMap::new(),
        };

        // Add exercises
        manager.add_exercise(
            "two_sum/two_sum_v1",
            "Two Sum (Classic LeetCode Problem)",
            "exercises::two_sum::two_sum_v1",
            vec![
                "Try using a data structure to store values you've seen while iterating.".to_string(),
                "For each number, calculate what value would complement it to reach the target.".to_string(),
            ],
        );

        manager.add_exercise(
            "two_sum/warehouse_weight",
            "Warehouse Box Weight Matcher",
            "exercises::two_sum::warehouse_weight",
            vec![
                "This problem is structurally similar to the classic Two Sum problem.".to_string(),
                "Think about what data structure could help you find matching boxes efficiently.".to_string(),
            ],
        );

        manager.add_exercise(
            "two_sum/construction_bars",
            "Construction Bar Length Finder",
            "exercises::two_sum::construction_bars",
            vec![
                "For each bar, what other length would you need to reach the target?".to_string(),
                "How can you quickly determine if that complementary length exists?".to_string(),
            ],
        );

        manager.add_exercise(
            "two_sum/recipe_ingredients",
            "Recipe Ingredient Calorie Counter",
            "exercises::two_sum::recipe_ingredients",
            vec![
                "Think about the target calories minus each ingredient's calories.".to_string(),
                "Is there a way to store ingredient information for quick lookups?".to_string(),
            ],
        );

        // Find the first incomplete exercise
        manager.set_to_first_incomplete();

        manager
    }

    fn add_exercise(&mut self, name: &str, display_name: &str, module_path: &str, hints: Vec<String>) {
        self.exercises.push(Exercise::new(name, display_name, module_path));
        self.hints.insert(name.to_string(), hints);
    }

    pub fn get_exercises(&self) -> &Vec<Exercise> {
        &self.exercises
    }

    pub fn get_current_exercise(&self) -> Option<&Exercise> {
        self.exercises.get(self.current_index)
    }

    pub fn get_current_exercise_mut(&mut self) -> Option<&mut Exercise> {
        self.exercises.get_mut(self.current_index)
    }

    pub fn get_exercise_hints(&self, exercise_name: &str) -> Option<&Vec<String>> {
        self.hints.get(exercise_name)
    }

    pub fn get_current_hints(&self) -> Option<&Vec<String>> {
        self.get_current_exercise().and_then(|ex| self.hints.get(&ex.name))
    }

    pub fn set_to_first_incomplete(&mut self) {
        self.current_index = self.exercises
            .iter()
            .position(|ex| !ex.is_complete)
            .unwrap_or(0);
    }

    pub fn move_to_next_exercise(&mut self) -> bool {
        if self.current_index + 1 < self.exercises.len() {
            self.current_index += 1;
            true
        } else {
            false
        }
    }

    pub fn refresh_all_statuses(&mut self) {
        for exercise in &mut self.exercises {
            exercise.refresh_status();
        }
    }

    pub fn all_completed(&self) -> bool {
        self.exercises.iter().all(|ex| ex.is_complete)
    }

    pub fn count_completed(&self) -> usize {
        self.exercises.iter().filter(|ex| ex.is_complete).count()
    }
}