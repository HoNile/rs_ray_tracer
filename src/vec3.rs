use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec3f32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// https://github.com/rust-lang/rust/issues/21188
impl Add<Vec3f32> for Vec3f32 {
    type Output = Vec3f32; // https://stackoverflow.com/questions/39115363/why-do-rusts-operators-have-the-type-output-variable
    #[inline(always)]
    fn add(self, other: Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
impl<'a> Add<Vec3f32> for &'a Vec3f32 {
    type Output = Vec3f32; // https://stackoverflow.com/questions/39115363/why-do-rusts-operators-have-the-type-output-variable
    #[inline(always)]
    fn add(self, other: Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
impl<'a> Add<&'a Vec3f32> for Vec3f32 {
    type Output = Vec3f32; // https://stackoverflow.com/questions/39115363/why-do-rusts-operators-have-the-type-output-variable
    #[inline(always)]
    fn add(self, other: &Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
impl<'a> Add<&'a Vec3f32> for &'a Vec3f32 {
    type Output = Vec3f32; // https://stackoverflow.com/questions/39115363/why-do-rusts-operators-have-the-type-output-variable
    #[inline(always)]
    fn add(self, other: &Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<f32> for Vec3f32 {
    type Output = Vec3f32;
    #[inline(always)]
    fn add(self, coef: f32) -> Vec3f32 {
        Vec3f32::new(self.x + coef, self.y + coef, self.z + coef)
    }
}
impl<'a> Add<f32> for &'a Vec3f32 {
    type Output = Vec3f32;
    #[inline(always)]
    fn add(self, coef: f32) -> Vec3f32 {
        Vec3f32::new(self.x + coef, self.y + coef, self.z + coef)
    }
}
impl<'a> Add<&'a f32> for Vec3f32 {
    type Output = Vec3f32;
    #[inline(always)]
    fn add(self, coef: &f32) -> Vec3f32 {
        Vec3f32::new(self.x + *coef, self.y + *coef, self.z + *coef)
    }
}
impl<'a> Add<&'a f32> for &'a Vec3f32 {
    type Output = Vec3f32;
    #[inline(always)]
    fn add(self, coef: &f32) -> Vec3f32 {
        Vec3f32::new(self.x + *coef, self.y + *coef, self.z + *coef)
    }
}

impl Sub<Vec3f32> for Vec3f32 {
    type Output = Vec3f32;
    #[inline(always)]
    fn sub(self, other: Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}
impl<'a> Sub<Vec3f32> for &'a Vec3f32 {
    type Output = Vec3f32;
    #[inline(always)]
    fn sub(self, other: Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}
impl<'a> Sub<&'a Vec3f32> for Vec3f32 {
    type Output = Vec3f32;
    #[inline(always)]
    fn sub(self, other: &Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}
impl<'a> Sub<&'a Vec3f32> for &'a Vec3f32 {
    type Output = Vec3f32;
    #[inline(always)]
    fn sub(self, other: &Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub<f32> for Vec3f32 {
    type Output = Vec3f32;
    #[inline(always)]
    fn sub(self, coef: f32) -> Vec3f32 {
        Vec3f32::new(self.x - coef, self.y - coef, self.z - coef)
    }
}
impl<'a> Sub<f32> for &'a Vec3f32 {
    type Output = Vec3f32;
    #[inline(always)]
    fn sub(self, coef: f32) -> Vec3f32 {
        Vec3f32::new(self.x - coef, self.y - coef, self.z - coef)
    }
}
impl<'a> Sub<&'a f32> for Vec3f32 {
    type Output = Vec3f32;
    #[inline(always)]
    fn sub(self, coef: &f32) -> Vec3f32 {
        Vec3f32::new(self.x - *coef, self.y - *coef, self.z - *coef)
    }
}
impl<'a> Sub<&'a f32> for &'a Vec3f32 {
    type Output = Vec3f32;
    #[inline(always)]
    fn sub(self, coef: &f32) -> Vec3f32 {
        Vec3f32::new(self.x - *coef, self.y - *coef, self.z - *coef)
    }
}

impl Mul<Vec3f32> for Vec3f32 {
    type Output = Vec3f32;
    #[inline(always)]
    fn mul(self, other: Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}
impl<'a> Mul<Vec3f32> for &'a Vec3f32 {
    type Output = Vec3f32;
    #[inline(always)]
    fn mul(self, other: Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}
impl<'a> Mul<&'a Vec3f32> for Vec3f32 {
    type Output = Vec3f32;
    #[inline(always)]
    fn mul(self, other: &Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}
impl<'a> Mul<&'a Vec3f32> for &'a Vec3f32 {
    type Output = Vec3f32;
    #[inline(always)]
    fn mul(self, other: &Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Mul<f32> for Vec3f32 {
    type Output = Vec3f32;
    #[inline(always)]
    fn mul(self, coef: f32) -> Vec3f32 {
        Vec3f32::new(self.x * coef, self.y * coef, self.z * coef)
    }
}
impl<'a> Mul<f32> for &'a Vec3f32 {
    type Output = Vec3f32;
    #[inline(always)]
    fn mul(self, coef: f32) -> Vec3f32 {
        Vec3f32::new(self.x * coef, self.y * coef, self.z * coef)
    }
}
impl<'a> Mul<&'a f32> for Vec3f32 {
    type Output = Vec3f32;
    #[inline(always)]
    fn mul(self, coef: &f32) -> Vec3f32 {
        Vec3f32::new(self.x * *coef, self.y * *coef, self.z * *coef)
    }
}
impl<'a> Mul<&'a f32> for &'a Vec3f32 {
    type Output = Vec3f32;
    #[inline(always)]
    fn mul(self, coef: &f32) -> Vec3f32 {
        Vec3f32::new(self.x * *coef, self.y * *coef, self.z * *coef)
    }
}

impl Div<Vec3f32> for Vec3f32 {
    type Output = Vec3f32;
    #[inline(always)]
    fn div(self, other: Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}
impl<'a> Div<Vec3f32> for &'a Vec3f32 {
    type Output = Vec3f32;
    #[inline(always)]
    fn div(self, other: Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}
impl<'a> Div<&'a Vec3f32> for Vec3f32 {
    type Output = Vec3f32;
    #[inline(always)]
    fn div(self, other: &Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}
impl<'a> Div<&'a Vec3f32> for &'a Vec3f32 {
    type Output = Vec3f32;
    #[inline(always)]
    fn div(self, other: &Vec3f32) -> Vec3f32 {
        Vec3f32::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

impl Div<f32> for Vec3f32 {
    type Output = Vec3f32;
    #[inline(always)]
    fn div(self, coef: f32) -> Vec3f32 {
        Vec3f32::new(self.x / coef, self.y / coef, self.z / coef)
    }
}
impl<'a> Div<f32> for &'a Vec3f32 {
    type Output = Vec3f32;
    #[inline(always)]
    fn div(self, coef: f32) -> Vec3f32 {
        Vec3f32::new(self.x / coef, self.y / coef, self.z / coef)
    }
}
impl<'a> Div<&'a f32> for Vec3f32 {
    type Output = Vec3f32;
    #[inline(always)]
    fn div(self, coef: &f32) -> Vec3f32 {
        Vec3f32::new(self.x / *coef, self.y / *coef, self.z / *coef)
    }
}
impl<'a> Div<&'a f32> for &'a Vec3f32 {
    type Output = Vec3f32;
    #[inline(always)]
    fn div(self, coef: &f32) -> Vec3f32 {
        Vec3f32::new(self.x / *coef, self.y / *coef, self.z / *coef)
    }
}

impl Vec3f32 {
    #[inline(always)]
    pub fn new(x: f32, y: f32, z: f32) -> Vec3f32 {
        Vec3f32 { x, y, z }
    }

    #[inline(always)]
    pub fn norm(self: &Self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    #[inline(always)]
    pub fn normalize(self: &mut Self) {
        let length = ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt();
        self.x /= length;
        self.y /= length;
        self.z /= length;
    }

    #[inline(always)] // NOTE: compiler seem to not inline method as much as free function
    pub fn dot_product(self: &Self, other: &Vec3f32) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}
