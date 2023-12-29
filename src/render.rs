use super::*;

pub fn render(
    fields: Vec<Box<dyn SignedDistance>>,
    resolution: (usize, usize),
    fov: (f64, f64),
) -> Vec<[u8; 3]> {
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

            let dists = fields.iter().map(|f| f.dist(&pos)).zip(0..);
            let (dist, nearest_index) =
                dists.reduce(|(dista, j), (dist, i)| match dist < dista {
                    true => (dist, i),
                    false => (dista, j),
                })?;

            if dist <= threshhold {
                return Some(fields[nearest_index].norm(&pos));
            } else {
                return aux_raycast(
                    fields,
                    dir,
                    _start_pos,
                    threshhold,
                    max_iter_count,
                    pos + dir.clone() * dist,
                    iter_count + 1,
                );
            }
        }
        aux_raycast(&fields, &dir, pos.clone(), 0.1, 100, pos, 0)
    }

    let light = Vec3 {
        x: -1.0,
        y: -0.5,
        z: -0.3,
    }
    .normalized();

    let mut pixels: Vec<[u8; 3]> = vec![];

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

            if let Some(norm) = raycast(
                &fields,
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                dir,
            ) {
                let brightness = light.dot(&norm);
                pixels.push([
                    ((norm.z * 256.0).abs() * brightness) as u8,
                    ((norm.y * 256.0).abs() * brightness) as u8,
                    ((norm.x * 256.0).abs() * brightness) as u8,
                ]);
            } else {
                pixels.push([0, 0, 0]);
            }
        }
    }
    pixels
}
