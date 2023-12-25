mod math;
mod render;
mod shapes;

use math::*;
use render::*;
use shapes::*;

fn main() {
    let fields: Vec<Box<dyn SignedDistance>> = vec![Box::new(Sphere {
        pos: Vec3 {
            x: 100.0,
            y: -25.0,
            z: 10.0,
        },
        rad: 50.0,
    })];

    let xres = 1280;
    let yres = 720;

    let pixels = render(fields, (xres, yres), (16.0 / 4.0, 9.0 / 4.0));

    let mut out_image = format!("P3\n{xres} {yres}\n255\n");

    for y in 0..yres {
        for x in 0..xres {
            let val = pixels[y * xres + x] as usize * 255;
            out_image.push_str(&format!("{val} {val} {val}\n"))
        }
    }

    println!("{out_image}");
}
