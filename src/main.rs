extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, ReleaseEvent, PressEvent, Key, Button};
use piston::window::WindowSettings;
use rand::Rng;
use std::f64::consts::PI;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    p1_x : f64,
    p1_y : f64,
    p2_x : f64,
    p2_y : f64,
    ball_x : f64,
    ball_y : f64,
    ball_x_speed : f64,
    ball_y_speed : f64,
    width : f64,
    height : f64,
    ran : rand::rngs::ThreadRng,
    hits : f64
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const I_DUNNU: [f32; 4] = [1.0, 1.0, 0.0, 1.0];

        // Okej så deklarera alla sorters typer av kombinationer av former och färger du behöver
        // här först
        let square = rectangle::square(0.0, 0.0, 50.0);
        let paddle = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let p1_x = self.p1_x;
        let p1_y = self.p1_y;
        let p2_x = self.p2_x;
        let p2_y = self.p2_y;
        let ball_x = self.ball_x;
        let hits = self.hits;
        let ball_y = self.ball_y;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);
            // Draw a box rotating around the middle of the screen.
            for i in 0..6000 {
                let angle : f64 = i as f64 * PI * 2.0 / 300.0;
                let rad_wave = angle*6.0 + rotation;
                let rad_amp = (rotation/5.0).cos().abs()*0.12*(hits*0.03);
                let radius : f64 = ((rad_wave.cos()*rad_amp) + 1.0)*(i as f64 / 3.0);
                let offset_x : f64 = angle.cos() * radius;
                let offset_y : f64 = angle.sin() * radius; 

                let transform = c
                    .transform
                    .trans(offset_x, offset_y)
                    .rot_rad(rotation/2.0)
                    .trans(-25.0, -25.0);


                rectangle(RED, square, transform, gl);
            }
            
            // Draw the motherfucking paddle motherfucker or something i am moonshot i could loose
            // you i am moon.,SHOT grow up to be, be a debaser
            let trans_rights = c
                    .transform
                    .trans(p1_x, p1_y)
                    .rot_rad(rotation*(0.5+hits*0.05))
                    .trans(-25.0, -25.0);
            rectangle(BLUE, paddle, trans_rights, gl);
            let trans_rights = c
                    .transform
                    .trans(p2_x, p2_y)
                    .rot_rad(rotation*(0.5+(hits*0.05)))
                    .trans(-25.0, -25.0);
            rectangle(BLUE, paddle, trans_rights, gl);

            let trans_rights = c
                    .transform
                    .trans(ball_x, ball_y)
                    .rot_rad(rotation*(1.0+ hits*0.1))
                    .trans(-25.0, -25.0);
            rectangle(BLUE, paddle, trans_rights, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs, p1_up : bool, p1_down : bool, p2_up : bool, p2_down : bool) {
        // Rotate 2 radians per second.
        self.rotation += 5.0 * args.dt;
        if p1_up {
            self.p1_y -= 600.0 * args.dt;
            if self.p1_y < 0.0 {
                self.p1_y += 600.0 * args.dt;
            }
        }
        if p1_down {
            self.p1_y += 600.0 * args.dt;
            if self.p1_y > self.height {
                self.p1_y -= 600.0 * args.dt;
            }
        }
        if p2_up {
            self.p2_y -= 600.0 * args.dt;
            if self.p2_y < 0.0 {
                self.p2_y += 600.0 * args.dt;
            }
        }
        if p2_down {
            self.p2_y += 600.0 * args.dt;
            if self.p2_y > self.height {
                self.p2_y -= 600.0 * args.dt;
            }
        }
        
        let x_dir = if self.ball_x_speed < 0.0 { -1.0 } else { 1.0 };
        let y_dir = if self.ball_y_speed < 0.0 { -1.0 } else { 1.0 };
        self.ball_x += (self.ball_x_speed + x_dir*10.0*self.hits) * args.dt;
        self.ball_y += (self.ball_y_speed + y_dir*10.0*self.hits) * args.dt;
        //println!("{} > {} = {}",self.ball_y, self.height,self.ball_y > self.height);
        //println!("{} < {} = {}",self.ball_y, 0,self.ball_y < 0.0);
        

        if self.ball_y > self.height || self.ball_y < 0.0 {
            self.ball_y -= self.ball_y_speed * args.dt;
            self.ball_y_speed *= -1.0;
        }
        if self.ball_x < -10.0 || self.ball_x > self.width + 10.0 {
            let rand_angle = self.ran.gen::<f64>() % 2.0*PI;
            let max_speed = (self.ball_x_speed*self.ball_x_speed + self.ball_y_speed*self.ball_y_speed).sqrt();
            self.ball_x = self.width / 2.0;
            self.ball_y = self.height / 2.0;
            self.ball_x_speed = max_speed * rand_angle.cos();
            self.ball_y_speed = max_speed * rand_angle.sin();
            self.hits = 0.0;
        }
        let hit_rad = 60.0;
        if self.ball_x_speed < 0.0 {
            let rad = ((self.ball_x - self.p1_x)*(self.ball_x - self.p1_x) + (self.ball_y - self.p1_y)*(self.ball_y - self.p1_y)).sqrt();
            if rad < hit_rad {
                self.ball_x_speed *= -1.0;
                self.hits+=1.0;
            }
        } else {
            let rad = ((self.ball_x - self.p2_x)*(self.ball_x - self.p2_x) + (self.ball_y - self.p2_y)*(self.ball_y - self.p2_y)).sqrt();
            if rad < hit_rad {
                self.ball_x_speed *= -1.0;
                self.hits+=1.0;
            }
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
    let width = 1600;
    let height = 800;
    let x_padding = 100.0;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [width, height])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        p1_x : x_padding,
        p1_y : height as f64 / 2.0,
        p2_x : width as f64 - x_padding,
        p2_y : height as f64 / 2.0,
        ball_x : 600.0,
        ball_y : 400.0,
        ball_x_speed : 200.0,
        ball_y_speed : 200.0,
        width : width as f64,
        height : height as f64,
        ran : rand::thread_rng(),
        hits : 0.0
    };

    let mut p1_up : bool = false;
    let mut p1_down : bool = false;
    let mut p2_up : bool = false;
    let mut p2_down : bool = false;
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

        if let Some(button) = e.press_args() {
            if button == Button::Keyboard(Key::W) {
                p1_up = true;
            }
            if button == Button::Keyboard(Key::S) {
                p1_down = true;
            }
            if button == Button::Keyboard(Key::Up) {
                p2_up = true;
            }
            if button == Button::Keyboard(Key::Down) {
                p2_down = true;
            }
        }

        if let Some(button) = e.release_args() {
            if button == Button::Keyboard(Key::W) {
                p1_up = false;
            }
            if button == Button::Keyboard(Key::S) {
                p1_down = false;
            }
            if button == Button::Keyboard(Key::Up) {
                p2_up = false;
            }
            if button == Button::Keyboard(Key::Down) {
                p2_down = false;
            }
        }


        if let Some(args) = e.render_args() {
            app.render(&args);
        }
       
         
        if let Some(args) = e.update_args() {
            app.update(&args, p1_up, p1_down, p2_up, p2_down);
        }
    }
}
