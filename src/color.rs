use std::ops::{Add, Div, Mul, Sub};
#[derive(Debug, Clone, Copy)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Rgba {
    #[inline(always)]
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Rgba {
        Rgba { r, g, b, a }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

// https://github.com/rust-lang/rust/issues/21188
impl Add<Rgb> for Rgb {
    type Output = Rgb; // https://stackoverflow.com/questions/39115363/whg-do-rusts-operators-have-the-type-output-variable
    #[inline(always)]
    fn add(self, other: Rgb) -> Rgb {
        Rgb::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}
impl<'a> Add<Rgb> for &'a Rgb {
    type Output = Rgb; // https://stackoverflow.com/questions/39115363/whg-do-rusts-operators-have-the-type-output-variable
    #[inline(always)]
    fn add(self, other: Rgb) -> Rgb {
        Rgb::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}
impl<'a> Add<&'a Rgb> for Rgb {
    type Output = Rgb; // https://stackoverflow.com/questions/39115363/whg-do-rusts-operators-have-the-type-output-variable
    #[inline(always)]
    fn add(self, other: &Rgb) -> Rgb {
        Rgb::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}
impl<'a> Add<&'a Rgb> for &'a Rgb {
    type Output = Rgb; // https://stackoverflow.com/questions/39115363/whg-do-rusts-operators-have-the-type-output-variable
    #[inline(always)]
    fn add(self, other: &Rgb) -> Rgb {
        Rgb::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}

impl Add<f32> for Rgb {
    type Output = Rgb;
    #[inline(always)]
    fn add(self, coef: f32) -> Rgb {
        Rgb::new(self.r + coef, self.g + coef, self.b + coef)
    }
}
impl<'a> Add<f32> for &'a Rgb {
    type Output = Rgb;
    #[inline(always)]
    fn add(self, coef: f32) -> Rgb {
        Rgb::new(self.r + coef, self.g + coef, self.b + coef)
    }
}
impl<'a> Add<&'a f32> for Rgb {
    type Output = Rgb;
    #[inline(always)]
    fn add(self, coef: &f32) -> Rgb {
        Rgb::new(self.r + *coef, self.g + *coef, self.b + *coef)
    }
}
impl<'a> Add<&'a f32> for &'a Rgb {
    type Output = Rgb;
    #[inline(always)]
    fn add(self, coef: &f32) -> Rgb {
        Rgb::new(self.r + *coef, self.g + *coef, self.b + *coef)
    }
}

impl Sub<Rgb> for Rgb {
    type Output = Rgb;
    #[inline(always)]
    fn sub(self, other: Rgb) -> Rgb {
        Rgb::new(self.r - other.r, self.g - other.g, self.b - other.b)
    }
}
impl<'a> Sub<Rgb> for &'a Rgb {
    type Output = Rgb;
    #[inline(always)]
    fn sub(self, other: Rgb) -> Rgb {
        Rgb::new(self.r - other.r, self.g - other.g, self.b - other.b)
    }
}
impl<'a> Sub<&'a Rgb> for Rgb {
    type Output = Rgb;
    #[inline(always)]
    fn sub(self, other: &Rgb) -> Rgb {
        Rgb::new(self.r - other.r, self.g - other.g, self.b - other.b)
    }
}
impl<'a> Sub<&'a Rgb> for &'a Rgb {
    type Output = Rgb;
    #[inline(always)]
    fn sub(self, other: &Rgb) -> Rgb {
        Rgb::new(self.r - other.r, self.g - other.g, self.b - other.b)
    }
}

impl Sub<f32> for Rgb {
    type Output = Rgb;
    #[inline(always)]
    fn sub(self, coef: f32) -> Rgb {
        Rgb::new(self.r - coef, self.g - coef, self.b - coef)
    }
}
impl<'a> Sub<f32> for &'a Rgb {
    type Output = Rgb;
    #[inline(always)]
    fn sub(self, coef: f32) -> Rgb {
        Rgb::new(self.r - coef, self.g - coef, self.b - coef)
    }
}
impl<'a> Sub<&'a f32> for Rgb {
    type Output = Rgb;
    #[inline(always)]
    fn sub(self, coef: &f32) -> Rgb {
        Rgb::new(self.r - *coef, self.g - *coef, self.b - *coef)
    }
}
impl<'a> Sub<&'a f32> for &'a Rgb {
    type Output = Rgb;
    #[inline(always)]
    fn sub(self, coef: &f32) -> Rgb {
        Rgb::new(self.r - *coef, self.g - *coef, self.b - *coef)
    }
}

impl Mul<Rgb> for Rgb {
    type Output = Rgb;
    #[inline(always)]
    fn mul(self, other: Rgb) -> Rgb {
        Rgb::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}
impl<'a> Mul<Rgb> for &'a Rgb {
    type Output = Rgb;
    #[inline(always)]
    fn mul(self, other: Rgb) -> Rgb {
        Rgb::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}
impl<'a> Mul<&'a Rgb> for Rgb {
    type Output = Rgb;
    #[inline(always)]
    fn mul(self, other: &Rgb) -> Rgb {
        Rgb::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}
impl<'a> Mul<&'a Rgb> for &'a Rgb {
    type Output = Rgb;
    #[inline(always)]
    fn mul(self, other: &Rgb) -> Rgb {
        Rgb::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}

impl Mul<f32> for Rgb {
    type Output = Rgb;
    #[inline(always)]
    fn mul(self, coef: f32) -> Rgb {
        Rgb::new(self.r * coef, self.g * coef, self.b * coef)
    }
}
impl<'a> Mul<f32> for &'a Rgb {
    type Output = Rgb;
    #[inline(always)]
    fn mul(self, coef: f32) -> Rgb {
        Rgb::new(self.r * coef, self.g * coef, self.b * coef)
    }
}
impl<'a> Mul<&'a f32> for Rgb {
    type Output = Rgb;
    #[inline(always)]
    fn mul(self, coef: &f32) -> Rgb {
        Rgb::new(self.r * *coef, self.g * *coef, self.b * *coef)
    }
}
impl<'a> Mul<&'a f32> for &'a Rgb {
    type Output = Rgb;
    #[inline(always)]
    fn mul(self, coef: &f32) -> Rgb {
        Rgb::new(self.r * *coef, self.g * *coef, self.b * *coef)
    }
}

impl Div<Rgb> for Rgb {
    type Output = Rgb;
    #[inline(always)]
    fn div(self, other: Rgb) -> Rgb {
        Rgb::new(self.r / other.r, self.g / other.g, self.b / other.b)
    }
}
impl<'a> Div<Rgb> for &'a Rgb {
    type Output = Rgb;
    #[inline(always)]
    fn div(self, other: Rgb) -> Rgb {
        Rgb::new(self.r / other.r, self.g / other.g, self.b / other.b)
    }
}
impl<'a> Div<&'a Rgb> for Rgb {
    type Output = Rgb;
    #[inline(always)]
    fn div(self, other: &Rgb) -> Rgb {
        Rgb::new(self.r / other.r, self.g / other.g, self.b / other.b)
    }
}
impl<'a> Div<&'a Rgb> for &'a Rgb {
    type Output = Rgb;
    #[inline(always)]
    fn div(self, other: &Rgb) -> Rgb {
        Rgb::new(self.r / other.r, self.g / other.g, self.b / other.b)
    }
}

impl Div<f32> for Rgb {
    type Output = Rgb;
    #[inline(always)]
    fn div(self, coef: f32) -> Rgb {
        Rgb::new(self.r / coef, self.g / coef, self.b / coef)
    }
}
impl<'a> Div<f32> for &'a Rgb {
    type Output = Rgb;
    #[inline(always)]
    fn div(self, coef: f32) -> Rgb {
        Rgb::new(self.r / coef, self.g / coef, self.b / coef)
    }
}
impl<'a> Div<&'a f32> for Rgb {
    type Output = Rgb;
    #[inline(always)]
    fn div(self, coef: &f32) -> Rgb {
        Rgb::new(self.r / *coef, self.g / *coef, self.b / *coef)
    }
}
impl<'a> Div<&'a f32> for &'a Rgb {
    type Output = Rgb;
    #[inline(always)]
    fn div(self, coef: &f32) -> Rgb {
        Rgb::new(self.r / *coef, self.g / *coef, self.b / *coef)
    }
}

impl Rgb {
    #[inline(always)]
    pub fn new(r: f32, g: f32, b: f32) -> Rgb {
        Rgb { r, g, b }
    }
}
