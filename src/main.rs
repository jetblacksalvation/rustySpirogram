use sdl2::pixels::{self, Color};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::f64::consts::PI;
struct circle{
    angle: f64,
    radius:f64,
    inc : f64,
}
impl circle{
    fn getPos(&mut self, pixel :&mut sdl2::rect::Point){
        pixel.x = (self.angle.cos()*self.radius )as i32+400;
        pixel.y = (self.angle.sin()*self.radius )as i32+400;

        self.angle += self.inc;

    }
}
fn main() {
    println!("Hello, world!");
    let sdl_context = sdl2::init().unwrap();
    let mut event_queue = sdl_context.event_pump().unwrap();

    let video_subsys = sdl_context.video().unwrap();
    let window = video_subsys.window("MandelBrot", 800, 800).position_centered().build().unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();
    let mut pixel = sdl2::rect::Point::new(0,0);  
    let mut arr_circle:Vec<circle> = Vec::new();
    arr_circle.push(circle{angle:0.0,radius:25.0, inc:0.1});
    arr_circle.push(circle{angle:0.0,radius:8.0, inc:0.4});

    // let mut circle1:circle = circle{angle:0.0,radius:25.0, inc:0.1};
    // let mut circle2:circle = circle{angle:0.0,radius:25.0, inc:0.1};

    'running : loop{
        for event in event_queue.poll_iter(){
            match event{
                Event::Quit { .. } |
                Event::KeyDown { keycode:Some(Keycode::Escape),..} => {break 'running},
                _ =>{}
            }
        }
        // canvas.set_draw_color(Color::RGB(0,0,0));
        // canvas.clear();
        // circle1.angle += circle1.inc;
        // // circle1.angle %=PI*2.0;
        print!("{:?}x , {:?} y\n",pixel.x, pixel.y);
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        // pixel.x = (circle1.angle.cos()*circle1.radius )as i32+400;
        // pixel.y = (circle1.angle.sin()*circle1.radius )as i32+400;
        pixel.x =0;
        pixel.y = 0;

        for mut Circle in arr_circle.iter_mut(){
            let lastPosx = pixel.x;
            let lastPosy = pixel.y;
            Circle.getPos(&mut pixel);
            pixel.x +=lastPosx;
            pixel.y +=lastPosy;

        }
        pixel.x -=400;
        pixel.y -=400;
        canvas.draw_point(pixel);
        canvas.present();
    }

}
