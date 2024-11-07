// src/main.rs
use rand::Rng;

fn main() {
  let n: i32 = 300;
  let mut bob_victories: f32 = 0.0;
  let loaded_probabilities = vec![0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];

  fn found_heads() -> bool {
    let mut rng = rand::thread_rng();

    rng.gen_range(0..2) == 0
  }

  fn found_heads_onloadedcoin(probability: f32) -> bool {
    rand::thread_rng().gen::<f32>() < probability
  }

  // Regular coin
  for _ in 0..1000 {
    let mut bob: i32 = 0;
    let mut alice: i32 = 0;

    for _ in 0..n+1 {
      if found_heads() {
        bob += 1;
      }
    }

    for _ in 0..n {
      if found_heads() {
        alice += 1;
      }
    }

    bob_victories += (bob > alice) as u8 as f32;
  }

  println!("Part 1: relative frequency: {}", bob_victories / 1000.0);

  // Loaded coin
  println!("--------------------------");
  println!("p relative frequency");
  println!("--------------------------");
  for &probability in &loaded_probabilities {
    let mut bob_loaded_victories: f32 = 0.0;
    for _ in 0..1000 {
      let mut bob_loaded: i32 = 0;
      let mut alice_loaded: i32 = 0;

      for _ in 0..n+1 {
        if found_heads_onloadedcoin(probability) {
          bob_loaded += 1;
        }
      }

      for _ in 0..n {
        if found_heads_onloadedcoin(probability) {
          alice_loaded += 1;
        }
      }

      bob_loaded_victories += (bob_loaded > alice_loaded) as u8 as f32;
    }

    println!("{:.1} {:.3}", probability, bob_loaded_victories / 1000.0);
  }
}
