use std::slice::Iter;

const ALIVE: u8 = 0x80;
const ALIVE_LAST_TICK: u8 = 0x40;

#[derive(Debug, Clone)]
pub struct Game {
	universe: Vec<Vec<u8>>,
	height: usize,
	width: usize,
	population: usize,
	generation: usize
}

impl Game {
	pub fn new(width: usize, height: usize) -> Game {
		let uni = vec![vec![0; width]; height];
		Game { universe: uni, height: height, width: width, population: 0, generation: 0 }
	}

	pub fn set_alive(&mut self, x: usize, y: usize) {
		if (self.universe[y][x] & ALIVE) == 0 {
			self.universe[y][x] = ALIVE;
			self.population = self.population + 1;
		}
	}

	pub fn set_dead(&mut self, x: usize, y: usize) {
		if (self.universe[y][x] & ALIVE) > 0 {
			self.universe[y][x] = self.universe[y][x] >> 1;
			self.population = self.population - 1;
		}
	}

	pub fn value(&self, x: usize, y: usize) -> u8 {
		self.universe[y][x]
	}

	pub fn iter(&self) -> Iter<Vec<u8>> {
		self.universe.iter()
	}

	pub fn population(&self) -> usize {
		self.population
	}

	pub fn generation(&self) -> usize {
		self.generation
	}

	pub fn tick(&mut self) {
		let mut changes: Vec<(usize, usize, u8)> = Vec::new();
		for row in self.universe.iter_mut() {
	        for item in row.iter_mut() {
	        	*item = *item >> 1;
	        }
	    }

	    for (i, row) in self.universe.iter().enumerate() {
	        for (j, col) in row.iter().enumerate() {
	        	
	            let neighbors = self.alive_neighbors(i, j);
	            if (*col & ALIVE_LAST_TICK) > 0 {
	                if neighbors >= 2 && neighbors <= 3{
	                    changes.push((i, j, ALIVE | *col));
	                } 
	            } else {
	                if neighbors == 3 {
	                    changes.push((i, j, ALIVE | *col));
	                }
	            }
	        }
	    }

	    for (i, j, value) in changes {
	    	self.universe[i][j] = value;
	    	if (self.universe[i][j] & ALIVE) > 0 { 
	    		self.population = self.population + 1;
	    	} else {
	    		self.population = self.population - 1;
	    	}
	    }
	}

	fn alive_neighbors(&self, i: usize, j: usize) -> usize {
	    let x = self.width;
	    let y = self.height;
	    let mut cnt = 0;
	    //top left
	    if i > 0 && j > 0 && (self.universe[i-1][j-1] & ALIVE_LAST_TICK) > 0 {
	        cnt += 1;
	    } 
	    if i > 0 && (self.universe[i-1][j] & ALIVE_LAST_TICK) > 0 {
	        cnt += 1;
	    }
	    if i > 0 && j < x -1 && (self.universe[i-1][j+1] & ALIVE_LAST_TICK) > 0 {
	        cnt += 1;
	    }

	    //bottom left
	    if i < y - 1 && j > 0 && (self.universe[i+1][j-1] & ALIVE_LAST_TICK) > 0 {
	        cnt += 1;
	    } 
	    if i < y - 1 && (self.universe[i+1][j] & ALIVE_LAST_TICK) > 0 {
	        cnt += 1;
	    }
	    if i < y - 1 && j < x -1 && (self.universe[i+1][j+1] & ALIVE_LAST_TICK) > 0 {
	        cnt += 1;
	    }

	    //mid left
	    if j > 0 && (self.universe[i][j-1] & ALIVE_LAST_TICK) > 0 {
	        cnt += 1;
	    }
	    if j < x - 1 && (self.universe[i][j+1] & ALIVE_LAST_TICK) > 0 {
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

