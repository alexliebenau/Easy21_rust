use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use std::io;

pub struct Card {
    color: bool, // true: black, false: red
    value: i16,
}

pub struct Gamestate {
    pub dealer: i16,
    pub player: i16,
    pub is_terminal: bool,
    pub reward: i16,
}

pub struct Game {
    pub dealer_sum: i16,
    pub player_sum: i16,
}

impl Game {
    pub fn step(&mut self, a: bool) -> Gamestate { // true: hit; false: stick
        // classic step fn
        let mut rew: i16 = 0;
        let mut terminal: bool = false;

        if a {
            self.player_sum = sum(self.player_sum, draw_card());
            if is_bust(self.player_sum) {
                rew = -1;
                terminal = true;
            }
        } else { // dealer finishes game
            while (0 <= self.dealer_sum) && (self.dealer_sum <= 17) {
               self.dealer_sum = sum(self.dealer_sum, draw_card()) // dealer draws next card
            }
            if is_bust(self.dealer_sum) || self.dealer_sum < self.player_sum {
                rew = 1;
                terminal = true;
            } else if self.dealer_sum > self.player_sum {
                rew = -1;
                terminal = true;
            } else {terminal = true}; // in this case dealer = player
        }
        let state = Gamestate {
            dealer: self.dealer_sum,
            player: self.player_sum,
            is_terminal: terminal,
            reward: rew,
        };
        return state
        }
    }

pub fn sum(mut sum: i16, c: Card) -> i16{
    // add card c to player sum
    if c.color { // is black
        sum += c.value
    } else { // is red
        sum -= c.value
    }
    return sum
}

fn is_bust(val: i16) -> bool {
    // check if is bust
    return !((0 <= val) && (val <= 21))
}

fn draw_card() -> Card {
    // Get card color with 2/3 probability of it being black
    let mut rng = thread_rng(); // create randomness
    let choices: [bool; 3] = [true, true, false];
    let color =  *choices.choose(&mut rng).unwrap();
    let value: i16 = rng.gen_range(2..11); // get random value between 1 and 10 -> [1, 11)

    // create new card
    let new_card = Card {
        color,
        value,
    };

    return new_card
}

pub fn main() {
    println!(" ############ Hello! Welcome to a new round of Easy21.\n\n");

    let mut rng = thread_rng();
    let player_val: i16 = rng.gen_range(1..11);
    let dealer_val: i16 = rng.gen_range(1..11);


    let mut input = String::new();
    let mut state: Gamestate;

    let mut inst = Game {
        dealer_sum: dealer_val,
        player_sum: player_val,
    };

    println!("Your card is a black {}, the dealer has a black {}\n", player_val, dealer_val);

    loop {
        println!("Do you want another card? [y / n]");

        io::stdin()
            .read_line(&mut input)
            .expect("That did not work. Ctrl+C this shit...");

        match input.as_str().trim() {
            "y" => state = inst.step(true),
            "n" => state = inst.step(false),
            _ => {
                println!("I could not read that input. Will end game.");
                state = inst.step(false);
                },
        }

        if state.is_terminal {
            println!("Game over! \n");
            println!("You have {} points, the dealer has {} points.", state.player, state.dealer);
            println!("Your reward is {}", state.reward);
            break
        } else{
            println!("Your sum is now {}", state.player);
            input.truncate(0);
        }
    }
}

