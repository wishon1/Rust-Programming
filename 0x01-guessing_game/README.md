# 0x01. Rust - Programming a Guessing Game
**Rust**
* Weight: 1
* Project duration: 2 days

## Concepts
*For this project, we expect you to look at these concepts:*
* Basic Rust syntax
* User input and output
* Random number generation
* Error handling

## Resources
Read or watch:
* The Rust Programming Language - Chapter 2
* [Standard Library Documentation](https://doc.rust-lang.org/std/)
* [Rust By Example - Guessing Game](https://doc.rust-lang.org/rust-by-example/std/result.html)

## Learning Objectives
At the end of this project, you are expected to be able to explain to anyone, without the help of Google:
* How to handle user input in Rust
* How to use external crates
* How to manage dependencies with Cargo
* How to generate random numbers
* How to handle match expressions
* How to implement basic error handling

## Requirements
* Allowed editors: vi, vim, emacs, VS Code
* All your files will be compiled on Ubuntu 20.04 LTS using rustc
* All your files should end with a new line
* A README.md file at the root of the repo, containing a description of the repository
* Your code should use the Rust 2021 edition

## Tasks

### 0. Basic Guessing Game
**mandatory**

Create a guessing game where the computer picks a random number between 1 and 100 and the player tries to guess it.
* The program must use the rand crate to generate a random number
* It should prompt the user for input
* It should tell the user if their guess is too small, too large, or correct
* The game should continue until the user guesses correctly
* You must use proper error handling for non-numeric input

```
$ cargo run
Guess the number!
The secret number is between 1 and 100.
Please input your guess: 50
Your guess: 50
Too small!
Please input your guess: 75
Your guess: 75
Too large!
Please input your guess: 62
Your guess: 62
You win! It took you 3 guesses.
$
```

**Repo:**
* GitHub repository: `alx-rust_programming`
* Directory: `0x01-guessing_game`
* File: `guessing_game/src/main.rs`, `guessing_game/Cargo.toml`

### 1. Enhanced Guessing Game
**mandatory**

Enhance the basic guessing game with the following features:
* Keep track of the number of guesses
* Set a maximum number of allowed guesses (e.g., 10)
* Add difficulty levels (easy: 1-50, medium: 1-100, hard: 1-200)
* Allow the player to play again without restarting the program
* Add colors to your outputs (using the colored crate)

```
$ cargo run
Welcome to the Enhanced Guessing Game!

Select difficulty:
1. Easy (1-50)
2. Medium (1-100)
3. Hard (1-200)
Your choice: 2

Guess the number! (Medium difficulty)
You have 10 guesses remaining.
Please input your guess: 50
Your guess: 50
Too large! You have 9 guesses remaining.
Please input your guess: 25
Your guess: 25
Too small! You have 8 guesses remaining.
Please input your guess: 35
Your guess: 35
You win! It took you 3 guesses.

Play again? (y/n): y

Select difficulty:
...
$
```

**Repo:**
* GitHub repository: `alx-rust_programming`
* Directory: `0x01-guessing_game`
* File: `enhanced_game/src/main.rs`, `enhanced_game/Cargo.toml`

### 2. Game with Statistics
**mandatory**

Modify your enhanced guessing game to keep track of player statistics.
* Track wins and losses
* Calculate and display average number of guesses per win
* Store the best score (lowest number of guesses)
* Display a summary of statistics before exiting
* Your program should handle all possible error cases gracefully

```
$ cargo run
Welcome to the Guessing Game with Statistics!

[Game interface as before]
...

Game Statistics:
Total games played: 3
Games won: 2
Games lost: 1
Win rate: 66.7%
Average guesses per win: 4.5
Best score: 3 guesses

Thanks for playing!
$
```

**Repo:**
* GitHub repository: `alx-rust_programming`
* Directory: `0x01-guessing_game`
* File: `stats_game/src/main.rs`, `stats_game/Cargo.toml`

### 3. Timed Guessing Game
**advanced**

Create a timed version of the guessing game.
* The player has a limited time to guess the number (e.g., 30 seconds)
* Display a timer counting down
* End the game if time runs out
* Use the chrono crate to manage time
* The program should have a clean termination even if the user interrupts with Ctrl+C

```
$ cargo run
Welcome to the Timed Guessing Game!
You have 30 seconds to guess the number between 1 and 100.

Time remaining: 30s
Please input your guess: 50
Your guess: 50
Too small!

Time remaining: 26s
Please input your guess: 75
Your guess: 75
Too large!

Time remaining: 22s
Please input your guess: 62
Your guess: 62
You win! It took you 3 guesses and 9 seconds.

Play again? (y/n): n
$
```

**Repo:**
* GitHub repository: `alx-rust_programming`
* Directory: `0x01-guessing_game`
* File: `timed_game/src/main.rs`, `timed_game/Cargo.toml`

### 4. Two-Player Game
**advanced**

Create a two-player version of the guessing game.
* One player sets the secret number
* The second player tries to guess it
* Implement a "hot and cold" hint system rather than larger/smaller
* Add a scoring system based on number of guesses and time taken
* Use proper Rust modules to organize your code

```
$ cargo run
Welcome to the Two-Player Guessing Game!

Player 1, enter a secret number (it will be hidden): ****

Player 2, try to guess the number between 1 and 100.
Please input your guess: 50
Cold.
Please input your guess: 90
Warm!
Please input your guess: 95
Hot!
Please input your guess: 97
Very hot!
Please input your guess: 98
You win! The secret number was 98.
Score: 820 points

Switch roles? (y/n): y
...
$
```

**Repo:**
* GitHub repository: `alx-rust_programming`
* Directory: `0x01-guessing_game`
* File: `two_player/src/main.rs`, `two_player/src/game.rs`, `two_player/src/player.rs`, `two_player/Cargo.toml`

