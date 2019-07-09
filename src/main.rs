use std::fs::File;
use std::io::prelude::*;
//use std::{mem, slice};

#[derive(Clone)]
struct Rgb {
    r: f32,
    g: f32,
    b: f32,
}

struct Vec3f32 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3f32 {

    fn normalize(self: &mut Self) {
        let length = ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt();
        self.x = self.x / length;
        self.y = self.y / length;
        self.z = self.z / length;
    }
}

fn scalar_product(v1: &Vec3f32, v2: &Vec3f32) -> f32 {
    v1.x*v2.x + v1.y*v2.y + v1.z*v2.z
}

struct Sphere {
    center: Vec3f32,
    radius: f32,
}

impl Sphere {

    fn ray_intersect(self: &Sphere, orig: &Vec3f32, dir: &Vec3f32, t0: &mut f32) -> bool {
        let L = Vec3f32{x: self.center.x - orig.x, y: self.center.y - orig.y, z: self.center.z - orig.z};
        let tca = scalar_product(&L, dir);
        let d2 = scalar_product(&L, &L) - tca * tca;
        if d2 > self.radius * self.radius {
            return false;
        }
        let thc = (self.radius * self.radius - d2 as f32).sqrt();
        *t0 = tca - thc;
        let t1 = tca + thc;
        if *t0 < 0.0 {
            *t0 = t1;
        }
        if *t0 < 0.0 {
            return false;
        }
        return true;
    }

}

fn cast_ray (orig: &Vec3f32, dir: &Vec3f32, sphere: &Sphere) -> Rgb {
    let mut sphere_dist = std::f32::MAX;
    if !sphere.ray_intersect(orig, dir, &mut sphere_dist) {
        return Rgb{r: 0.2, g: 0.7, b: 0.8}; // background color
    }

    return Rgb{r: 0.4, g: 0.4, b: 0.3};
}

fn render(sphere: &Sphere) -> std::io::Result<()> {
    const WIDTH: i32 = 1024;
    const HEIGHT: i32 = 728;
    const FOV: f32 = std::f32::consts::PI / 2.0;

    let mut framebuffer: Vec<Rgb> = vec![
        Rgb {
            r: 0.,
            g: 0.,
            b: 0.
        };
        (HEIGHT * WIDTH) as usize
    ];

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            /*framebuffer[(i + j * WIDTH) as usize] = Rgb {
                r: j as f32 / HEIGHT as f32,
                g: i as f32 / WIDTH as f32,
                b: 0.0,
            };*/
            let xf =  (2.0 *(i as f32 + 0.5)/WIDTH as f32 - 1.0) * (FOV/2.).tan() * WIDTH as f32/HEIGHT as f32;
            let yf = -(2.0 *(j as f32 + 0.5)/HEIGHT as f32 - 1.0)* (FOV/2.).tan();
            let mut dir = Vec3f32{x: xf, y: yf, z: -1.0};
            dir.normalize();
            framebuffer[(i+j*WIDTH) as usize] = cast_ray(&Vec3f32{x: 0.0, y: 0.0, z: 0.0}, &dir, sphere);
        }
    }

    let mut file = File::create("out.ppm")?;
    write!(&mut file, "P6\n{} {}\n255\n", WIDTH, HEIGHT).unwrap();
    for i in 0..WIDTH * HEIGHT {
        for j in 0..3 {
            match j {
                0 => file.write_all(&[(255. * framebuffer[i as usize].r.min(1.).max(0.)) as u8])?,
                1 => file.write_all(&[(255. * framebuffer[i as usize].g.min(1.).max(0.)) as u8])?,
                2 => file.write_all(&[(255. * framebuffer[i as usize].b.min(1.).max(0.)) as u8])?,
                _ => panic!("Impossiblu !!!"),
            }
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let sphere = Sphere{center: Vec3f32{x: -3., y: 0., z: -16.}, radius: 2.};
    render(&sphere)
}
