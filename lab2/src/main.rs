// [START src/main.rs]
use rand::Rng;

fn main() {
  let range_of_values = vec![10, 20, 30, 40, 50];


  fn pick_from_vase(a: &mut i32, c: &mut i32, last_color: Option<bool>) -> bool {
    // if only one ball remains, return if it's azure
    if *a + *c == 1 {
      return *a == 1;
    }

    let pick_azure = rand::thread_rng().gen_ratio(*a as u32, (*a + *c) as u32);

    match (last_color, pick_azure) {
      (Some(last), current) if last != current => {
        // diff color picked, replace and restart
        if current { *a += 1 } else { *c += 1 };
          pick_from_vase(a, c, None)
        },
        _ => {
          // same color or first pick, discard and continue
          if pick_azure { *a -= 1 } else { *c -= 1 };
            pick_from_vase(a, c, Some(pick_azure))
        }
    }
  }

  println!("----------------------------------------------");
  println!("#azure #carmine  proportion ending in azure");
  println!("----------------------------------------------");

  for &a in &range_of_values {
    let mut a_picked_last: f32 = 0.0;
    for _ in 0..2000 {
      let mut total = 100;
      let mut azure = a;
      let mut carmine = total - a;

      while total > 1 {
        pick_from_vase(&mut azure, &mut carmine, Some(true));
        total -= 1;
      }

      a_picked_last += (azure > carmine) as u8 as f32;
    }

    println!("  {}      {}     {:.4}", &a, 100 - a, a_picked_last / 2000.0);
  }
}

// [END src/main.rs]
