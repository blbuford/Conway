extern crate piston_window;
extern crate find_folder;
extern crate fps_counter;
extern crate conway;

use piston_window::*;
use piston_window::rectangle::square;

fn main() {
    const PIXEL_SIZE: f64 = 10.0;
    const SCREEN_WIDTH: f64 = 1200.0;
    const SCREEN_HEIGHT: f64 = 800.0;
    let mut window: PistonWindow = WindowSettings::new(
            "Conway's Rusty Life",
            [SCREEN_WIDTH, SCREEN_HEIGHT]
        )
        .exit_on_esc(true)
        .opengl(OpenGL::V2_1) // Set a different OpenGl version
        .build()
        .unwrap();

    // let assets = find_folder::Search::ParentsThenKids(3, 3)
    //     .for_folder("assets").unwrap();
    // println!("{:?}", assets);
    // let ref font = assets.join("FiraSans-Regular.ttf");
    // let factory = window.factory.clone();
    // let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

    let mut cw = conway::Game::new(
        (SCREEN_WIDTH / PIXEL_SIZE) as usize, 
        (SCREEN_HEIGHT / PIXEL_SIZE) as usize);

    cw.set_alive(3, 4);
    cw.set_alive(3, 5);
    cw.set_alive(2, 5);

    cw.set_alive(14, 1);
    cw.set_alive(15, 1);
    cw.set_alive(13, 2);
    cw.set_alive(12, 3);
    cw.set_alive(12, 4);
    cw.set_alive(12, 5);
    cw.set_alive(13, 6);
    cw.set_alive(14, 7);
    cw.set_alive(15, 7);

    cw.set_alive(26, 3);
    cw.set_alive(26, 4);
    cw.set_alive(25, 4);

    cw.set_alive(26, 8);
    cw.set_alive(25, 8);
    cw.set_alive(26, 9);

    cw.set_alive(28, 5);
    cw.set_alive(28, 6);
    cw.set_alive(28, 7);

    cw.set_alive(29, 5);
    cw.set_alive(29, 6);
    cw.set_alive(29, 7);

    cw.set_alive(30, 6);

    cw.set_alive(37, 6);
    cw.set_alive(37, 7);
    cw.set_alive(36, 7);
    window.set_lazy(true);
    let mut fps = fps_counter::FPSCounter::new();
    let mut settings = EventSettings::new();
    settings.max_fps = 60;
    let mut events = Events::new(settings);

    while let Some(e) = events.next(&mut window) {
        
        window.draw_2d(&e, |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            
            let alive = cw.alive_pairs();
            for (y, x) in alive.iter() {
                let rectangle = Rectangle::new(color::WHITE);
                let dims = square(*x as f64 * PIXEL_SIZE, *y as f64 * PIXEL_SIZE, PIXEL_SIZE);
                rectangle.draw(dims, &c.draw_state, c.transform, g);
            }
            cw.tick();
            println!("FPS: {:?}", fps.tick());
        });
        
    }
}
