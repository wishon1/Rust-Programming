// print Welcome to the Two-Player Guessing Game!
//  print Player 1, enter a secret number (it will be hidden): ****
// take input from user. dont show the number while taking input rather show ***
// and store as string trim and remove any space.
// print Player 2, try to guess the number between 1 and 100.
// take input from player 2 and store in memory dont show the number while entering
//

mod game;
mod player;

fn main() {
    println!("Welcome to the Two-player Guessing Game!");

    let mut play_again = true;
    while play_again {
        game::play_round();

        println!("Switch roles? (y/n): ");
        let mut answer = String::new();
        std::io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");
        
        play_again = answer.trim() == "y" || answer.trim() == "Y";
    }
    println!("Thanks for playing!");
}
