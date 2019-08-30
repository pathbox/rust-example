use rand::Rng;
use rand::distributions::{IndependentSample, Range};

struct SimulationResult {
  win: bool,
  switch: bool,
}

fn simulate<R: Rng>(random_door: &Range<u32>, rng: &mut R)
    -> SimulationResult {
      let car = random_door.ind_sample(rng);

      let mut choice = random_door.ind_sample(rng);

      let open = game_host_open(car, choice, rng);

      let switch = rng.gen()
      if switch {
        choice = switch_door(choice, open);
      }

      SimulationResult {win: choice == car, switch: switch}
}


fn game_host_open<R: Rng>(car: u32, choice: u32, rng: &mut R) -> u32 {
  let choice = free_doors(&[car, choice]);
  rand::seq::sample_slice(rng, &choices, 1)[0]
}

fn switch_door(choice: u32, open: u32) -> u32 {
  free_doors(&[choice, open])[0]
}

fn free_doors(blocked: &[u32]) -> Vec<u32> {
  (0..3).filter(|x| !blocked.contains(x)).collect()
}

fn main() {
  let num_simulations = 10000;

  let mut rng = rand::thread_rng();
  let random_door = Range::new(0,3);

  let (mut switch_wins, mut switch_losses) = (0,0);
  let (mut keep_wins, mut keep_losses) = (0,0);

  println!("Running {} simulations...", num_simulations);

  for _ in 0..num_simulations {
    let result = simulate(&random_door, &mut rng);

    match (result.win, result.siwtch) {
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