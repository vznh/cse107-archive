use rand::Rng;
use std::collections::HashMap;
// [START src/main]

fn main() {
    let n = 7;
    let p = 0.6;
    let q = 0.7;
    let trials = 100_000;

    let joint_frequency = run_simulation(n, p, q, trials);
    let joint_pmf = compute_joint_pmf(&joint_frequency, trials);

    print_joint_pmf(&joint_pmf, n);

    let (conditional_pmf_x_given_y, conditional_pmf_y_given_x) = compute_conditional_pmfs(&joint_pmf, n);

    print_conditional_pmf_x_given_y(&conditional_pmf_x_given_y, n);
    print_conditional_pmf_y_given_x(&conditional_pmf_y_given_x, n);
}

fn run_simulation(n: usize, p: f64, q: f64, trials: usize) -> HashMap<(usize, usize), usize> {
    let mut joint_frequency: HashMap<(usize, usize), usize> = HashMap::new();

    // Initialize joint frequency table with all possible (x, y) pairs
    for x in 0..=n {
        for y in 0..=x {
            joint_frequency.insert((x, y), 0);
        }
    }

    let mut rng = rand::thread_rng();

    // Run simulation for trials
    for _ in 0..trials {
        let mut x = 0; // Number of weeks Joe plays
        let mut y = 0; // Number of weeks Joe wins

        for _ in 0..n {
            if rng.gen::<f64>() < p {
                x += 1;
                if rng.gen::<f64>() < q {
                    y += 1;
                }
            }
        }

        if let Some(count) = joint_frequency.get_mut(&(x, y)) {
            *count += 1;
        }
    }

    joint_frequency
}

fn compute_joint_pmf(joint_frequency: &HashMap<(usize, usize), usize>, trials: usize) -> HashMap<(usize, usize), f64> {
    let mut joint_pmf: HashMap<(usize, usize), f64> = HashMap::new();
    for (&(x, y), &count) in joint_frequency {
        joint_pmf.insert((x, y), count as f64 / trials as f64);
    }
    joint_pmf
}

fn compute_conditional_pmfs(joint_pmf: &HashMap<(usize, usize), f64>, n: usize) -> (HashMap<(usize, usize), f64>, HashMap<(usize, usize), f64>) {
    let mut conditional_pmf_x_given_y: HashMap<(usize, usize), f64> = HashMap::new();
    let mut conditional_pmf_y_given_x: HashMap<(usize, usize), f64> = HashMap::new();

    // Calculate marginal PMFs
    let mut marginal_pmf_y: HashMap<usize, f64> = HashMap::new();
    let mut marginal_pmf_x: HashMap<usize, f64> = HashMap::new();

    for (&(x, y), &value) in joint_pmf {
        *marginal_pmf_y.entry(y).or_insert(0.0) += value;
        *marginal_pmf_x.entry(x).or_insert(0.0) += value;
    }

    // Calculate conditional PMFs
    for (&(x, y), &joint_value) in joint_pmf {
        if let Some(&marginal_y) = marginal_pmf_y.get(&y) {
            if marginal_y > 0.0 {
                conditional_pmf_x_given_y.insert((x, y), joint_value / marginal_y);
            }
        }
        if let Some(&marginal_x) = marginal_pmf_x.get(&x) {
            if marginal_x > 0.0 {
                conditional_pmf_y_given_x.insert((y, x), joint_value / marginal_x);
            }
        }
    }

    (conditional_pmf_x_given_y, conditional_pmf_y_given_x)
}

fn print_joint_pmf(joint_pmf: &HashMap<(usize, usize), f64>, n: usize) {
    println!("Joint PMF of X and Y");
    print!("   y:   ");
    for y in 0..=n {
        print!("{:>8}", y);
    }
    println!("");
    println!(" x ------------------------------------------------------------------");
    for x in 0..=n {
        print!("{:>2} |", x);
        for y in 0..=n {
            if y <= x {
                let value = joint_pmf.get(&(x, y)).unwrap_or(&0.0);
                print!("{:>8.4}", value);
            } else {
                print!("        ");
            }
        }
        println!("");
    }
}

fn print_conditional_pmf_x_given_y(conditional_pmf_x_given_y: &HashMap<(usize, usize), f64>, n: usize) {
    println!("\nConditional PMF of X given Y");
    print!("   y:   ");
    for y in 0..=n {
        print!("{:>8}", y);
    }
    println!("");
    println!(" x ------------------------------------------------------------------");
    for x in 0..=n {
        print!("{:>2} |", x);
        for y in 0..=n {
            if y <= x {
                let value = conditional_pmf_x_given_y.get(&(x, y)).unwrap_or(&0.0);
                print!("{:>8.4}", value);
            } else {
                print!("        ");
            }
        }
        println!("");
    }
}

fn print_conditional_pmf_y_given_x(conditional_pmf_y_given_x: &HashMap<(usize, usize), f64>, n: usize) {
    println!("\nConditional PMF of Y given X");
    print!("   y:   ");
    for y in 0..=n {
        print!("{:>8}", y);
    }
    println!("");
    println!(" x ------------------------------------------------------------------");
    for x in 0..=n {
        print!("{:>2} |", x);
        for y in 0..=n {
            if y <= x {
                let value = conditional_pmf_y_given_x.get(&(y, x)).unwrap_or(&0.0);
                print!("{:>8.4}", value);
            } else {
                print!("        ");
            }
        }
        println!("");
    }
}

// [END src/main]
