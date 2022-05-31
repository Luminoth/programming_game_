use mlua::prelude::*;
use rand::Rng;

const POSSIBLE_PLAY_STRINGS: [&str; 3] = ["scissors", "rock", "paper"];
const SCORE_TABLE: [[isize; 3]; 3] = [[0, -1, 1], [1, 0, -1], [-1, 1, 0]];

fn main() -> anyhow::Result<()> {
    let lua = Lua::new();

    let globals = lua.globals();

    let get_ai_move = lua.create_function(|_, ()| Ok(get_ai_move()))?;
    globals.set("cpp_GetAIMove", get_ai_move)?;

    let evaluate_the_guesses = lua.create_function(
        |_, (user_guess, comp_guess, user_score, comp_score): (String, String, usize, usize)| {
            Ok(evaluate_the_guesses(
                &user_guess,
                &comp_guess,
                user_score,
                comp_score,
            ))
        },
    )?;
    globals.set("cpp_EvaluateTheGuesses", evaluate_the_guesses)?;

    lua.load(include_str!("../rock_paper_scissors.lua"))
        .exec()?;

    Ok(())
}

fn get_ai_move() -> &'static str {
    let mut rng = rand::thread_rng();

    POSSIBLE_PLAY_STRINGS[rng.gen_range(0..=2)]
}

fn guess_to_index(guess: impl AsRef<str>) -> Option<usize> {
    let guess = guess.as_ref();
    for (i, &play) in POSSIBLE_PLAY_STRINGS.iter().enumerate() {
        if guess == play {
            return Some(i);
        }
    }

    None
}

fn evaluate_the_guesses(
    user_guess: impl AsRef<str>,
    comp_guess: impl AsRef<str>,
    mut user_score: usize,
    mut comp_score: usize,
) -> (usize, usize) {
    let user_guess = user_guess.as_ref();
    let comp_guess = comp_guess.as_ref();
    println!("user guess: {} comp guess: {}", user_guess, comp_guess);

    let user_guess = guess_to_index(user_guess).unwrap();
    let comp_guess = guess_to_index(comp_guess).unwrap();

    let score = SCORE_TABLE[user_guess][comp_guess];
    if score == 1 {
        println!("You have won this round!");
        user_score += 1;
    } else if score == -1 {
        println!("Computer wins this round!");
        comp_score += 1;
    } else {
        println!("It's a draw!");
    }

    (user_score, comp_score)
}
