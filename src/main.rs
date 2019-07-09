use std::fs::File;
use std::io::prelude::*;

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
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

#[derive(Clone)]
struct Material {
    diffuse_color: Rgb,
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

fn scene_intersect(
    orig: &Vec3f32,
    dir: &Vec3f32,
    spheres: &Vec<Sphere>,
    hit: &mut Vec3f32,
    N: &mut Vec3f32,
    material: &mut Material,
) -> bool {
    let mut sphere_dist = std::f32::MAX;
    for s in spheres.iter() {
        let mut dist_i = 0.;
        let ret_ray_intersect = s.ray_intersect(orig, dir, &mut dist_i);
        if ret_ray_intersect && dist_i < sphere_dist {
            sphere_dist = dist_i;
            *hit = Vec3f32 {
                x: orig.x + dir.x * dist_i,
                y: orig.y + dir.y * dist_i,
                z: orig.z + dir.z * dist_i,
            };
            *N = Vec3f32 {
                x: hit.x - s.center.x,
                y: hit.y - s.center.y,
                z: hit.z - s.center.z,
            };
            N.normalize();
            *material = s.material.clone();
        }
    }
    return sphere_dist < 1000.0;
}

impl Sphere {
    fn ray_intersect(self: &Sphere, orig: &Vec3f32, dir: &Vec3f32, t0: &mut f32) -> bool {
        let L = Vec3f32 {
            x: self.center.x - orig.x,
            y: self.center.y - orig.y,
            z: self.center.z - orig.z,
        };
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

fn cast_ray(orig: &Vec3f32, dir: &Vec3f32, spheres: &Vec<Sphere>, lights: &Vec<Light>) -> Rgb {
    let mut point = Vec3f32 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let mut N = Vec3f32 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let mut material = Material {
        diffuse_color: Rgb {
            r: 0.,
            g: 0.,
            b: 0.,
        },
    };

    if !scene_intersect(orig, dir, spheres, &mut point, &mut N, &mut material) {
        return Rgb {
            r: 0.2,
            g: 0.7,
            b: 0.8,
        }; // background color
    }

    let mut diffuse_light_intensity = 0.;
    for l in lights {
        let mut light_dir = Vec3f32 {
            x: l.position.x - point.x,
            y: l.position.y - point.y,
            z: l.position.z - point.z,
        };
        light_dir.normalize();
        diffuse_light_intensity += l.intensity * scalar_product(&light_dir, &N).max(0.);
    }
    return Rgb {
        r: material.diffuse_color.r * diffuse_light_intensity,
        g: material.diffuse_color.g * diffuse_light_intensity,
        b: material.diffuse_color.b * diffuse_light_intensity,
    };
}

fn render(spheres: &Vec<Sphere>, lights: &Vec<Light>) -> std::io::Result<()> {
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
            let mut dir = Vec3f32 {
                x: xf,
                y: yf,
                z: -1.0,
            };
            dir.normalize();
            framebuffer[(i + j * WIDTH) as usize] = cast_ray(
                &Vec3f32 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                &dir,
                spheres,
                lights,
            );
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
        diffuse_color: Rgb {
            r: 0.4,
            g: 0.4,
            b: 0.3,
        },
    };
    let red_rubber = Material {
        diffuse_color: Rgb {
            r: 0.3,
            g: 0.1,
            b: 0.1,
        },
    };

    let spheres = vec![
        Sphere {
            center: Vec3f32 {
                x: -3.0,
                y: 0.0,
                z: -16.0,
            },
            radius: 2.0,
            material: ivory.clone(),
        },
        Sphere {
            center: Vec3f32 {
                x: -1.0,
                y: -1.5,
                z: -12.0,
            },
            radius: 2.0,
            material: red_rubber.clone(),
        },
        Sphere {
            center: Vec3f32 {
                x: 1.5,
                y: -0.5,
                z: -18.0,
            },
            radius: 2.0,
            material: red_rubber.clone(),
        },
        Sphere {
            center: Vec3f32 {
                x: 7.0,
                y: 5.0,
                z: -18.0,
            },
            radius: 2.0,
            material: ivory.clone(),
        },
    ];

    let lights = vec![Light {
        position: Vec3f32 {
            x: -20.,
            y: 20.,
            z: 20.,
        },
        intensity: 1.5,
    }];
    render(&spheres, &lights)
}
