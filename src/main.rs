extern crate rand;
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;


use rand::Rng;

struct SimulationResult {
    win: bool,
    switch: bool,
}

custom_derive! {
    #[derive(Debug, PartialEq, Eq, Clone,
        IterVariants(DoorChoiceVariants), IterVariantNames(DoorChoiceVariantNames))]
    pub enum DoorChoice { Door1, Door2, Door3 }
}

// Run a single simulation of the Monty Hall problem.
fn simulate_choice<R: Rng>(door_choices: &Vec<DoorChoice>, rng: &mut R) -> SimulationResult {

    let car_choice    : DoorChoice = rand::sample( rng, door_choices.into_iter(), 1)[0].clone();
    let player_choice : DoorChoice = rand::sample( rng, door_choices.into_iter(), 1)[0].clone();

    // // The game host opens a door
    let host_choice   : DoorChoice = game_host_open_choice( &car_choice, &player_choice, rng );
    let switch_choice              = switch_door_choice( &player_choice, &host_choice );

    // Shall we switch?
    let we_should_switch = rng.gen();
    if we_should_switch {
        SimulationResult { win: switch_choice == car_choice, switch: we_should_switch }
    } else {
        SimulationResult { win: player_choice == car_choice, switch: we_should_switch }
    }
}

// Returns the door the game host opens given our choice and knowledge of
// where the car is. The game host will never open the door with the car.
fn game_host_open_choice<R: Rng>( car_choice: &DoorChoice, player_choice: &DoorChoice, rng: &mut R ) -> DoorChoice {
    let choices = free_door_choices(&[ car_choice, player_choice]);
    rand::sample(rng, choices.into_iter(), 1)[0].clone()
}


// Returns the door we switch to, given our current choice and
// the open door. There will only be one valid door.
fn switch_door_choice( player_choice: &DoorChoice, host_choice: &DoorChoice) -> DoorChoice {
    free_door_choices(&[ player_choice, host_choice])[0].clone()
}

fn free_door_choices(blocked: &[ &DoorChoice]) -> Vec< DoorChoice> {
    let door_choices  = DoorChoice::iter_variants();//.collect::<Vec<_>>();
    door_choices.filter(| x| !blocked.contains( &x)).collect()
}

pub
fn guessing_game() {
    // The estimation will be more accurate with more simulations
    let num_simulations = 1_000_000;

    let mut rng = rand::thread_rng();

    let door_choices  = DoorChoice::iter_variants().collect::<Vec<_>>();

    let (mut switch_wins, mut switch_losses) = (0, 0);
    let (mut keep_wins, mut keep_losses) = (0, 0);

    println!("Running {} simulations...", num_simulations);
    for _ in 0..num_simulations {
        // let result = simulate(&random_door, &mut rng);
        let result = simulate_choice( &door_choices, &mut rng);

        match (result.win, result.switch) {
            (true, true) => switch_wins += 1,
            (true, false) => keep_wins += 1,
            (false, true) => switch_losses += 1,
            (false, false) => keep_losses += 1,
        }
    }

    let total_switches = switch_wins + switch_losses;
    let total_keeps = keep_wins + keep_losses;

    println!("Switched door {} times with {} wins and {} losses",
             total_switches, switch_wins, switch_losses);

    println!("Kept our choice {} times with {} wins and {} losses",
             total_keeps, keep_wins, keep_losses);

    // With a large number of simulations, the values should converge to
    // 0.667 and 0.333 respectively.
    println!("Estimated chance to win if we switch: {}",
             switch_wins as f32 / total_switches as f32);
    println!("Estimated chance to win if we don't: {}",
             keep_wins as f32 / total_keeps as f32);
}

fn main() {
    guessing_game();
}