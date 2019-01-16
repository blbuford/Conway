use std::slice::Iter;

#[derive(Debug, Clone)]
pub struct Game {
	universe: Vec<Vec<usize>>,
	last_universe: Vec<Vec<usize>>
}

impl Game {
	pub fn new(width: usize, height: usize) -> Game {
		let uni = vec![vec![0; width]; height];
		let last_uni = uni.clone();
		Game { universe: uni, last_universe: last_uni }
	}

	pub fn set_alive(&mut self, x: usize, y: usize) {
		self.universe[y][x] = 1;
	}

	pub fn set_dead(&mut self, x: usize, y: usize) {
		self.universe[y][x] = 0;
	}

	pub fn value(&self, x: usize, y: usize) -> usize {
		self.universe[y][x]
	}

	pub fn alive_pairs(&self) -> Vec<(usize, usize)> {
		let mut alive = Vec::new();
		for (y, row) in self.universe.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if *col == 1 {
                    alive.push((y,x));
                }
            }
        }
        alive
	}

	pub fn tick(&mut self) {
	    self.last_universe = self.universe.clone();
	    for (i, row) in self.last_universe.iter().enumerate() {
	        for (j, col) in row.iter().enumerate() {
	            let neighbors = self.alive_neighbors(i, j);
	            if *col == 1 {
	                if neighbors < 2 || neighbors > 3{
	                    self.universe[i][j] = 0;
	                } 
	            } else {
	                if neighbors == 3 {
	                    self.universe[i][j] = 1;
	                }
	            }
	        }
	    }
	}

	fn alive_neighbors(&self, i: usize, j: usize) -> usize {
	    let x = self.last_universe[0].len();
	    let y = self.last_universe.len();
	    let mut cnt = 0;
	    //top left
	    if i > 0 && j > 0 && self.last_universe[i-1][j-1] == 1 {
	        cnt += 1;
	    } 
	    if i > 0 && self.last_universe[i-1][j] == 1 {
	        cnt += 1;
	    }
	    if i > 0 && j < x -1 && self.last_universe[i-1][j+1] == 1 {
	        cnt += 1;
	    }

	    //bottom left
	    if i < y - 1 && j > 0 && self.last_universe[i+1][j-1] == 1 {
	        cnt += 1;
	    } 
	    if i < y - 1 && self.last_universe[i+1][j] == 1 {
	        cnt += 1;
	    }
	    if i < y - 1 && j < x -1 && self.last_universe[i+1][j+1] == 1 {
	        cnt += 1;
	    }

	    //mid left
	    if j > 0 && self.last_universe[i][j-1] == 1 {
	        cnt += 1;
	    }
	    if j < x - 1 && self.last_universe[i][j+1] == 1 {
	        cnt += 1;
	    }
	    cnt
	}

}

// pub struct GameIter<'a> {
// 	inner: Iter<Vec<usize>>
// }

// impl<'a> Iterator for GameIter<'a> {
// 	type Item = &'a Vec<usize>;

// 	fn next(&mut self) -> Option<Vec<usize>> {
// 		self.inner.next()
// 	}

// }

