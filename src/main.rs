#[derive(Clone)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Vec3 {
    fn dist(&self, other: &Vec3) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }

    fn len(&self) -> f64 {
        self.dist(&Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        })
    }

    fn normalized(self) -> Vec3 {
        let len = self.len();
        self * len.recip()
    }
}

trait SignedDistance {
    fn dist(&self, point: &Vec3) -> f64;
}

fn render(
    fields: Vec<Box<dyn SignedDistance>>,
    resolution: (usize, usize),
    fov: (f64, f64),
) -> Vec<bool> {
    fn raycast(fields: &Vec<Box<dyn SignedDistance>>, pos: Vec3, dir: Vec3) -> Option<Vec3> {
        fn aux_raycast(
            fields: &Vec<Box<dyn SignedDistance>>,
            dir: &Vec3,
            _start_pos: Vec3,
            threshhold: f64,
            max_iter_count: usize,
            pos: Vec3,
            iter_count: usize,
        ) -> Option<Vec3> {
            if iter_count > max_iter_count {
                return None;
            }

            let dists = fields.iter().map(|f| f.dist(&pos));
            let nearest = dists.reduce(f64::min)?;

            if nearest <= threshhold {
                return Some(pos);
            } else {
                return aux_raycast(
                    fields,
                    dir,
                    _start_pos,
                    threshhold,
                    max_iter_count,
                    pos + dir.clone() * nearest,
                    iter_count + 1,
                );
            }
        }
        aux_raycast(&fields, &dir, pos.clone(), 0.1, 10, pos, 0)
    }

    let mut pixels: Vec<bool> = vec![];

    for x in 0..resolution.0 {
        for y in 0..resolution.1 {
            let dir = Vec3 {
                x: 1.0,
                y: (y as f64 - ((resolution.1) / 2) as f64) * fov.1,
                z: (x as f64 - ((resolution.0) / 2) as f64) * fov.0,
            }
            .normalized();

            let hit_pos = raycast(
                &fields,
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                dir,
            );

            pixels.push(hit_pos.is_some());
        }
    }

    pixels
}

struct Sphere {
    pos: Vec3,
    rad: f64,
}

impl SignedDistance for Sphere {
    fn dist(&self, point: &Vec3) -> f64 {
        point.dist(&self.pos) - self.rad
    }
}

fn main() {
    let fields: Vec<Box<dyn SignedDistance>> = vec![Box::new(Sphere {
        pos: Vec3 {
            x: 100.0,
            y: -25.0,
            z: 10.0,
        },
        rad: 50.0,
    })];

    let xres = 500;
    let yres = 500;

    let pixels = render(fields, (xres, yres), (0.01, 0.01));

    let mut out_image = format!("P3\n{xres} {yres}\n255\n");

    for x in 0..xres {
        for y in 0..yres {
            let val = pixels[y * xres + x] as usize * 255;
            out_image.push_str(&format!("{val} {val} {val}\n"))
        }
    }

    println!("{out_image}");
}
