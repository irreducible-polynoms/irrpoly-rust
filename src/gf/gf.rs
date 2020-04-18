use std::boxed::Box;
use std::sync::Arc;
use std::ops;
use std::fmt;
use std::vec;
use std::cmp;

#[derive(Debug)]
pub enum GfError {
    EmptyField,
    ZeroField,
    TooLargeField(usize),
    NotAField(usize),
}

impl fmt::Display for GfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GfError::EmptyField =>
                write!(f, "GF[0] is empty"),
            GfError::ZeroField =>
                write!(f, "GF[1] can contain only zero"),
            GfError::TooLargeField(base) =>
                write!(f, "GF[{}] is too large", base),
            GfError::NotAField(base) =>
                write!(f, "field GF[{}] do not exist", base),
        }
    }
}

type Result<T> = std::result::Result<T, GfError>;

#[derive(Debug)]
pub struct GfBase {
    base: usize,
    inv:  Box<[usize]>,
}

#[derive(Debug, Clone)]
pub struct Gf(Arc<GfBase>);

impl ops::Deref for Gf {
    type Target = Arc<GfBase>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl Gf {
    pub fn new(base: usize) -> Result<Gf> {
        let field = GfBase::new(base)?;
        Ok(Gf(Arc::new(field)))
    }
    pub fn clone(field: &Gf) -> Self { Gf(Arc::clone(&field.0)) }
}

impl cmp::PartialEq for Gf {
    fn eq (&self, other: &Gf) -> bool { self.base == other.base }
}

impl cmp::Eq for Gf {}

impl GfBase {
    fn new(base: usize) -> Result<GfBase> {
        if base == 0 { return Err(GfError::EmptyField) }
        if base == 1 { return Err(GfError::ZeroField) }
        // guarantees that base could be safely converted to isize
        if usize::max_value() / (base - 1) < (base - 1) {
            return Err(GfError::TooLargeField(base))
        }

        let i_base = base as isize;
        fn inv_calc(base: isize, val: isize) -> Result<usize> {
            let (mut u0, mut u1, mut u2) = (base, 1isize, 0isize);
            let (mut v0, mut v1, mut v2) = (val, 0isize, 1isize);
            let (mut w0, mut w1, mut w2); let mut q;

            while v0 > 0 {
                q = u0 / v0;
                w0 = u0 - q * v0; w1 = u1 - q * v1; w2 = u2 - q * v2;
                u0 = v0; u1 = v1; u2 = v2; v0 = w0; v1 = w1; v2 = w2;
            }
            if u0 > 1 { return Err(GfError::NotAField(base as usize)) }
            Ok(if u2 < 0 { base + u2 } else { u2 } as usize)
        }

        let mut inv = vec![0; base];
        inv[1] = 1;
        for i in 2..base {
            if inv[i] != 0 { continue; }
            let tmp = inv_calc(i_base, i as isize)?;
            inv[i] = tmp;
            inv[tmp] = i;
        }

        let inv = inv.into_boxed_slice();
        Ok(GfBase {base, inv})
    }

    pub fn base(&self) -> usize { self.base }

    pub fn mul_inv(&self, val: usize) -> Option<usize> {
        match val % self.base {
            0 => None,
            i => Some(self.inv[i]),
        }
    }
}
