use std::ops::{Add, Div, Mul, Sub};

//TODO this should be generic

#[derive(Clone)]
pub struct Vec3f32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// https://github.com/rust-lang/rust/issues/21188
impl Add<Vec3f32> for Vec3f32 {
    type Output = Vec3f32; // https://stackoverflow.com/questions/39115363/why-do-rusts-operators-have-the-type-output-variable
    fn add(self, other: Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
impl<'a> Add<Vec3f32> for &'a Vec3f32 {
    type Output = Vec3f32; // https://stackoverflow.com/questions/39115363/why-do-rusts-operators-have-the-type-output-variable
    fn add(self, other: Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
impl<'a> Add<&'a Vec3f32> for Vec3f32 {
    type Output = Vec3f32; // https://stackoverflow.com/questions/39115363/why-do-rusts-operators-have-the-type-output-variable
    fn add(self, other: &Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
impl<'a> Add<&'a Vec3f32> for &'a Vec3f32 {
    type Output = Vec3f32; // https://stackoverflow.com/questions/39115363/why-do-rusts-operators-have-the-type-output-variable
    fn add(self, other: &Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<f32> for Vec3f32 {
    type Output = Vec3f32;
    fn add(self, coef: f32) -> Vec3f32 {
        Vec3f32::new(self.x + coef, self.y + coef, self.z + coef)
    }
}
impl<'a> Add<f32> for &'a Vec3f32 {
    type Output = Vec3f32;
    fn add(self, coef: f32) -> Vec3f32 {
        Vec3f32::new(self.x + coef, self.y + coef, self.z + coef)
    }
}
impl<'a> Add<&'a f32> for Vec3f32 {
    type Output = Vec3f32;
    fn add(self, coef: &f32) -> Vec3f32 {
        Vec3f32::new(self.x + *coef, self.y + *coef, self.z + *coef)
    }
}
impl<'a> Add<&'a f32> for &'a Vec3f32 {
    type Output = Vec3f32;
    fn add(self, coef: &f32) -> Vec3f32 {
        Vec3f32::new(self.x + *coef, self.y + *coef, self.z + *coef)
    }
}

impl Sub<Vec3f32> for Vec3f32 {
    type Output = Vec3f32;
    fn sub(self, other: Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}
impl<'a> Sub<Vec3f32> for &'a Vec3f32 {
    type Output = Vec3f32;
    fn sub(self, other: Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}
impl<'a> Sub<&'a Vec3f32> for Vec3f32 {
    type Output = Vec3f32;
    fn sub(self, other: &Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}
impl<'a> Sub<&'a Vec3f32> for &'a Vec3f32 {
    type Output = Vec3f32;
    fn sub(self, other: &Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub<f32> for Vec3f32 {
    type Output = Vec3f32;
    fn sub(self, coef: f32) -> Vec3f32 {
        Vec3f32::new(self.x - coef, self.y - coef, self.z - coef)
    }
}
impl<'a> Sub<f32> for &'a Vec3f32 {
    type Output = Vec3f32;
    fn sub(self, coef: f32) -> Vec3f32 {
        Vec3f32::new(self.x - coef, self.y - coef, self.z - coef)
    }
}
impl<'a> Sub<&'a f32> for Vec3f32 {
    type Output = Vec3f32;
    fn sub(self, coef: &f32) -> Vec3f32 {
        Vec3f32::new(self.x - *coef, self.y - *coef, self.z - *coef)
    }
}
impl<'a> Sub<&'a f32> for &'a Vec3f32 {
    type Output = Vec3f32;
    fn sub(self, coef: &f32) -> Vec3f32 {
        Vec3f32::new(self.x - *coef, self.y - *coef, self.z - *coef)
    }
}

impl Mul<Vec3f32> for Vec3f32 {
    type Output = Vec3f32;
    fn mul(self, other: Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}
impl<'a> Mul<Vec3f32> for &'a Vec3f32 {
    type Output = Vec3f32;
    fn mul(self, other: Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}
impl<'a> Mul<&'a Vec3f32> for Vec3f32 {
    type Output = Vec3f32;
    fn mul(self, other: &Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}
impl<'a> Mul<&'a Vec3f32> for &'a Vec3f32 {
    type Output = Vec3f32;
    fn mul(self, other: &Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Mul<f32> for Vec3f32 {
    type Output = Vec3f32;
    fn mul(self, coef: f32) -> Vec3f32 {
        Vec3f32::new(self.x * coef, self.y * coef, self.z * coef)
    }
}
impl<'a> Mul<f32> for &'a Vec3f32 {
    type Output = Vec3f32;
    fn mul(self, coef: f32) -> Vec3f32 {
        Vec3f32::new(self.x * coef, self.y * coef, self.z * coef)
    }
}
impl<'a> Mul<&'a f32> for Vec3f32 {
    type Output = Vec3f32;
    fn mul(self, coef: &f32) -> Vec3f32 {
        Vec3f32::new(self.x * *coef, self.y * *coef, self.z * *coef)
    }
}
impl<'a> Mul<&'a f32> for &'a Vec3f32 {
    type Output = Vec3f32;
    fn mul(self, coef: &f32) -> Vec3f32 {
        Vec3f32::new(self.x * *coef, self.y * *coef, self.z * *coef)
    }
}

impl Div<Vec3f32> for Vec3f32 {
    type Output = Vec3f32;
    fn div(self, other: Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}
impl<'a> Div<Vec3f32> for &'a Vec3f32 {
    type Output = Vec3f32;
    fn div(self, other: Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}
impl<'a> Div<&'a Vec3f32> for Vec3f32 {
    type Output = Vec3f32;
    fn div(self, other: &Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}
impl<'a> Div<&'a Vec3f32> for &'a Vec3f32 {
    type Output = Vec3f32;
    fn div(self, other: &Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

impl Div<f32> for Vec3f32 {
    type Output = Vec3f32;
    fn div(self, coef: f32) -> Vec3f32 {
        Vec3f32::new(self.x / coef, self.y / coef, self.z / coef)
    }
}
impl<'a> Div<f32> for &'a Vec3f32 {
    type Output = Vec3f32;
    fn div(self, coef: f32) -> Vec3f32 {
        Vec3f32::new(self.x / coef, self.y / coef, self.z / coef)
    }
}
impl<'a> Div<&'a f32> for Vec3f32 {
    type Output = Vec3f32;
    fn div(self, coef: &f32) -> Vec3f32 {
        Vec3f32::new(self.x / *coef, self.y / *coef, self.z / *coef)
    }
}
impl<'a> Div<&'a f32> for &'a Vec3f32 {
    type Output = Vec3f32;
    fn div(self, coef: &f32) -> Vec3f32 {
        Vec3f32::new(self.x / *coef, self.y / *coef, self.z / *coef)
    }
}

impl Vec3f32 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3f32 {
        Vec3f32 { x, y, z }
    }

    pub fn norm(self: &Self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(self: &mut Self) {
        let length = ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt();
        self.x /= length;
        self.y /= length;
        self.z /= length;
    }
}

#[derive(Clone)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Rgba {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Rgba {
        Rgba { r, g, b, a }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

// https://github.com/rust-lang/rust/issues/21188
impl Add<Rgb> for Rgb {
    type Output = Rgb; // https://stackoverflow.com/questions/39115363/whg-do-rusts-operators-have-the-type-output-variable
    fn add(self, other: Rgb) -> Rgb {
        Rgb::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}
impl<'a> Add<Rgb> for &'a Rgb {
    type Output = Rgb; // https://stackoverflow.com/questions/39115363/whg-do-rusts-operators-have-the-type-output-variable
    fn add(self, other: Rgb) -> Rgb {
        Rgb::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}
impl<'a> Add<&'a Rgb> for Rgb {
    type Output = Rgb; // https://stackoverflow.com/questions/39115363/whg-do-rusts-operators-have-the-type-output-variable
    fn add(self, other: &Rgb) -> Rgb {
        Rgb::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}
impl<'a> Add<&'a Rgb> for &'a Rgb {
    type Output = Rgb; // https://stackoverflow.com/questions/39115363/whg-do-rusts-operators-have-the-type-output-variable
    fn add(self, other: &Rgb) -> Rgb {
        Rgb::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}

impl Add<f32> for Rgb {
    type Output = Rgb;
    fn add(self, coef: f32) -> Rgb {
        Rgb::new(self.r + coef, self.g + coef, self.b + coef)
    }
}
impl<'a> Add<f32> for &'a Rgb {
    type Output = Rgb;
    fn add(self, coef: f32) -> Rgb {
        Rgb::new(self.r + coef, self.g + coef, self.b + coef)
    }
}
impl<'a> Add<&'a f32> for Rgb {
    type Output = Rgb;
    fn add(self, coef: &f32) -> Rgb {
        Rgb::new(self.r + *coef, self.g + *coef, self.b + *coef)
    }
}
impl<'a> Add<&'a f32> for &'a Rgb {
    type Output = Rgb;
    fn add(self, coef: &f32) -> Rgb {
        Rgb::new(self.r + *coef, self.g + *coef, self.b + *coef)
    }
}

impl Sub<Rgb> for Rgb {
    type Output = Rgb;
    fn sub(self, other: Rgb) -> Rgb {
        Rgb::new(self.r - other.r, self.g - other.g, self.b - other.b)
    }
}
impl<'a> Sub<Rgb> for &'a Rgb {
    type Output = Rgb;
    fn sub(self, other: Rgb) -> Rgb {
        Rgb::new(self.r - other.r, self.g - other.g, self.b - other.b)
    }
}
impl<'a> Sub<&'a Rgb> for Rgb {
    type Output = Rgb;
    fn sub(self, other: &Rgb) -> Rgb {
        Rgb::new(self.r - other.r, self.g - other.g, self.b - other.b)
    }
}
impl<'a> Sub<&'a Rgb> for &'a Rgb {
    type Output = Rgb;
    fn sub(self, other: &Rgb) -> Rgb {
        Rgb::new(self.r - other.r, self.g - other.g, self.b - other.b)
    }
}

impl Sub<f32> for Rgb {
    type Output = Rgb;
    fn sub(self, coef: f32) -> Rgb {
        Rgb::new(self.r - coef, self.g - coef, self.b - coef)
    }
}
impl<'a> Sub<f32> for &'a Rgb {
    type Output = Rgb;
    fn sub(self, coef: f32) -> Rgb {
        Rgb::new(self.r - coef, self.g - coef, self.b - coef)
    }
}
impl<'a> Sub<&'a f32> for Rgb {
    type Output = Rgb;
    fn sub(self, coef: &f32) -> Rgb {
        Rgb::new(self.r - *coef, self.g - *coef, self.b - *coef)
    }
}
impl<'a> Sub<&'a f32> for &'a Rgb {
    type Output = Rgb;
    fn sub(self, coef: &f32) -> Rgb {
        Rgb::new(self.r - *coef, self.g - *coef, self.b - *coef)
    }
}

impl Mul<Rgb> for Rgb {
    type Output = Rgb;
    fn mul(self, other: Rgb) -> Rgb {
        Rgb::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}
impl<'a> Mul<Rgb> for &'a Rgb {
    type Output = Rgb;
    fn mul(self, other: Rgb) -> Rgb {
        Rgb::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}
impl<'a> Mul<&'a Rgb> for Rgb {
    type Output = Rgb;
    fn mul(self, other: &Rgb) -> Rgb {
        Rgb::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}
impl<'a> Mul<&'a Rgb> for &'a Rgb {
    type Output = Rgb;
    fn mul(self, other: &Rgb) -> Rgb {
        Rgb::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}

impl Mul<f32> for Rgb {
    type Output = Rgb;
    fn mul(self, coef: f32) -> Rgb {
        Rgb::new(self.r * coef, self.g * coef, self.b * coef)
    }
}
impl<'a> Mul<f32> for &'a Rgb {
    type Output = Rgb;
    fn mul(self, coef: f32) -> Rgb {
        Rgb::new(self.r * coef, self.g * coef, self.b * coef)
    }
}
impl<'a> Mul<&'a f32> for Rgb {
    type Output = Rgb;
    fn mul(self, coef: &f32) -> Rgb {
        Rgb::new(self.r * *coef, self.g * *coef, self.b * *coef)
    }
}
impl<'a> Mul<&'a f32> for &'a Rgb {
    type Output = Rgb;
    fn mul(self, coef: &f32) -> Rgb {
        Rgb::new(self.r * *coef, self.g * *coef, self.b * *coef)
    }
}

impl Div<Rgb> for Rgb {
    type Output = Rgb;
    fn div(self, other: Rgb) -> Rgb {
        Rgb::new(self.r / other.r, self.g / other.g, self.b / other.b)
    }
}
impl<'a> Div<Rgb> for &'a Rgb {
    type Output = Rgb;
    fn div(self, other: Rgb) -> Rgb {
        Rgb::new(self.r / other.r, self.g / other.g, self.b / other.b)
    }
}
impl<'a> Div<&'a Rgb> for Rgb {
    type Output = Rgb;
    fn div(self, other: &Rgb) -> Rgb {
        Rgb::new(self.r / other.r, self.g / other.g, self.b / other.b)
    }
}
impl<'a> Div<&'a Rgb> for &'a Rgb {
    type Output = Rgb;
    fn div(self, other: &Rgb) -> Rgb {
        Rgb::new(self.r / other.r, self.g / other.g, self.b / other.b)
    }
}

impl Div<f32> for Rgb {
    type Output = Rgb;
    fn div(self, coef: f32) -> Rgb {
        Rgb::new(self.r / coef, self.g / coef, self.b / coef)
    }
}
impl<'a> Div<f32> for &'a Rgb {
    type Output = Rgb;
    fn div(self, coef: f32) -> Rgb {
        Rgb::new(self.r / coef, self.g / coef, self.b / coef)
    }
}
impl<'a> Div<&'a f32> for Rgb {
    type Output = Rgb;
    fn div(self, coef: &f32) -> Rgb {
        Rgb::new(self.r / *coef, self.g / *coef, self.b / *coef)
    }
}
impl<'a> Div<&'a f32> for &'a Rgb {
    type Output = Rgb;
    fn div(self, coef: &f32) -> Rgb {
        Rgb::new(self.r / *coef, self.g / *coef, self.b / *coef)
    }
}

impl Rgb {
    pub fn new(r: f32, g: f32, b: f32) -> Rgb {
        Rgb { r, g, b }
    }
}
