use crate::{Gf, GfNum};

#[test]
fn gf_works() {
    let gf2 = Gf::new(2);
    assert!(gf2.is_ok());
    let gf2 = gf2.unwrap();
    assert_eq!(gf2.base(), 2);
}

#[test]
fn inverse_correct() {
    let gf5 = Gf::new(5).unwrap();
    assert!(gf5.mul_inv(0).is_none());
    assert_eq!(gf5.mul_inv(1).unwrap(), 1);
    assert_eq!(gf5.mul_inv(2).unwrap(), 3);
    assert_eq!(gf5.mul_inv(3).unwrap(), 2);
    assert_eq!(gf5.mul_inv(4).unwrap(), 4);
}

#[test]
fn not_a_field() {
    assert!(Gf::new(0).is_err());
    assert!(Gf::new(1).is_err());
    assert!(Gf::new(4).is_err());
    assert!(Gf::new(isize::max_value() as usize).is_err());
}

#[test]
fn gfn_works() {
    let gf5 = Gf::new(5).unwrap();
    let num = GfNum::new(&gf5, 7);
    assert_eq!(num.field(), &gf5);
    assert_eq!(num.num(), 2);
}

#[test]
#[should_panic]
fn mul_inv_for_zero() {
    let gf2 = Gf::new(2).unwrap();
    let num = GfNum::new(&gf2, 0);
    num.mul_inv();
}

#[test]
fn gfn_sum_works() {
    let gf5 = Gf::new(5).unwrap();

    let gfn3 = GfNum::new(&gf5, 3);
    let gfn4 = GfNum::new(&gf5, 4);

    let gfn2 = gfn3 + gfn4;
    assert_eq!(gfn2.num(), 2);
    let gfn3 = gfn2 + 1;
    assert_eq!(gfn3.num(), 3);
    let gfn0 = 2 + gfn3;
    assert_eq!(gfn0.num(), 0);

    let mut num = gfn0;
    let gfn1 = GfNum::new(&gf5, 1);

    num += gfn1;
    assert_eq!(num.num(), 1);
    num += 1;
    assert_eq!(num.num(), 2);
}

#[test]
fn gfn_sub_works() {
    let gf5 = Gf::new(5).unwrap();

    let gfn3 = GfNum::new(&gf5, 3);
    let gfn4 = GfNum::new(&gf5, 4);

    let gfn4 = gfn3 - gfn4;
    assert_eq!(gfn4.num(), 4);
    let gfn3 = gfn4 - 1;
    assert_eq!(gfn3.num(), 3);
    let gfn1 = 2 - gfn3;
    assert_eq!(gfn1.num(), 1);

    let mut num = gfn1;
    let gfn1 = GfNum::new(&gf5, 1);

    num -= gfn1;
    assert_eq!(num.num(), 0);
    num -= 2;
    assert_eq!(num.num(), 3);
}

#[test]
fn gfn_neg_works() {
    let gf5 = Gf::new(5).unwrap();

    let gfn4 = GfNum::new(&gf5, 4);

    let gfn1 = -gfn4;
    assert_eq!(gfn1.num(), 1);
}

#[test]
fn gfn_mul_works() {
    let gf5 = Gf::new(5).unwrap();

    let gfn3 = GfNum::new(&gf5, 3);
    let gfn4 = GfNum::new(&gf5, 4);

    let gfn2 = gfn3 * gfn4;
    assert_eq!(gfn2.num(), 2);
    let gfn4 = gfn2 * 2;
    assert_eq!(gfn4.num(), 4);
    let gfn3 = 2 * gfn4;
    assert_eq!(gfn3.num(), 3);

    let mut num = gfn3;
    let gfn2 = GfNum::new(&gf5, 2);

    num *= gfn2;
    assert_eq!(num.num(), 1);
    num *= 0;
    assert_eq!(num.num(), 0);
}

#[test]
fn gfn_div_works() {
    let gf5 = Gf::new(5).unwrap();

    let gfn3 = GfNum::new(&gf5, 3);
    let gfn4 = GfNum::new(&gf5, 4);

    let gfn2 = gfn3 / gfn4;
    assert_eq!(gfn2.num(), 2);
    let gfn1 = gfn2 / 2;
    assert_eq!(gfn1.num(), 1);
    let gfn3 = 3 / gfn1;
    assert_eq!(gfn3.num(), 3);

    let mut num = gfn3;
    let gfn2 = GfNum::new(&gf5, 2);

    num /= gfn2;
    assert_eq!(num.num(), 4);
    num /= 2;
    assert_eq!(num.num(), 2);
}

#[test]
#[should_panic]
fn gfn_division_by_zero() {
    let gf2 = Gf::new(2).unwrap();
    let mut num = GfNum::new(&gf2, 0);
    num /= 0;
}

#[test]
fn gfn_comparison() {
    let gf5 = Gf::new(5).unwrap();

    let gfn3 = GfNum::new(&gf5, 3);
    let gfn4 = GfNum::new(&gf5, 4);

    assert_eq!(gfn3, gfn3);
    assert_eq!(gfn3, 8);
    assert_eq!(8, gfn3);

    assert!(gfn3 < gfn4);
    assert!(gfn3 < 9);
    assert!(8 < gfn4);

    assert!(!gfn3.is_zero())
}