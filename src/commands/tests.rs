use super::{calc::DesktopCalc, calc::Calc, calc::add};

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_by_calculator() {
        assert_eq!(DesktopCalc::add(1, 2), 3);
    }
}
