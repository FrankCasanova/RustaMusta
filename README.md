# 🦀 RustAmusta: Where Variables Lift Ownership! 

```
   (\_/)
  ( •_•)
  / >🚀 "Compiling gains!"
```

Welcome to the **sweatiest** Rust dojo where we turn `Result<(), Error>` into `Result<(), Gains>`! 💪

## 🚀 Getting Started: Pumping Rust Iron

### Installation (No Spotter Needed)
```bash
# Install Rust compiler (if you haven't already)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone our virtual gym
git clone https://github.com/your-repo/rustamusta.git
cd rustamusta  # Enter our virtual weight room
cargo run rustamusta
```

### Pro Tips
🏋️ `cargo clean` = Sauna break (reduces target/ directory bloat)
🔧 `cargo fmt` = Get your code gains Instagram-ready
🦀 `cargo doc --open` = Study the exercise manuals

⚠️ Warning: Our ownership rules are stricter than a gym bro's curl form!

## 🎯 Exercises Breakdown

Our current workout plan includes:
- `merge_strings_alternately`: String-stitching olympics
- `two_sum`: HashMap heavy-lifting
- Coming soon: Lifetime limbo (how low can you go?)

## 🏋️ Contributing New Exercises (AKA How to Avoid Borrow Checker Side-Eye)

### Step 1: Create Your Exercise Cave
```rust
// exercises/your_module/src/lib.rs
pub fn exercise_name(param: Type) -> Result<(), Error> {
    // Your code here (no panics allowed!)
}
```

### Step 2: Test Your Mettle
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_party_parrot() {
        assert_eq!(exercise_name("🐦"), Ok(()));
    }
}
```

### Step 3: Register with the Exercise Manager
Navigate to `src/exercise_manager/mod.rs` and add:

```rust
manager.add_exercise(
    "your_module/exercise_name",
    "Funny Display Name",
    "exercises::your_module::exercise_name",
    vec![
        "This hint is hotter than a CPU running cargo build --release".into(),
    ],
);
```

## 🦀 Rusty Rules (Read Before You PR)

1. **Ownership Olympics**: Your code must pass the borrow checker without any cloning cheating! 🏅
2. **Error Handling**: Panics are like dropping weights - only acceptable if you yell "LIFTOFF!" first 🚀
3. **Performance**: Faster than `cargo clean` after deleting target/!

## 📜 Contributor Covenant

By submitting PRs you agree:
- To laugh at least once per compiler error
- To name variables more creatively than `x`, `y`, `z`
- That Ferris is the one true crab 🦀

```
   (\_/)
  ( •_•)
  / >🎉 "Happy coding!"
```