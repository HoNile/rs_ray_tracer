use std::fs::File;
use std::io::prelude::*;

use vec3::Vec3f32;

#[derive(Clone)]
struct Rgb {
    r: f32,
    g: f32,
    b: f32,
}

#[derive(Clone)]
struct Vec2f32 {
    x: f32,
    y: f32,
}

#[derive(Clone)]
struct Material {
    albedo: Vec2f32,
    diffuse_color: Rgb,
    specular_exponent: f32,
}

struct Sphere {
    center: Vec3f32,
    radius: f32,
    material: Material,
}

struct Light {
    position: Vec3f32,
    intensity: f32,
}

fn scalar_product(v1: &Vec3f32, v2: &Vec3f32) -> f32 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

fn reflect(i: &Vec3f32, n: &Vec3f32) -> Vec3f32 {
    // TODO remove or test this comment: i + n
    Vec3f32::new(
        i.x - n.x * 2. * scalar_product(i, n),
        i.y - n.y * 2. * scalar_product(i, n),
        i.z - n.z * 2. * scalar_product(i, n),
    )
}

fn scene_intersect(
    orig: &Vec3f32,
    dir: &Vec3f32,
    spheres: &[Sphere],
    hit: &mut Vec3f32,
    n: &mut Vec3f32,
    material: &mut Material,
) -> bool {
    let mut sphere_dist = std::f32::MAX;
    for s in spheres.iter() {
        let mut dist_i = 0.;
        let ret_ray_intersect = s.ray_intersect(orig, dir, &mut dist_i);
        if ret_ray_intersect && dist_i < sphere_dist {
            sphere_dist = dist_i;
            *hit = orig + dir * dist_i;
            *n = hit.clone() - &s.center;
            n.normalize();
            *material = s.material.clone();
        }
    }
    sphere_dist < 1000.0
}

impl Sphere {
    fn ray_intersect(self: &Sphere, orig: &Vec3f32, dir: &Vec3f32, t0: &mut f32) -> bool {
        let l = Vec3f32::new(
            self.center.x - orig.x,
            self.center.y - orig.y,
            self.center.z - orig.z,
        );
        let tca = scalar_product(&l, dir);
        let d2 = scalar_product(&l, &l) - tca * tca;
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
        true
    }
}

fn cast_ray(orig: &Vec3f32, dir: &Vec3f32, spheres: &[Sphere], lights: &[Light]) -> Rgb {
    let mut point = Vec3f32::new(0., 0., 0.);
    let mut n = Vec3f32::new(0., 0., 0.);
    let mut material = Material {
        albedo: Vec2f32 { x: 0., y: 0. },
        diffuse_color: Rgb {
            r: 0.,
            g: 0.,
            b: 0.,
        },
        specular_exponent: 0.,
    };

    if !scene_intersect(orig, dir, spheres, &mut point, &mut n, &mut material) {
        return Rgb {
            r: 0.2,
            g: 0.7,
            b: 0.8,
        }; // background color
    }

    let mut diffuse_light_intensity = 0.;
    let mut specular_light_intensity = 0.;
    for l in lights {
        let mut light_dir = &l.position - &point;
        let _light_distance = light_dir.norm(); // TODO in progress
        light_dir.normalize();
        diffuse_light_intensity += l.intensity * scalar_product(&light_dir, &n).max(0.);
        let mut vec_reflect = reflect(&(light_dir * -1.), &n);
        vec_reflect = vec_reflect * -1.;
        specular_light_intensity += scalar_product(&vec_reflect, &dir)
            .max(0.)
            .powf(material.specular_exponent)
            * l.intensity
    }
    Rgb {
        r: material.diffuse_color.r * diffuse_light_intensity * material.albedo.x
            + specular_light_intensity * material.albedo.y,
        g: material.diffuse_color.g * diffuse_light_intensity * material.albedo.x
            + specular_light_intensity * material.albedo.y,
        b: material.diffuse_color.b * diffuse_light_intensity * material.albedo.x
            + specular_light_intensity * material.albedo.y,
    }
}

fn render(spheres: &[Sphere], lights: &[Light]) -> std::io::Result<()> {
    const WIDTH: i32 = 1024;
    const HEIGHT: i32 = 728;
    const FOV: f32 = (std::f64::consts::PI / 2.0) as f32;

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
            let xf =
                (2.0 * (i as f32 + 0.5) / WIDTH as f32 - 1.0) * (FOV / 2.).tan() * WIDTH as f32
                    / HEIGHT as f32;
            let yf = -(2.0 * (j as f32 + 0.5) / HEIGHT as f32 - 1.0) * (FOV / 2.).tan();
            let mut dir = Vec3f32::new(
                xf,
                yf,
                -1.0,
            );
            dir.normalize();
            framebuffer[(i + j * WIDTH) as usize] =
                cast_ray(&Vec3f32::new(0.0, 0.0, 0.0), &dir, spheres, lights);
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
    let ivory = Material {
        albedo: Vec2f32 { x: 0.6, y: 0.3 },
        diffuse_color: Rgb {
            r: 0.4,
            g: 0.4,
            b: 0.3,
        },
        specular_exponent: 50.,
    };
    let red_rubber = Material {
        albedo: Vec2f32 { x: 0.9, y: 0.1 },
        diffuse_color: Rgb {
            r: 0.3,
            g: 0.1,
            b: 0.1,
        },
        specular_exponent: 50.,
    };

    let spheres = vec![
        Sphere {
            center: Vec3f32::new(-3.0, 0.0, -16.0),
            radius: 2.0,
            material: ivory.clone(),
        },
        Sphere {
            center: Vec3f32::new(-1.0, -1.5, -12.0),
            radius: 2.0,
            material: red_rubber.clone(),
        },
        Sphere {
            center: Vec3f32::new(1.5, -0.5, -18.0),
            radius: 3.0,
            material: red_rubber.clone(),
        },
        Sphere {
            center: Vec3f32::new(7.0, 5.0, -18.0),
            radius: 4.0,
            material: ivory.clone(),
        },
    ];

    let lights = vec![
        Light {
            position: Vec3f32::new(-20., 20., 20.),
            intensity: 1.5,
        },
        Light {
            position: Vec3f32::new(30., 50., -25.),
            intensity: 1.8,
        },
        Light {
            position: Vec3f32::new(30., 20., 30.),
            intensity: 1.7,
        },
    ];
    render(&spheres, &lights)
}
