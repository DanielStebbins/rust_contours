use std::env;

#[inline]
pub fn get_x_step(y: i8) -> i8 {
    1 - ((y & 1) << 1)
}

#[inline]
pub fn get_y_step(x: i8) -> i8 {
    1 - ((x & 1) << 1)
}

struct Walk {
    steps: u64,
    length: u8,
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

    pub fn get_endpoint(self: &Self) -> (i8, i8) {
        let mut x = 0;
        let mut y = 0;
        let mut x_step = get_x_step(y);
        let mut y_step = get_y_step(x);
        for _ in 0..self.length {
            if (self.steps & 1) == 1 {
                y += y_step;
                x_step = -x_step;
            } else {
                x += x_step;
                y_step = -y_step;
            }
        }
        (x, y)
    }

    pub fn has_loop(self: &Self, end_x: i8, end_y: i8) -> bool {
        if self.length < 12 {
            return false;
        }
        let mut steps = self.steps;
        let limit = self.length;
        let mut x = 0;
        let mut y = 0;
        let mut x_step = get_x_step(y);
        let mut y_step = get_y_step(x);
        let mut loop_found = x == end_x && y == end_y;
        let mut i = 0;
        while !loop_found && i < limit {
            if (steps & 1) == 1 {
                y += y_step;
                x_step = -x_step;
            } else {
                x += x_step;
                y_step = -y_step;
            }
            loop_found = x == end_x && y == end_y;
            steps >>= 1;
            i += 1;
        }
        loop_found
    }

    #[inline]
    pub fn approach(self: &Self) -> u8 {
        ((self.steps >> (self.length - 2)) & 0b11) as u8
    }

    pub fn to_binary(self: &Self) -> String {
        format!("{:b}", self.steps)
    }
}

fn brute_force(length: u8) -> u64 {
    let mut walks: Vec<Walk> = vec![];
    walks.push(Walk {
        steps: 0,
        length: 1,
    });
    let mut count: u64 = 0;
    while !walks.is_empty() {
        let current = walks.pop().expect("Should not be empty");
        let horizontal = current.step_horizontal();
        let (hx, hy) = horizontal.get_endpoint();
        if !horizontal.has_loop(hx, hy) {
            if horizontal.length < length {
                walks.push(horizontal);
            } else {
                count += 1;
            }
        }
        let vertical = current.step_vertical();
        let (vx, vy) = vertical.get_endpoint();
        if !vertical.has_loop(vx, vy) {
            if vertical.length < length {
                walks.push(vertical);
            } else {
                count += 1;
            }
        }
    }
    count << 1
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(
        args.len() == 3,
        "Requires 2 command line arguments, recieved {}",
        args.len() - 1
    );

    let automaton_size = args[1].parse::<u32>().unwrap();
    let iterations = args[2].parse::<u8>().unwrap();

    println!("Building automaton of size {automaton_size}");
    println!("And running it for {iterations} iterations");

    let out = brute_force(iterations);
    println!("{}", out);
}
