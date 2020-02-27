mod color;
mod vec3;

use rayon::prelude::*;
use std::io::prelude::*;
use std::{convert::TryFrom, error, fmt, fs::File, io, io::BufWriter, time::Instant};

use image::{ImageError, RgbImage};

use crate::color::{Rgb, Rgba};
use crate::vec3::Vec3f32;

#[derive(Debug)]
enum RayTracerError {
    Parse(ImageError),
    Render(io::Error),
}

impl fmt::Display for RayTracerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RayTracerError::Parse(ref err) => write!(f, "Parse error: {}", err),
            RayTracerError::Render(ref err) => write!(f, "Render error: {}", err),
        }
    }
}

impl error::Error for RayTracerError {
    fn description(&self) -> &str {
        match *self {
            RayTracerError::Parse(ref err) => error::Error::description(err),
            RayTracerError::Render(ref err) => error::Error::description(err),
        }
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            RayTracerError::Parse(ref err) => Some(err),
            RayTracerError::Render(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for RayTracerError {
    fn from(err: io::Error) -> RayTracerError {
        RayTracerError::Render(err)
    }
}

impl From<ImageError> for RayTracerError {
    fn from(err: ImageError) -> RayTracerError {
        RayTracerError::Parse(err)
    }
}

// https://doc.rust-lang.org/rust-by-example/error/result/result_alias.html
// Generic form not needed today type ResultRayTracer<T> = Result<T, RayTracerError>;
type ResultRayTracer = Result<(), RayTracerError>;

#[derive(Debug, Clone, Copy)]
struct Material {
    refractive_index: f32,
    albedo: Rgba,
    diffuse_color: Rgb,
    specular_exponent: f32,
}

#[derive(Debug)]
struct Sphere {
    center: Vec3f32,
    radius: f32,
    material: Material,
}

#[derive(Debug)]
struct Light {
    position: Vec3f32,
    intensity: f32,
}

fn reflect(i: &Vec3f32, n: &Vec3f32) -> Vec3f32 {
    i - n * 2. * i.dot_product(n)
}

fn refract(i: &Vec3f32, n: &Vec3f32, eta_t: f32, eta_i: f32) -> Vec3f32 {
    let cosi = -i.dot_product(n).min(1.).max(-1.);
    if cosi < 0. {
        return refract(i, &(n * -1.), eta_i, eta_t);
    }
    let eta = eta_i / eta_t;
    let k = 1. - eta * eta * (1. - cosi * cosi);
    if k < 0. {
        Vec3f32::new(1., 0., 0.)
    } else {
        i * eta + n * (eta * cosi - k.sqrt())
    }
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
        if s.ray_intersect(orig, dir, &mut dist_i) && dist_i < sphere_dist {
            sphere_dist = dist_i;
            *hit = orig + dir * dist_i;
            *n = *hit - &s.center;
            n.normalize();
            *material = s.material;
        }
    }

    let mut checkerboard_dist = std::f32::MAX;
    if dir.y.abs() > 1e-3 {
        let d = -(orig.y + 4.) / dir.y;
        let pt = orig + dir * d;
        if d > 0. && pt.x.abs() < 10. && pt.z < -10. && pt.z > -30. && d < sphere_dist {
            checkerboard_dist = d;
            *hit = pt;
            *n = Vec3f32::new(0., 1., 0.);
            material.diffuse_color =
                if (((0.5 * hit.x + 1000.) as i32 + (0.5 * hit.z) as i32) & 1) != 0 {
                    Rgb::new(0.3, 0.3, 0.3)
                } else {
                    Rgb::new(0.3, 0.2, 0.1)
                };
        }
    }

    sphere_dist.min(checkerboard_dist) < 1000.
}

impl Sphere {
    fn ray_intersect(self: &Sphere, orig: &Vec3f32, dir: &Vec3f32, t0: &mut f32) -> bool {
        let l = &self.center - orig;
        let tca = l.dot_product(dir);
        let d2 = l.dot_product(&l) - tca * tca;
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

fn cast_ray(
    orig: &Vec3f32,
    dir: &Vec3f32,
    spheres: &[Sphere],
    lights: &[Light],
    background: &RgbImage,
    depth: usize,
) -> Rgb {
    let mut point = Vec3f32::new(0., 0., 0.);
    let mut n = Vec3f32::new(0., 0., 0.);
    let mut material = Material {
        refractive_index: 1.,
        albedo: Rgba::new(1., 0., 0., 0.),
        diffuse_color: Rgb::new(0., 0., 0.),
        specular_exponent: 0.,
    };

    if !scene_intersect(orig, dir, spheres, &mut point, &mut n, &mut material) || depth > 4 {
        let mut norm_dir = *dir;
        norm_dir.normalize();
        let x = (norm_dir.z.atan2(norm_dir.x) / (2. * std::f32::consts::PI) + 0.5)
            * background.width() as f32;
        let y = (norm_dir.y.acos() / std::f32::consts::PI) * background.height() as f32;
        let rgb_pixel = background.get_pixel(x as u32, y as u32);
        return Rgb::new(
            rgb_pixel[0] as f32 / 255.,
            rgb_pixel[1] as f32 / 255.,
            rgb_pixel[2] as f32 / 255.,
        );
    }

    let mut reflect_dir = reflect(&dir, &n);
    reflect_dir.normalize();
    let mut refract_dir = refract(&dir, &n, material.refractive_index, 1.);
    refract_dir.normalize();
    let reflect_orig = if reflect_dir.dot_product(&n) < 0. {
        point - &n * 1e-3
    } else {
        point + &n * 1e-3
    };
    let refract_orig = if refract_dir.dot_product(&n) < 0. {
        point - &n * 1e-3
    } else {
        point + &n * 1e-3
    };
    let reflect_color = cast_ray(
        &reflect_orig,
        &reflect_dir,
        &spheres,
        &lights,
        background,
        depth + 1,
    );
    let refract_color = cast_ray(
        &refract_orig,
        &refract_dir,
        &spheres,
        &lights,
        background,
        depth + 1,
    );

    let mut diffuse_light_intensity = 0.;
    let mut specular_light_intensity = 0.;
    for l in lights {
        let mut light_dir = &l.position - &point;
        let light_distance = light_dir.norm();
        light_dir.normalize();

        let shadow_orig = if (&light_dir * &n).norm() < 0. {
            point - &n * 1e-3
        } else {
            point + &n * 1e-3
        };
        let mut shadow_pt = Vec3f32::new(0., 0., 0.);
        let mut shadow_n = Vec3f32::new(0., 0., 0.);
        let mut tmp_material = Material {
            refractive_index: 1.,
            albedo: Rgba::new(1., 0., 0., 0.),
            diffuse_color: Rgb::new(0., 0., 0.),
            specular_exponent: 0.,
        };

        if scene_intersect(
            &shadow_orig,
            &light_dir,
            &spheres,
            &mut shadow_pt,
            &mut shadow_n,
            &mut tmp_material,
        ) && (shadow_pt - shadow_orig).norm() < light_distance
        {
            continue;
        }

        diffuse_light_intensity += l.intensity * light_dir.dot_product(&n).max(0.);
        let vec_reflect = reflect(&(light_dir * -1.), &n) * -1.;
        specular_light_intensity += vec_reflect
            .dot_product(dir)
            .max(0.)
            .powf(material.specular_exponent)
            * l.intensity
    }

    material.diffuse_color * diffuse_light_intensity * material.albedo.r
        + Rgb::new(1., 1., 1.) * specular_light_intensity * material.albedo.g
        + reflect_color * material.albedo.b
        + refract_color * material.albedo.a
}

fn render(background: &RgbImage, spheres: &[Sphere], lights: &[Light]) -> ResultRayTracer {
    const WIDTH: usize = 1024;
    const HEIGHT: usize = 728;
    const FOV: f32 = (std::f64::consts::PI / 2.0) as f32;

    let mut framebuffer: Vec<Rgb> = vec![Rgb::new(0., 0., 0.); HEIGHT * WIDTH];

    framebuffer
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, v)| {
            // unwrap could not failed with actual WIDTH and HEIGHT
            let i = u32::try_from(index).unwrap() as f32 % WIDTH as f32;
            let j = u32::try_from(index).unwrap() as f32 / WIDTH as f32;
            let dir_x = (i as f32 + 0.5) - WIDTH as f32 / 2.;
            let dir_y = -(j as f32 + 0.5) + HEIGHT as f32 / 2.;
            let dir_z = -1. * HEIGHT as f32 / (2. * (FOV / 2.).tan());
            let mut dir = Vec3f32::new(dir_x, dir_y, dir_z);
            dir.normalize();
            *v = cast_ray(
                &Vec3f32::new(0.0, 0.0, 0.0),
                &dir,
                spheres,
                lights,
                background,
                0,
            );
        });

    let mut file = BufWriter::new(File::create("out.ppm")?);
    write!(&mut file, "P6\n{} {}\n255\n", WIDTH, HEIGHT)?;
    for v in framebuffer.iter_mut() {
        let max = v.r.max(v.g.max(v.b));
        if max > 1. {
            *v = *v / max;
        }
        file.write_all(&[(255. * v.r.min(1.).max(0.)) as u8])?;
        file.write_all(&[(255. * v.g.min(1.).max(0.)) as u8])?;
        file.write_all(&[(255. * v.b.min(1.).max(0.)) as u8])?;
    }
    Ok(())
}

fn main() -> ResultRayTracer {
    let background = image::open("./envmap.jpg")?.to_rgb();

    let ivory = Material {
        refractive_index: 1.,
        albedo: Rgba::new(0.6, 0.3, 0.1, 0.),
        diffuse_color: Rgb::new(0.4, 0.4, 0.3),
        specular_exponent: 50.,
    };
    let glass = Material {
        refractive_index: 1.5,
        albedo: Rgba::new(0., 0.5, 0.1, 0.8),
        diffuse_color: Rgb::new(0.6, 0.7, 0.8),
        specular_exponent: 125.,
    };
    let red_rubber = Material {
        refractive_index: 1.,
        albedo: Rgba::new(0.9, 0.1, 0., 0.),
        diffuse_color: Rgb::new(0.3, 0.1, 0.1),
        specular_exponent: 10.,
    };
    let mirror = Material {
        refractive_index: 1.,
        albedo: Rgba::new(0., 10., 0.8, 0.),
        diffuse_color: Rgb::new(1., 1., 1.),
        specular_exponent: 1425.,
    };

    let spheres = vec![
        Sphere {
            center: Vec3f32::new(-3.0, 0.0, -16.0),
            radius: 2.0,
            material: ivory,
        },
        Sphere {
            center: Vec3f32::new(-1.0, -1.5, -12.0),
            radius: 2.0,
            material: glass,
        },
        Sphere {
            center: Vec3f32::new(1.5, -0.5, -18.0),
            radius: 3.0,
            material: red_rubber,
        },
        Sphere {
            center: Vec3f32::new(7.0, 5.0, -18.0),
            radius: 4.0,
            material: mirror,
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
    let start = Instant::now();

    render(&background, &spheres, &lights)?;

    let elapsed = start.elapsed();
    println!(
        "Elapsed: {} ms",
        (elapsed.as_secs() * 1_000) + elapsed.subsec_millis() as u64
    );
    Ok(())
}
