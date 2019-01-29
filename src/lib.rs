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

	pub fn is_alive(&self, x: usize, y: usize ) -> bool {
		(self.universe[y][x] & ALIVE) > 0
	}

	pub fn dimensions(&self) -> (usize, usize) {

		(self.width, self.height)
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
		self.generation = self.generation + 1;
		let mut changes: Vec<(usize, usize, u8)> = Vec::new();

		// Shift last generation to the next bit down
		for row in self.universe.iter_mut() {
	        for item in row.iter_mut() {
	        	*item = *item >> 1;
	        }
	    }

	    // Calculate the rules that need to be applied from last generation
	    for (i, row) in self.universe.iter().enumerate() {
	        for (j, col) in row.iter().enumerate() {
	        	
	            let neighbors = self.alive_neighbors(i, j);
	            if (*col & ALIVE_LAST_TICK) > 0 {
	                if neighbors >= 2 && neighbors <= 3{
	                    changes.push((i, j, ALIVE | *col));
	                } else {
	                	self.population = self.population - 1;
	                }
	            } else {
	                if neighbors == 3 {
	                    changes.push((i, j, ALIVE | *col));
	                    self.population = self.population + 1;
	                } 
	            }
	        }
	    }

	    // Apply the changes to update this generation
	    for (i, j, value) in changes {
	    	self.universe[i][j] = value;

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
