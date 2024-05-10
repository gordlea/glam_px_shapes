#[cfg(not(any(feature = "glam_027", feature = "glam_026", feature = "glam_025")))]
compile_error!("Please enable one of the following features: 'glam_027', 'glam_026', 'glam_025'");

#[cfg(feature = "glam_027")]
pub use glam_027::*;
#[cfg(feature = "glam_026")]
pub use glam_026::*;
#[cfg(feature = "glam_025")]
pub use glam_025::*;
