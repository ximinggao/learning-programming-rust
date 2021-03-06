#[derive(Clone, Copy, Debug)]
struct Complex<T> {
    re: T,
    im: T,
}

use std::ops::Add;

#[cfg(skip)]
impl Add for Complex<i32> {
    type Output = Complex<i32>;
    fn add(self, rhs: Self) -> Self {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

#[cfg(skip)]
impl<L, R, O> Add<Complex<R>> for Complex<L>
where
    L: Add<R, Output = O>,
{
    type Output = Complex<O>;
    fn add(self, rhs: Complex<R>) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + self.im,
        }
    }
}

#[cfg(skip)]
impl<'a, P, RHS> Add for &'a Complex<P>
where
    P: Add<Output = P>,
    RHS: AsRef<Complex<P>>,
{
    type Output = Complex<P>;
    fn add(self, rhs: RHS) -> Self::Output {
        let rhs = rhs.as_ref();
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

use std::ops::Mul;
use std::ops::Sub;

impl<T> Mul<Complex<T>> for Complex<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Complex {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

#[test]
fn test() {
    let z = Complex { re: -2, im: 6 };
    let c = Complex { re: 1, im: 2 };
    assert_eq!(z + c, Complex { re: -1, im: 8 });
    assert_eq!(z * c, Complex { re: -14, im: 2 });
    assert_eq!(z.add(c), Complex { re: -1, im: 8 });
}

#[test]
fn test_explicit() {
    use std::ops::Add;

    assert_eq!(4.125f32.add(5.75), 9.875);
    assert_eq!(10.add(20), 30);
}

impl Add<Complex<f64>> for f64 {
    type Output = Complex<f64>;
    fn add(self, rhs: Complex<f64>) -> Complex<f64> {
        Complex {
            re: rhs.re + self,
            im: rhs.im,
        }
    }
}

#[test]
fn add_complex_to_real() {
    assert_eq!(
        30f64
            + Complex {
                re: 10.0f64,
                im: 20.0
            },
        Complex { re: 40.0, im: 20.0 }
    );
}

use std::ops::Neg;

impl<T, O> Neg for Complex<T>
where
    T: Neg<Output = O>,
{
    type Output = Complex<O>;
    fn neg(self) -> Complex<O> {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

#[test]
fn negate_complex() {
    let z = Complex { re: 3, im: 4 };
    assert_eq!(-z, Complex { re: -3, im: -4 });
}

use std::ops::AddAssign;

impl<T> AddAssign for Complex<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Complex<T>) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

#[test]
fn compound_assignment() {
    let mut z = Complex { re: 5, im: 6 };
    z += Complex { re: 7, im: 8 };
    assert_eq!(z, Complex { re: 12, im: 14 });

    let mut title = "Love".to_string();
    title += ", Actually";
    assert_eq!(title, "Love, Actually");
}

impl<T: PartialEq> PartialEq for Complex<T> {
    fn eq(&self, other: &Complex<T>) -> bool {
        self.re == other.re && self.im == other.im
    }
}

impl<T: Eq> Eq for Complex<T> {}

#[test]
fn comparison() {
    let x = Complex { re: 5, im: 2 };
    let y = Complex { re: 2, im: 5 };
    assert_eq!(x * y, Complex { re: 0, im: 29 });
}

use std::fmt;

impl fmt::Display for Complex<f64> {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        let (r, i) = (self.re, self.im);
        if dest.alternate() {
            let abs = f64::sqrt(r * r + i * i);
            let angle = f64::atan2(i, r) / std::f64::consts::PI * 180.0;
            write!(dest, "{} ∠ {}°", abs, angle)
        } else {
            let i_sign = if i < 0.0 { '-' } else { '+' };
            write!(dest, "{} {} {}i", r, i_sign, f64::abs(i))
        }
    }
}

#[test]
fn custom_display_impl() {
    let one_twenty = Complex {
        re: -0.5,
        im: 0.866,
    };
    assert_eq!(format!("{}", one_twenty), "-0.5 + 0.866i");

    let two_forty = Complex {
        re: -0.5,
        im: -0.866,
    };
    assert_eq!(format!("{}", two_forty), "-0.5 - 0.866i");

    let ninety = Complex { re: 0.0, im: 2.0 };
    assert_eq!(format!("{}", ninety), "0 + 2i");
    assert_eq!(format!("{:#}", ninety), "2 ∠ 90°");
}

struct Selector<T> {
    elements: Vec<T>,
    current: usize,
}

use std::ops::{Deref, DerefMut};

impl<T> Deref for Selector<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.elements[self.current]
    }
}

#[test]
fn test_selector() {
    let mut s = Selector {
        elements: vec!['x', 'y', 'z'],
        current: 2,
    };

    assert_eq!(*s, 'z');
    assert!(s.is_alphabetic());
    *s = 'w';
    assert_eq!(s.elements, ['x', 'y', 'w']);

    fn show_it(thing: &str) {
        println!("{}", thing);
    }

    use std::fmt::Display;
    fn show_it_generic<T: Display>(thing: T){
        println!("{}", thing);
    }

    let ss = Selector {
        elements: vec!["good", "bad", "ugly"],
        current: 0,
    };
    show_it(&ss);
    show_it_generic(&ss as &str);
}
