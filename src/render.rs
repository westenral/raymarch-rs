use super::*;

pub fn render(
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

    for y in 0..resolution.1 {
        let y_completion_percentage = y as f64 / resolution.1 as f64;
        for x in 0..resolution.0 {
            let x_completion_percentage = x as f64 / resolution.0 as f64;

            let dir = Vec3 {
                x: 1.0,
                y: fov.1 * y_completion_percentage - fov.1 / 2.0,
                z: fov.0 * x_completion_percentage - fov.0 / 2.0,
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
