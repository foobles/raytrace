use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    Index,
    IndexMut,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign
};

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    a: f64,
    b: f64,
    c: f64,
}

impl Vec3 {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Vec3 { a, b, c }
    }

    pub fn zero() -> Self {
        Vec3 { a: 0.0, b: 0.0, c: 0.0}
    }

    pub fn length_squared(self) -> f64 {
        self.a*self.a +
            self.b*self.b +
            self.c*self.c
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalize(self) -> Self {
        let factor = 1.0 / self.length();
        Vec3::new(self.a * factor, self.b * factor, self.c * factor)
    }

    pub fn x(self) -> f64 { self.a }
    pub fn y(self) -> f64 { self.b }
    pub fn z(self) -> f64 { self.c }

    pub fn r(self) -> f64 { self.a }
    pub fn g(self) -> f64 { self.b }
    pub fn b(self) -> f64 { self.c }

    pub fn dot(self, rhs: Vec3) -> f64 {
        let p = self * rhs;
        p.a + p.b + p.c 
    }

    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.b*rhs.c - self.c*rhs.b,
            self.c*rhs.a - self.a*rhs.c,
            self.a*rhs.b - self.b*rhs.a
        )
    }
}

macro_rules! generate_arithmetic_2_arg {
    ($lhs:ty, $rhs:ty) => {
        impl Add<$rhs> for $lhs {
            type Output = Vec3;

            fn add(self, rhs: $rhs) -> Self::Output {
                Vec3::new(self.a + rhs.a, self.b + rhs.b, self.c + rhs.c)
            }
        }

        impl Sub<$rhs> for $lhs {
            type Output = Vec3;

            fn sub(self, rhs: $rhs) -> Self::Output {
                Vec3::new(self.a - rhs.a, self.b - rhs.b, self.c - rhs.c)
            }
        }


        impl Mul<$rhs> for $lhs {
            type Output = Vec3;

            fn mul(self, rhs: $rhs) -> Self::Output {
                Vec3::new(self.a * rhs.a, self.b * rhs.b, self.c * rhs.c)
            }
        }

        impl Div<$rhs> for $lhs {
            type Output = Vec3;

            fn div(self, rhs: $rhs) -> Self::Output {
                Vec3::new(self.a / rhs.a, self.b / rhs.b, self.c / rhs.c)
            }
        }
    };
}

macro_rules! generate_arithmetic_1_arg {
    ($rhs:ty) => {
        impl AddAssign<$rhs> for Vec3 {
            fn add_assign(&mut self, rhs: $rhs) {
                *self = *self + rhs;
            }
        }

        impl SubAssign<$rhs> for Vec3 {
            fn sub_assign(&mut self, rhs: $rhs) {
                *self = *self - rhs;
            }
        }

        impl MulAssign<$rhs> for Vec3 {
            fn mul_assign(&mut self, rhs: $rhs) {
                *self = *self * rhs;
            }
        }

        impl DivAssign<$rhs> for Vec3 {
            fn div_assign(&mut self, rhs: $rhs) {
                *self = *self / rhs;
            }
        }

        impl Neg for $rhs {
            type Output = Vec3;
            fn neg(self) -> Self::Output {
                Vec3::zero() - self
            }
        }

        impl Mul<f64> for $rhs {
            type Output = Vec3;
            fn mul(self, rhs: f64) -> Self::Output{
                self * Vec3::new(rhs, rhs, rhs)
            }
        }


        impl Div<f64> for $rhs {
            type Output = Vec3;
            fn div(self, rhs: f64) -> Self::Output{
                let f = 1.0 / rhs;
                self * f
            }
        }
    };
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, idx: usize) -> &f64 {
        match idx {
            0 => &self.a,
            1 => &self.b,
            2 => &self.b,
            _ => panic!("Index {} is out of bounds for Vec3", idx)
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, idx: usize) -> &mut f64 {
        match idx {
            0 => &mut self.a,
            1 => &mut self.b,
            2 => &mut self.b,
            _ => panic!("Index {} is out of bounds for Vec3", idx)
        }
    }
}


generate_arithmetic_2_arg!(Vec3, Vec3);
generate_arithmetic_2_arg!(&Vec3, Vec3);
generate_arithmetic_2_arg!(Vec3, &Vec3);
generate_arithmetic_2_arg!(&Vec3, &Vec3);
generate_arithmetic_1_arg!(Vec3);
generate_arithmetic_1_arg!(&Vec3);