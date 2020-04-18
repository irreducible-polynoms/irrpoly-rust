use crate::{Gf, GfNum, GfPoly};

use std::fmt::Write;
use std::vec;

#[test]
fn gf_poly_works() {
    let gf5 = Gf::new(5).unwrap();

    let mut output = String::new();
    let poly = GfPoly::new(&gf5, 3);
    write!(&mut output, "{}", poly).unwrap();
    assert_eq!(output, "[ 3 ]");

    let mut output = String::new();
    let num = GfNum::new(&gf5, 5);
    let poly = GfPoly::from_gf_num(num);
    write!(&mut output, "{}", poly).unwrap();
    assert_eq!(output, "[  ]");

    let mut output = String::new();
    let poly = GfPoly::from_vec(&gf5, vec![1, 2, 3, 0, 0]);
    write!(&mut output, "{}", poly).unwrap();
    assert_eq!(output, "[ 1, 2, 3 ]");
    assert_eq!(poly.field(), &gf5);
    assert_eq!(poly.poly(), &GfNum::from_vec(&gf5, vec![1, 2, 3])[..]);
    assert_eq!(poly.len(), 3);
    assert_eq!(poly.deg(), 2);
    assert_eq!(poly.into_vec(), vec![1, 2, 3]);

    let mut poly = GfPoly::from_vec(&gf5, vec![1, 2, 3, 0, 0]);
    assert_eq!(poly.into_gf_num_vec(), GfNum::from_vec(&gf5, vec![1, 2, 3]));

    let mut poly = GfPoly::from_vec(&gf5, vec![1, 2, 3, 0, 0]);
    poly.set_zero();
    assert!(poly.is_zero());
}

#[test]
#[should_panic]
fn gf_poly_degree_for_zero() {
    let gf5 = Gf::new(5).unwrap();
    let poly = GfPoly::new(&gf5, 5);
    poly.deg();
}
