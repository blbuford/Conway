extern crate piston_window;
extern crate conway;

use piston_window::*;
use piston_window::rectangle::{square, Border};
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};

fn main() {
    // Sizes
    const PIXEL_SIZE: f64 = 10.0;
    const SCREEN_WIDTH: f64 = 1200.0;
    const SCREEN_HEIGHT: f64 = 800.0;

    // Colors
    let color_alive: [f32; 4] = color::hex("58CCED");
    let color_alive_border = Border{color: color::hex("0d5d74"), radius: 1.0};
    let color_dead: [f32; 4] = color::hex("000000");

    let mut window: PistonWindow = WindowSettings::new(
            "Conway's Rusty Life",
            [SCREEN_WIDTH, SCREEN_HEIGHT]
        )
        .exit_on_esc(true)
        .opengl(OpenGL::V2_1) // Set a different OpenGl version
        .build()
        .unwrap();

    

    let mut game = conway::Game::new(
        (SCREEN_WIDTH / PIXEL_SIZE) as usize, 
        (SCREEN_HEIGHT / PIXEL_SIZE) as usize);

    let mut pattern = read_ascii(File::open("patterns/b.txt").unwrap()).unwrap();
    pattern = center_pattern(pattern, (SCREEN_WIDTH / PIXEL_SIZE) as usize, (SCREEN_HEIGHT / PIXEL_SIZE) as usize);
    for (x, y) in pattern {
        game.set_alive(x,y);
    }
    
    window.set_lazy(true);
    let mut settings = EventSettings::new();
    settings.max_fps = 10;
    let mut events = Events::new(settings);

    while let Some(e) = events.next(&mut window) {

        window.draw_2d(&e, |c, g| {
                
                clear(color_dead, g);

                let (dimx, dimy) = game.dimensions();
                for x in 0..dimx {
                    for y in 0..dimy {
                        if game.is_alive(x, y) {
                            let rectangle = Rectangle::new(color_alive).border(color_alive_border);
                                
                            let dims = square(
                                            (x as f64) * PIXEL_SIZE, 
                                            (y as f64) * PIXEL_SIZE,
                                            PIXEL_SIZE);
                            rectangle.draw(dims, &c.draw_state, c.transform, g);
                        }
                    }
                }

                game.tick();
            });

    }
}

fn read_ascii<R: Read>(io: R) -> Result<Vec<(usize, usize)>, Error> {
    let br = BufReader::new(io);
    let mut coords = vec![];
    for (y, line) in br.lines().enumerate() {
        let line = line?;
        let line = line.trim();
        for (x, char) in line.chars().enumerate() {
            if char == 'O' {
                coords.push((x,y));
            }
        }
    }
    Ok(coords)
}

fn center_pattern( pat: Vec<(usize, usize)>, width:usize, height:usize ) -> Vec<(usize, usize)> {
    let centerw = width / 2;
    let centerh = height / 2;

    let mut minx = width;
    let mut miny = height;
    let mut maxx = 0;
    let mut maxy = 0;

    let mut centered: Vec<(usize, usize)> = Vec::with_capacity(pat.len());
    for (x,y) in &pat {
        if *x < minx {
            minx = *x;
        }

        if *x > maxx {
            maxx = *x;
        }

        if *y < miny {
            miny = *y;
        }

        if *y > maxy {
            maxy = *y;
        }
    }

    let shiftx = centerw - ((maxx - minx) / 2);
    let shifty = centerh - ((maxy - miny) / 2);

    for (x, y) in pat.iter() {
        centered.push((*x + shiftx, *y + shifty));
    }

    centered

}






