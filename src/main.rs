extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use piston::input::{RenderArgs};
use opengl_graphics::{GlGraphics, OpenGL};

pub struct App {
    gl: GlGraphics
}

impl App {

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BACKGROUND: [f32; 4] = [0.0, 0.5, 0.5, 1.0];
        const FOREGROUND: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);
            let left = rectangle::square(0.0, 0.0, 50.0);
            rectangle(FOREGROUND, left, c.transform.trans(-40.0, 1.0), gl);
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    //Create a Glutin window
    let mut window: Window = WindowSettings::new("Pong", [512, 342])
                                .opengl(opengl)
                                .exit_on_esc(true)
                                .build()
                                .unwrap();

    let mut app = App {
        gl:GlGraphics::new(opengl)
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window){
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
