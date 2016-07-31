extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::ops::{Add, AddAssign};
use std::f64;

pub struct App {
    gl: GlGraphics,
    rotation: f64,
}

struct Point {
    x: i32,
    y: i32,
}


impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32;4] = [0.333, 0.847, 0.654, 1.0];
        const RED: [f32; 4] = [0.0823, 0.435, 0.262, 1.0];

        let square = rectangle::square(0.0, 0.0, 10.0);
        let rotation = self.rotation;

        let (x, y) = ((args.width / 2 ) as f64,
                      (args.height / 2 ) as f64);
        
        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);

            
            for n in 0..8{
                let var = n as f64;
               let float = f64::consts::PI/2.0+rotation+var;
                let transform = c.transform.trans(50.0+var*20.0+float.sin()*5.0, y+20.0+float.sin()*30.0)
                                           .trans(-25.0, -25.0);
                ellipse(RED, square, transform, gl);
            }
            //ellipse(RED, square, transform2, gl);
            //ellipse(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // let mut rotation = self.rotation;
        // rotation += 0.001;
        // self.rotation = rotation;
        self.rotation += 0.03;
    }
}


fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
            "spinning-circle",
            [200,200]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.003,
    };

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
