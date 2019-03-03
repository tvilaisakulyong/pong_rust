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
    gl: GlGraphics,
    left_pos: i32,
    right_pos:i32,
}

impl App {

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BACKGROUND: [f32; 4] = [0.0, 0.5, 0.5, 1.0];
        const FOREGROUND: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let left = rectangle::square(0.0, 0.0, 50.0);
        let left_pos = self.left_pos as f64;

        let right = rectangle::square(0.0, 0.0, 50.0);
        let right_pos = self.right_pos as f64;

        let ball = rectangle::square(0.0, 0.0, 10.0);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);
            // let ball = rectangle::square(0.0, 0.0, 10.0);

            //render left paddle (left side, move right 40 px)
            rectangle(FOREGROUND, left, c.transform.trans(-40.0, left_pos), gl);

            //render right paddle (to the right edge of the sceeen, moved in by 10 px)
            rectangle(FOREGROUND, right, c.transform.trans(args.width as f64 - 10.0, right_pos), gl);

            //render ball at the middle of the screen
            rectangle(FOREGROUND, ball, c.transform.trans(0.0, 0.0), gl);
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
        gl:GlGraphics::new(opengl),
        left_pos:1,
        right_pos:1
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
