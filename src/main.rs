mod math;
mod render;
mod shapes;

use math::*;
use render::*;
use shapes::*;

fn main() {
    let fields: Vec<Box<dyn SignedDistance>> = vec![
        Box::new(Sphere {
            pos: Vec3 {
                x: 100.0,
                y: -25.0,
                z: 10.0,
            },
            radius: 50.0,
        }),
        Box::new(Intersection {
            fields: vec![
                Box::new(Sphere {
                    pos: Vec3 {
                        x: 50.0,
                        y: 40.0,
                        z: -30.0,
                    },
                    radius: 20.0,
                }),
                Box::new(Sphere {
                    pos: Vec3 {
                        x: 60.0,
                        y: 20.0,
                        z: -30.0,
                    },
                    radius: 30.0,
                }),
            ],
        }),
        Box::new(Difference {
            field1: Box::new(Sphere {
                pos: Vec3 {
                    x: 80.0,
                    y: 30.0,
                    z: 80.0,
                },
                radius: 30.0,
            }),
            field2: Box::new(Sphere {
                pos: Vec3 {
                    x: 70.0,
                    y: 10.0,
                    z: 60.0,
                },
                radius: 30.0,
            }),
        }),
    ];

    let xres = 1280;
    let yres = 720;

    let start_time = std::time::Instant::now();
    let pixels = render(fields, (xres, yres), (16.0 / 4.0, 9.0 / 4.0));
    eprintln!("Render time: {} ms", start_time.elapsed().as_millis());

    let mut out_image = format!("P3\n{xres} {yres}\n255\n");

    for y in 0..yres {
        for x in 0..xres {
            let p = pixels[y * xres + x];
            let (r, g, b) = (p[0], p[1], p[2]);
            out_image.push_str(&format!("{r} {g} {b}\n"))
        }
    }

    print!("{out_image}");
}
