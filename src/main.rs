use sdl2::pixels::{self, Color};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::f64::consts::PI;
use std::time;

struct circle{
    angle: f64,
    radius:f64,
    inc : f64,
}
impl circle{
    fn getPos(&mut self, pixel : sdl2::rect::Point) -> sdl2::rect::Point {
        let mut newPixel = pixel.clone();
        
        newPixel.x = (self.angle.cos()*self.radius )as i32+400;
        newPixel.y = (self.angle.sin()*self.radius )as i32+400;

        self.angle += self.inc;
        newPixel
    }
}
fn main() {
    println!("Hello, world!");
    let sdl_context = sdl2::init().unwrap();
    let mut event_queue = sdl_context.event_pump().unwrap();

    let video_subsys = sdl_context.video().unwrap();
    let window = video_subsys.window("rusty spirogram", 800, 800).position_centered().build().unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();
    let mut pixel = sdl2::rect::Point::new(0,0);  
    let mut arr_circle:Vec<circle> = Vec::new();
    arr_circle.push(circle{angle:0.0,radius:25.0, inc:0.01});
    arr_circle.push(circle{angle:0.0,radius:30.0, inc:0.01});
    arr_circle.push(circle{angle:0.0,radius: 50.0, inc:0.1});
    arr_circle.push(circle{angle:0.0,radius: 55.0, inc:0.0001});

    // let mut circle1:circle = circle{angle:0.0,radius:25.0, inc:0.1};
    // let mut circle2:circle = circle{angle:0.0,radius:25.0, inc:0.1};
    let mut offset:i32 =( (arr_circle.len()-1)*400) as i32;
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    offset = if arr_circle.len() ==1 {400} else{offset};

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
        print!("{:?} x, {:?} y\n",pixel.x, pixel.y);

        // pixel.x = (circle1.angle.cos()*circle1.radius )as i32+400;
        // pixel.y = (circle1.angle.sin()*circle1.radius )as i32+400;
        pixel.x=0;
        pixel.y=0;

        for Circle in arr_circle.iter_mut(){
            let lastPosx = pixel.x;
            let lastPosy = pixel.y;
            pixel = Circle.getPos(pixel);
            pixel.x +=lastPosx;
            pixel.y +=lastPosy;

        }
        pixel.x -=offset;
        pixel.y -=offset;
        canvas.draw_point(pixel);
        canvas.present();
    }

}
