extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use graphics::*;
use crate::Direction::{Right, Left};
use piston::{ButtonEvent, ButtonState, Button, Key};
use piston::input::keyboard::Key::R;

#[derive(Clone, PartialEq)]
enum Direction {
    Right, Left, Up, Down
}

struct Game {
    gl: GlGraphics,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];


        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c
                .transform
                .trans(x, y)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
    }
}

struct Snake {
    pos_x: i32,
    pos_y: i32,
    gl: GlGraphics,
    dir: Direction
}

impl Snake {
    fn render(&mut self, args: &RenderArgs) {

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(self.pos_x as f64,
        self.pos_y as f64, 20_f64);

        let (x, y) = (self.pos_x as f64, self.pos_y as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.

            let transform = c
                .transform
                .trans(x, y)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self) {

        match self.dir {
            Direction::Right => self.pos_x += 1,
            Direction::Left => self.pos_x += 1,
            Direction::Up => self.pos_y -= 1,
            Direction::Down => self.pos_y += 1,
        }
    }

    fn pressed(&mut self, btn: &Button) {
        let last_direction = self.dir.clone();

        self.dir = match btn {
            &Button::Keyboard(Key::Up)
                if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down)
                if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left)
                if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right)
                if last_direction != Direction::Left => Direction::Right,
            _ => last_direction
        }
    }
}


fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("snake game", [800, 800])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut snake = Snake{
        pos_x: 200,
        pos_y: 200,
        gl: GlGraphics::new(opengl),
        dir: Direction::Up
    };

    let mut game = Game {
        gl: GlGraphics::new(opengl)
    };

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
            snake.render(&args);

        }
        if let Some(args) = e.update_args() {
            snake.update();
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                snake.pressed(&k.button)
            }
        }
    }
}