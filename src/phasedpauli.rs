use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhasedPauliEnum {
    X(bool, bool),
    Y(bool, bool),
    Z(bool, bool),
    I(bool, bool),
}

impl fmt::Display for PhasedPauliEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PhasedPauliEnum::X(a, b) => write!(f, "X({},{})", a, b),
            PhasedPauliEnum::Y(a, b) => write!(f, "Y({},{})", a, b),
            PhasedPauliEnum::Z(a, b) => write!(f, "Z({},{})", a, b),
            PhasedPauliEnum::I(a, b) => write!(f, "I({},{})", a, b),
        }
    }
}

impl FromStr for PhasedPauliEnum {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Remove optional surrounding spaces
        let trimmed = s.trim();

        // Check if the string starts with a known variant
        if let Some(rest) = trimmed.strip_prefix('X') {
            let trimmed_rest = rest.trim_matches(|c| c == '(' || c == ')');
            let parts: Vec<bool> = trimmed_rest
                .split(',')
                .map(|x| x.trim().parse().unwrap())
                .collect();
            if parts.len() == 2 {
                return Ok(PhasedPauliEnum::X(parts[0], parts[1]));
            }
        } else if let Some(rest) = trimmed.strip_prefix('Y') {
            let trimmed_rest = rest.trim_matches(|c| c == '(' || c == ')');
            let parts: Vec<bool> = trimmed_rest
                .split(',')
                .map(|x| x.trim().parse().unwrap())
                .collect();
            if parts.len() == 2 {
                return Ok(PhasedPauliEnum::Y(parts[0], parts[1]));
            }
        } else if let Some(rest) = trimmed.strip_prefix('Z') {
            let trimmed_rest = rest.trim_matches(|c| c == '(' || c == ')');
            let parts: Vec<bool> = trimmed_rest
                .split(',')
                .map(|x| x.trim().parse().unwrap())
                .collect();
            if parts.len() == 2 {
                return Ok(PhasedPauliEnum::Z(parts[0], parts[1]));
            }
        } else if let Some(rest) = trimmed.strip_prefix('I') {
            let trimmed_rest = rest.trim_matches(|c| c == '(' || c == ')');
            let parts: Vec<bool> = trimmed_rest
                .split(',')
                .map(|x| x.trim().parse().unwrap())
                .collect();
            if parts.len() == 2 {
                return Ok(PhasedPauliEnum::I(parts[0], parts[1]));
            }
        }

        Err("Invalid PhasedPauliEnum format".into())
    }
}
