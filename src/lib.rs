mod vec3;
use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;

/// Simple linear algebra vector library
#[pymodule]
mod pyglam {
    #[pymodule_export]
    use crate::vec3::DVec3;
    #[pymodule_export]
    use crate::vec3::Vec3;
    // #[pymodule_export]
    // use crate::vec3::IVec3;
}

define_stub_info_gatherer!(stub_info);
