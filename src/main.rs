use std::collections::HashSet;

struct ZnZmStar {
    n: u32,
    m: u32,
}

impl ZnZmStar {
    fn new(n: u32, m: u32) -> Self {
        ZnZmStar { n, m }
    }

    fn find_generators(&self) -> Vec<(u32, u32)> {
        let mut generators = Vec::new();
        
        let multiplicative_group = self.get_multiplicative_group();

        for i in 0..self.n {
            for &j in &multiplicative_group {
                if self.is_generator(i, j) {
                    generators.push((i, j));
                }
            }
        }

        generators
    }

    fn get_multiplicative_group(&self) -> HashSet<u32> {
        let mut set = HashSet::new();
        for i in 1..self.m {
            if gcd(i, self.m) == 1 {
                set.insert(i);
            }
        }
        set
    }

    fn is_generator(&self, a: u32, b: u32) -> bool {
        let mut visited = vec![vec![false; self.m as usize]; self.n as usize];
        let mut count = 0;

        let mut x = 0;
        let mut y = 1;

        loop {
            if visited[x as usize][y as usize] {
                break;
            }

            visited[x as usize][y as usize] = true;
            count += 1;

            x = (x + a) % self.n;
            y = (y * b) % self.m;
        }

        count == (self.n * self.get_multiplicative_group().len() as u32)
    }
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let group = ZnZmStar::new(5, 10);
    let generators = group.find_generators();

    println!("Generators of Z4 x Z10^*: {:?}", generators);
}

