pub mod quat;
pub mod vec3;

use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;

pub use self::quat::*;
pub use self::vec3::*;

/// Simple linear algebra vector library
#[pymodule]
mod pyglam {
    #[pymodule_export]
    use crate::quat::DQuat;
    #[pymodule_export]
    use crate::vec3::DVec3;
    #[pymodule_export]
    use crate::quat::dquat;
    #[pymodule_export]
    use crate::vec3::dvec3;

    #[cfg(feature = "f32")]
    #[pymodule_export]
    use crate::vec3::Vec3;
    #[cfg(feature = "f32")]
    #[pymodule_export]
    use crate::vec3::vec3;

    #[cfg(feature = "f32")]
    #[pymodule_export]
    use crate::quat::Quat;
    #[cfg(feature = "f32")]
    #[pymodule_export]
    use crate::quat::quat;
}

define_stub_info_gatherer!(stub_info);
