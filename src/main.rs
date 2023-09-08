use std::io::stdin;

use rand::random;

struct Field {
    n: usize,
    m: usize,
    data: Vec<Vec<bool>>,
}

impl Field {
    fn new_random(n: usize, m: usize) -> Self {
        let mut data = vec![vec![false; m]; n];
        for row in data.iter_mut() {
            for item in row.iter_mut() {
                *item = random();
            }
        }
        Field { n, m, data }
    }
}

impl Field {
    fn print(&self) {
        for row in self.data.iter() {
            for item in row.iter() {
                print!("{}", if *item { '#' } else { '.' });
            }
            println!();
        }
    }
    fn alive_neighbors(&self, x: usize, y: usize) -> usize {
        let mut result = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                fn wrap(i: usize, di: isize, n: usize) -> usize {
                    ((i as isize + di + n as isize) as usize) % n
                }

                let nx = wrap(x, dx, self.n);
                let ny = wrap(y, dy, self.m);
                if self.data[nx][ny] {
                    result += 1;
                }
            }
        }
        result
    }

    fn next(&mut self) {
        let mut data = self.data.clone();
        for x in 0..self.n {
            for y in 0..self.m {
                let neighbors = self.alive_neighbors(x, y);
                let alive = data[x][y];
                if !alive && neighbors == 3 {
                    data[x][y] = true;
                } else if alive && !matches!(neighbors, 2 | 3) {
                    data[x][y] = false;
                }
            }
        }
        self.data = data;
    }
}

fn main() {
    let mut field = Field::new_random(12, 25);
    field.print();
    loop {
        field.print();
        field.next();
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
    }
}
