pub(crate) fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub trait Calc {
    fn add(a: i32, b: i32) -> i32;
}

pub struct DesktopCalc;

impl Calc for DesktopCalc {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}