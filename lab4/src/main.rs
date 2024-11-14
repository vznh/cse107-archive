use rand::Rng;

// [START lab4/src/main]
fn main() {
  let ps = [0.1_f64, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
  let qs = [0.1_f64, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
  let trials = 10_000_u32;

  let mut means = vec![vec![0.0; qs.len()]; ps.len()];
  let mut variances = vec![vec![0.0; qs.len()]; ps.len()];

  for (i, &p) in ps.iter().enumerate() {
    for (j, &q) in qs.iter().enumerate() {
      let (mean, variance) = experiment(p, q, trials);
      means[i][j] = mean;
      variances[i][j] = variance;
    }
  }

  // Print the mean table
  println!("mean");
  print!("    q: ");
  for &q in &qs {
    print!("{:^7.1}", q);
  }
  println!();
  println!("p   ------------------------------------------------------------------------");
  for (i, &p) in ps.iter().enumerate() {
    print!("{:<4}|", p);
    for &mean in &means[i] {
      print!("{:^7.3}", mean);
    }
    println!();
  }

  // Print the variance table
  println!("\nvariance");
  print!("    q: ");
  for &q in &qs {
    print!("{:^7.1}", q);
  }
  println!();
  println!("p   ------------------------------------------------------------------------");
  for (i, &p) in ps.iter().enumerate() {
    print!("{:<4}|", p);
    for &variance in &variances[i] {
      print!("{:^7.3}", variance);
    }
    println!();
  }
}

/// Simulates flipping a coin until getting heads
///
/// # Parameters
/// * `p` - Probability of getting heads on a single flip
///
/// # Returns
/// Number of flips needed to get heads
fn simulate_first_coin(p: f64) -> u32 {
  let mut rng = rand::thread_rng();
  let mut n = 0;

  loop {
    n += 1;
    if rng.gen::<f64>() < p {
      break;
    }
  }

  n
}

/// Simulates flipping a coin n times and counting heads
///
/// # Parameters
/// * `q` - Probability of getting heads on a single flip
/// * `n` - Number of times to flip the coin
///
/// # Returns
/// Number of heads obtained in n flips
fn simulate_second_coin(q: f64, n: u32) -> u32 {
  let mut rng = rand::thread_rng();
  let mut y = 0;

  for _ in 0..n {
    if rng.gen::<f64>() < q {
      y += 1;
    }
  }

  y
}

/// Runs multiple trials of the two-coin experiment
///
/// # Parameters
/// * `p` - Probability for first coin
/// * `q` - Probability for second coin
/// * `trials` - Number of trials to run
///
/// # Returns
/// Tuple containing the mean and variance of the second coin results
fn experiment(p: f64, q: f64, trials: u32) -> (f64, f64) {
  let mut ys = Vec::with_capacity(trials as usize);

  for _ in 0..trials {
    let n = simulate_first_coin(p);
    let y = simulate_second_coin(q, n);
    ys.push(y as f64);
  }

  let mean = ys.iter().sum::<f64>() / trials as f64;
  let var = ys.iter().map(|y| (y - mean).powi(2)).sum::<f64>() / trials as f64;

  (mean, var)
}
// [END lab4/src/main]
