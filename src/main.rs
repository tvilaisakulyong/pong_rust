extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::process;
use glutin_window::GlutinWindow as Window;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use piston::input::{RenderArgs};
use opengl_graphics::{GlGraphics, OpenGL};

pub struct App {
    gl: GlGraphics,
    left_pos: f64,
    left_vel: f64,
    left_score: i32,
    right_pos: f64,
    right_vel: f64,
    right_score: i32,
    ball_x: f64,
    ball_y: f64,
    ball_vx: f64,
    ball_vy: f64,

}

//TODO: score board, grceful win reset.
impl App {

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BACKGROUND: [f32; 4] = [0.0, 0.5, 0.5, 1.0];
        const FOREGROUND: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let left = rectangle::square(0.0, 0.0, 50.0);
        let left_pos = self.left_pos;

        let right = rectangle::square(0.0, 0.0, 50.0);
        let right_pos = self.right_pos;

        let ball = rectangle::square(0.0, 0.0, 10.0);
        let ball_x = self.ball_x;
        let ball_y = self.ball_y;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);
            // let ball = rectangle::square(0.0, 0.0, 10.0);

            //render left paddle (left side, move right 40 px)
            rectangle(FOREGROUND, left, c.transform.trans(-40.0, left_pos), gl);

            //render right paddle (to the right edge of the sceeen, moved in by 10 px)
            rectangle(FOREGROUND, right, c.transform.trans(args.width as f64 - 10.0, right_pos), gl);

            //render ball at the middle of the screen
            rectangle(FOREGROUND, ball, c.transform.trans(ball_x, ball_y), gl);
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {

        const BALL_X_CENTER: f64 = 256.0;
        const BALL_Y_CENTER: f64 = 171.0;

        //update paddles positions
        //check paddle against screen y edge. If adding velocity will move it over, stop and reset.
        if (self.left_pos > 1.0 && self.left_vel == -1.0) ||
           (self.left_pos < 291.0 && self.left_vel == 1.0) {
                self.left_pos += self.left_vel;
           }

        if (self.right_pos > 1.0 && self.right_vel == -1.0) ||
           (self.right_pos < 291.0 && self.right_vel == 1.0) {
               self.right_pos += self.right_vel;
           }

        //update ball position
        self.ball_x += self.ball_vx;
        self.ball_y += self.ball_vy;

        //collision detection with screen, x direction
        if self.ball_x > 502.0 {
            //reverse x velocity
            self.ball_vx = -self.ball_vx;
            // right side, check ball y position against right paddle y position
            if self.ball_y < self.right_pos || self.ball_y > self.right_pos + 50.0 {
                //ball position passed right paddle, score increase for left player and reset ball.
                self.left_score += 1;
                if self.left_score > 5 {
                    println!("Left Player wins!");
                    process::exit(0);
                }
                self.ball_x = BALL_X_CENTER;
                self.ball_y = BALL_Y_CENTER;
            }
        } else if self.ball_x < 1.0 {
            self.ball_vx = -self.ball_vx;
            // left side, check ball y position against left paddle y position
            if self.ball_y < self.left_pos || self.ball_y > self.left_pos + 50.0 {
                //ball position passed left paddle, score increase for right player and reset ball.
                self.right_score +=1;
                if self.right_score > 5 {
                    println!("Right Player wins!");
                    process::exit(0);
                }
                self.ball_x = BALL_X_CENTER;
                self.ball_y = BALL_Y_CENTER;
            }
        }

        //collision detection with screen, y direction,
        //reverse thruster!
        if self.ball_y > 342.0 || self.ball_y < 1.0 {
            self.ball_vy = - self.ball_vy;
        }
    }

    fn press(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            match key {
                Key::Up => {
                    self.right_vel = -1.0;
                }
                Key::Down => {
                    self.right_vel = 1.0;
                }
                Key::W => {
                    self.left_vel = -1.0;
                }
                Key::S => {
                    self.left_vel = 1.0;
                }
                _ => {}
            }
        }
    }

    fn release(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            match key {
                Key::Up => {
                    self.right_vel = 0.0;
                }
                Key::Down => {
                    self.right_vel = 0.0;
                }
                Key::W => {
                    self.left_vel = 0.0;
                }
                Key::S => {
                    self.left_vel = 0.0;
                }
                _ => {}
            }
        }
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
        left_pos:171.0,
        left_vel:0.0,
        left_score:0,
        right_pos:171.0,
        right_vel:0.0,
        right_score:0,
        ball_x:256.0,
        ball_y:171.0,
        ball_vx:1.0,
        ball_vy:1.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window){
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(b) = e.press_args() {
            app.press(&b);
        }

        if let Some(b) = e.release_args() {
            app.release(&b);
        }
    }
}
