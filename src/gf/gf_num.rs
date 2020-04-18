use crate::Gf;

use std::vec::Vec;
use std::fmt;
use std::ops;
use std::cmp;

#[derive(Debug, Clone)]
pub struct GfNum {
    field: Gf,
    num:   usize,
}

impl fmt::Display for GfNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}

impl GfNum {
    pub fn new(field: &Gf, value: usize) -> GfNum {
        let field = Gf::clone(field);
        let value = value % field.base();
        GfNum {field, num: value }
    }

    pub fn from_vec(field: &Gf, value: Vec<usize>) -> Vec<GfNum> {
        value.iter().map(|x| GfNum::new(field, *x) ).collect()
    }

    pub fn into_num(self) -> usize { self.num }

    pub fn field(&self) -> &Gf { &self.field }

    pub fn num(&self) -> usize { self.num }

    pub fn mul_inv(&self) -> Self {
        let field = Gf::clone(self.field());
        let value = field.mul_inv(self.num)
            .expect("Multiplicative inverse for zero do not exist");
        GfNum {field, num: value }
    }

    pub fn is_zero(&self) -> bool { self.num == 0 }
}

impl ops::Add<GfNum> for GfNum {
    type Output = GfNum;

    fn add(self, rhs: GfNum) -> GfNum {
        debug_assert_eq!(self.field(), rhs.field(), "Numbers from different fields");
        let base = self.field().base();
        GfNum {
            field: self.field,
            num: (self.num + rhs.num) % base
        }
    }
}

impl ops::Add<usize> for GfNum {
    type Output = GfNum;

    fn add(self, rhs: usize) -> GfNum {
        let base = self.field().base();
        GfNum {
            field: self.field,
            num: (self.num + (rhs % base)) % base
        }
    }
}

impl ops::Add<GfNum> for usize {
    type Output = GfNum;

    fn add(self, rhs: GfNum) -> GfNum {
        let base = rhs.field().base();
        GfNum {
            field: rhs.field,
            num: (rhs.num + (self % base)) % base
        }
    }
}

impl ops::AddAssign<GfNum> for GfNum {
    fn add_assign(&mut self, other: GfNum) {
        debug_assert_eq!(self.field(), other.field(), "Numbers from different fields");
        let base = self.field().base();
        self.num = (self.num + other.num) % base;
    }
}

impl ops::AddAssign<usize> for GfNum {
    fn add_assign(&mut self, other: usize) {
        let base = self.field().base();
        self.num = (self.num + (other % base)) % base;
    }
}

impl ops::Sub<GfNum> for GfNum {
    type Output = GfNum;

    fn sub(self, rhs: GfNum) -> GfNum {
        debug_assert_eq!(self.field(), rhs.field(), "Numbers from different fields");
        let base = self.field().base();
        GfNum {
            field: self.field,
            num: (base + self.num - rhs.num) % base
        }
    }
}

impl ops::Sub<usize> for GfNum {
    type Output = GfNum;

    fn sub(self, rhs: usize) -> GfNum {
        let base = self.field().base();
        GfNum {
            field: self.field,
            num: (base + self.num - (rhs % base)) % base
        }
    }
}

impl ops::Sub<GfNum> for usize {
    type Output = GfNum;

    fn sub(self, rhs: GfNum) -> GfNum {
        let base = rhs.field().base();
        GfNum {
            field: rhs.field,
            num: (base + rhs.num - (self % base)) % base
        }
    }
}

impl ops::SubAssign<GfNum> for GfNum {
    fn sub_assign(&mut self, other: GfNum) {
        debug_assert_eq!(self.field(), other.field(), "Numbers from different fields");
        let base = self.field().base();
        self.num = (base + self.num - other.num) % base;
    }
}

impl ops::SubAssign<usize> for GfNum {
    fn sub_assign(&mut self, other: usize) {
        let base = self.field().base();
        self.num = (base + self.num - (other % base)) % base;
    }
}

impl ops::Neg for GfNum {
    type Output = GfNum;

    fn neg(self) -> GfNum {
        let base = self.field().base();
        GfNum {
            field: self.field,
            num: base - self.num
        }
    }
}

impl ops::Mul<GfNum> for GfNum {
    type Output = GfNum;

    fn mul(self, rhs: GfNum) -> GfNum {
        debug_assert_eq!(self.field(), rhs.field(), "Numbers from different fields");
        let base = self.field().base();
        GfNum {
            field: self.field,
            num: (self.num * rhs.num) % base
        }
    }
}

impl ops::Mul<usize> for GfNum {
    type Output = GfNum;

    fn mul(self, rhs: usize) -> GfNum {
        let base = self.field().base();
        GfNum {
            field: self.field,
            num: (self.num * (rhs % base)) % base
        }
    }
}

impl ops::Mul<GfNum> for usize {
    type Output = GfNum;

    fn mul(self, rhs: GfNum) -> GfNum {
        let base = rhs.field().base();
        GfNum {
            field: rhs.field,
            num: (rhs.num * (self % base)) % base
        }
    }
}

impl ops::MulAssign<GfNum> for GfNum {
    fn mul_assign(&mut self, other: GfNum) {
        debug_assert_eq!(self.field(), other.field(), "Numbers from different fields");
        let base = self.field().base();
        self.num = (self.num * other.num) % base;
    }
}

impl ops::MulAssign<usize> for GfNum {
    fn mul_assign(&mut self, other: usize) {
        let base = self.field().base();
        self.num = (self.num * (other % base)) % base;
    }
}

impl ops::Div<GfNum> for GfNum {
    type Output = GfNum;

    fn div(self, rhs: GfNum) -> GfNum {
        debug_assert_eq!(self.field(), rhs.field(), "Numbers from different fields");
        let base = self.field().base();
        let inv = self.field().mul_inv(rhs.num).unwrap();
        GfNum {
            field: self.field,
            num: (self.num * inv) % base
        }
    }
}

impl ops::Div<usize> for GfNum {
    type Output = GfNum;

    fn div(self, rhs: usize) -> GfNum {
        let base = self.field().base();
        let inv = self.field().mul_inv(rhs).unwrap();
        GfNum {
            field: self.field,
            num: (self.num * inv) % base
        }
    }
}

impl ops::Div<GfNum> for usize {
    type Output = GfNum;

    fn div(self, rhs: GfNum) -> GfNum {
        let base = rhs.field().base();
        let inv = rhs.field().mul_inv(rhs.num).unwrap();
        GfNum {
            field: rhs.field,
            num: (self * inv) % base
        }
    }
}

impl ops::DivAssign<GfNum> for GfNum {
    fn div_assign(&mut self, other: GfNum) {
        debug_assert_eq!(self.field(), other.field(), "Numbers from different fields");
        let base = self.field().base();
        let inv = self.field().mul_inv(other.num).unwrap();
        self.num = (self.num * inv) % base;
    }
}

impl ops::DivAssign<usize> for GfNum {
    fn div_assign(&mut self, other: usize) {
        let base = self.field().base();
        let inv = self.field().mul_inv(other).unwrap();
        self.num = (self.num * inv) % base;
    }
}

impl cmp::PartialEq<GfNum> for GfNum {
    fn eq (&self, other: &GfNum) -> bool {
        debug_assert_eq!(self.field(), other.field(), "Numbers from different fields");
        self.num() == other.num()
    }
}

impl cmp::PartialEq<usize> for GfNum {
    fn eq (&self, other: &usize) -> bool {
        let base = self.field().base();
        self.num() == other % base
    }
}

impl cmp::PartialEq<GfNum> for usize {
    fn eq (&self, other: &GfNum) -> bool {
        let base = other.field().base();
        self % base == other.num()
    }
}

impl cmp::Eq for GfNum {}

impl cmp::PartialOrd<GfNum> for GfNum {
    fn partial_cmp(&self, other: &GfNum) -> Option<cmp::Ordering> {
        debug_assert_eq!(self.field(), other.field(), "Numbers from different fields");
        self.num.partial_cmp(&other.num)
    }
}

impl cmp::PartialOrd<usize> for GfNum {
    fn partial_cmp(&self, other: &usize) -> Option<cmp::Ordering> {
        let base = self.field().base();
        self.num.partial_cmp(&(other % base))
    }
}

impl cmp::PartialOrd<GfNum> for usize {
    fn partial_cmp(&self, other: &GfNum) -> Option<cmp::Ordering> {
        let base = other.field().base();
        (self % base).partial_cmp(&other.num)
    }
}

impl cmp::Ord for GfNum {
    fn cmp(&self, other: &GfNum) -> cmp::Ordering {
        debug_assert_eq!(self.field(), other.field(), "Numbers from different fields");
        self.num.cmp(&other.num)
    }
}
