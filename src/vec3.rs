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

    pub fn empty() -> Self {
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
                Vec3::empty() - self
            }
        }
    };
}

generate_arithmetic_2_arg!(Vec3, Vec3);
generate_arithmetic_2_arg!(&Vec3, Vec3);
generate_arithmetic_2_arg!(Vec3, &Vec3);
generate_arithmetic_2_arg!(&Vec3, &Vec3);
generate_arithmetic_1_arg!(Vec3);
generate_arithmetic_1_arg!(&Vec3);