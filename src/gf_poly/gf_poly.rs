use crate::{Gf, GfNum};

use std::vec::Vec;
use std::fmt;
use std::ops;
use std::cmp;

#[derive(Debug, Clone)]
pub struct GfPoly {
    field: Gf,
    poly:  Vec<GfNum>,
}

impl fmt::Display for GfPoly {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ ")?;
        for i in 0..self.poly.len() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{}", self.poly[i])?;
        }
        write!(f, " ]")
    }
}

impl GfPoly {
    pub fn new(field: &Gf, value: usize) -> GfPoly {
        let mut poly = GfPoly {
            field: Gf::clone(field),
            poly:  vec![GfNum::new(field, value)]
        };
        poly.normalize();
        poly
    }

    pub fn from_gf_num(value: GfNum) -> GfPoly {
        let mut poly = GfPoly {
            field: Gf::clone(value.field()),
            poly:  vec![value]
        };
        poly.normalize();
        poly
    }

    pub fn from_vec(field: &Gf, value: Vec<usize>) -> GfPoly {
        let mut poly = GfPoly {
            field: Gf::clone(field),
            poly:  GfNum::from_vec(field, value)
        };
        poly.normalize();
        poly
    }

    pub fn into_vec(self) -> Vec<usize> {
        self.poly.into_iter().map(|x| x.into_num() ).collect()
    }

    pub fn into_gf_num_vec(self) -> Vec<GfNum> { self.poly }

    pub fn field(&self) -> &Gf { &self.field }

    pub fn poly(&self) -> &[GfNum] { &self.poly }

    pub fn len(&self) -> usize { self.poly.len() }

    pub fn deg(&self) -> usize {
        assert!(self.poly.len() > 0, "Degree is undefined for zero polynomial");
        self.poly.len() - 1
    }

    pub fn is_zero(&self) -> bool { self.poly.is_empty() }

    pub fn set_zero(&mut self) { self.poly.clear(); }

    pub fn normalize(&mut self) {
        if let Some(i) = self.poly.iter().rposition(|x| !x.is_zero()) {
            self.poly.truncate(i + 1);
        } else { self.poly.clear(); }
    }
}
