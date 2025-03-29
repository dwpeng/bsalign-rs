///! Score parameters for alignment
///! M: match score
///! X: mismatch score
///! O: gap open score
///! E: gap extension score
///! Q: gap open score for second score function
///! P: gap extension score for second score function
#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy)]
pub struct AlignScore {
    pub M: i32,
    pub X: i32,
    pub O: i32,
    pub E: i32,
    pub Q: i32,
    pub P: i32,
}

impl AlignScore {
    pub fn poa_default() -> Self {
        AlignScore {
            M: 2,
            X: -6,
            O: -3,
            E: -2,
            Q: -8,
            P: -1,
        }
    }
    pub fn pairwise_default() -> Self {
        AlignScore {
            M: 2,
            X: -6,
            O: -3,
            E: -2,
            Q: 0,
            P: 0,
        }
    }
}
