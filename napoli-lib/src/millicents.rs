pub struct Millicents(i64);

#[derive(Debug)]
pub enum MillicentsError {
    Negative,
    ParseError,
}

impl Millicents {
    pub fn from_euro_float(euros: f64) -> Result<Self, MillicentsError> {
        if euros < 0.0 {
            return Err(MillicentsError::Negative);
        }

        Ok(Millicents((euros * 100000.0) as i64))
    }

    pub fn from_euro_human(s: &str) -> Result<Self, MillicentsError> {
        let f: f64 = s.parse::<f64>().map_err(|_| MillicentsError::ParseError)?;
        Self::from_euro_float(f)
    }

    pub fn from_raw(i: i64) -> Result<Self, MillicentsError> {
        if i < 0 {
            return Err(MillicentsError::Negative);
        }
        Ok(Millicents(i))
    }

    pub fn raw(&self) -> i64 {
        self.0
    }

    pub fn zero() -> Self {
        Millicents(0)
    }

    pub fn to_euro_float(&self) -> f64 {
        let (euros, cents) = Self::millicents_to_euro(self.0);
        euros as f64 + cents as f64 / 100.0
    }

    pub fn to_euro_tuple(&self) -> (i64, i64) {
        Millicents::millicents_to_euro(self.0)
    }

    pub fn is_zero(&self) -> bool {
        return self.0 == 0;
    }

    fn millicents_to_euro(mc: i64) -> (i64, i64) {
        let euros = mc / 100000;
        let cents = divide_and_round(mc - euros * 100000, 1000);
        (euros, cents)
    }
}

pub fn divide_and_round(dividend: i64, divisor: i64) -> i64 {
    if dividend.is_positive() && divisor.is_positive() {
        // Math hacks powered by https://stackoverflow.com/a/17005390
        (dividend - 1) / divisor + 1
    } else {
        ((dividend as f64) / (divisor as f64)).round() as i64
    }
}

#[cfg(test)]
mod tests {
    use crate::millicents::Millicents;

    #[test]
    fn test_millicent_to_euro() {
        let mc_price = 13 * 100000 + 37 * 1000; // 13.37 EUR
        assert_eq!(Millicents::millicents_to_euro(mc_price), (13, 37));
        assert_eq!(Millicents::millicents_to_euro(133736999), (1337, 37));
        assert_eq!(Millicents::millicents_to_euro(1337000), (13, 37));
    }

    #[test]
    fn millicents_to_euro_float() {
        assert_eq!(Millicents::from_raw(1300000).unwrap().to_euro_float(), 13.0);
        assert_eq!(
            Millicents::from_raw(1337000).unwrap().to_euro_float(),
            13.37
        );
    }
}
