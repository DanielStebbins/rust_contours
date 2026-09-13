use std::env;

struct Walk {
    steps: u64,
    length: u32,
}

impl Walk {
    pub fn step_horizontal(self: &Self) -> Walk {
        Walk {
            steps: self.steps,
            length: self.length + 1,
        }
    }

    pub fn step_vertical(self: &Self) -> Walk {
        Walk {
            steps: self.steps | (1 << self.length),
            length: self.length + 1,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(
        args.len() == 3,
        "Requires 2 command line arguments, recieved {}",
        args.len() - 1
    );

    let automaton_size = &args[1].parse::<u32>().unwrap();
    let iterations = &args[2].parse::<u32>().unwrap();

    println!("Building automaton of size {automaton_size}");
    println!("And running it for {iterations} iterations");
}
