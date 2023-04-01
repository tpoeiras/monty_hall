use rand::distributions::{Distribution, Uniform};
use rand::rngs::StdRng;
use rand::seq::IteratorRandom;
use rand::SeedableRng;

fn main() {
    let mut rng = StdRng::seed_from_u64(131254153212);

    let mut num_never = 0;
    let mut num_always = 0;
    for _ in 0..100000 {
        let (never, always) = random_game(&mut rng);
        if never {
            num_never += 1;
        }

        if always {
            num_always += 1;
        }
    }

    println!("NEVER: {num_never}");
    println!("ALWAYS: {num_always}");
}

fn random_game(rng: &mut StdRng) -> (bool, bool) {
    let between = Uniform::from(0..3);
    let correct_door = between.sample(rng);

    let player_choice = between.sample(rng);

    let mut other_choices = (0..3).filter(|&c| c != player_choice);
    let opened_door = other_choices
        .clone()
        .filter(|&d| d != correct_door)
        .choose(rng)
        .unwrap();
    let change = other_choices.find(|&d| d != opened_door).unwrap();

    (player_choice == correct_door, change == correct_door)
}
