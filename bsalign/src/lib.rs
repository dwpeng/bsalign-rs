mod align_mode;
mod align_score;
mod vector;

#[cfg(target_arch = "x86_64")]
pub mod pairwise;
#[cfg(target_arch = "x86_64")]
pub mod poa;

pub use align_mode::AlignMode;
pub use align_score::AlignScore;
