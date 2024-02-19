#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Precedence {
    PrecNone,
    PrecAssignment,
    PrecOr,
    PrecAnd,
    PrecEquality,
    PrecComparison,
    PrecTerm,
    PrecFactor,
    PrecUnary,
    PrecCall,
    PrecPrimary,
}

impl From<usize> for Precedence {
    fn from(value: usize) -> Self {
        match value {
            0 => Precedence::PrecNone,
            1 => Precedence::PrecAssignment,
            2 => Precedence::PrecOr,
            3 => Precedence::PrecAnd,
            4 => Precedence::PrecEquality,
            5 => Precedence::PrecComparison,
            6 => Precedence::PrecTerm,
            7 => Precedence::PrecFactor,
            8 => Precedence::PrecUnary,
            9 => Precedence::PrecCall,
            10 => Precedence::PrecPrimary,
            _ => panic!("Invalid precedence"),
        }
    }
}

impl From<Precedence> for usize {
    fn from(value: Precedence) -> Self {
        match value {
            Precedence::PrecNone => 0,
            Precedence::PrecAssignment => 1,
            Precedence::PrecOr => 2,
            Precedence::PrecAnd => 3,
            Precedence::PrecEquality => 4,
            Precedence::PrecComparison => 5,
            Precedence::PrecTerm => 6,
            Precedence::PrecFactor => 7,
            Precedence::PrecUnary => 8,
            Precedence::PrecCall => 9,
            Precedence::PrecPrimary => 10,
        }
    }
}

impl Precedence {
    pub fn get_next(self) -> Self {
        if self == Precedence::PrecNone {
            Precedence::PrecAssignment
            // panic!("Cannot get next precedence from PrecNone")
        } else {
            ((self as usize) + 1).into()
        }
    }
    pub fn get_previous(self) -> Self {
        if self == Precedence::PrecPrimary {
            Precedence::PrecPrimary
            // panic!("Cannot get previous precedence from PrecPrimary")
        } else {
            ((self as usize) - 1).into()
        }
    }
}
