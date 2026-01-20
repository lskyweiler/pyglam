use glam;
use pyo3::{
    exceptions::{PyNotImplementedError, PyValueError},
    prelude::*,
};
use pyo3_stub_gen::derive::*;
use std::ops::{Deref, DerefMut};

/// Supported types for arithmetic operations on vecs
/// vec3 * Some
#[derive(FromPyObject)]
enum Vec3ScaleOpsEnum {
    Float(f64),
    Int(i64),
    DVec3(DVec3),
    Vec3(DVec3),
    // IVec3(DVec3),
}

/// Supported types for vector operations on other vecs where scalars don't make sense
/// example: dot, cross
#[derive(FromPyObject)]
pub enum Vec3VecOpsEnum {
    DVec3(DVec3),
    Vec3(Vec3),
}

macro_rules! vec3_glam_wrapper {
    ($py_class_name: ident, $glam_class_name: ty, $var_type: ty) => {
        /// 3 Component vector xyz
        #[gen_stub_pyclass]
        #[pyclass]
        #[derive(Clone)]
        pub struct $py_class_name($glam_class_name);

        impl $py_class_name {
            pub fn new(vec: $glam_class_name) -> Self {
                Self(vec)
            }
        }

        #[gen_stub_pymethods]
        #[pymethods]
        impl $py_class_name {
            #[new]
            #[pyo3(signature = (x, y=None, z=None))]
            pub fn py_new(x: $var_type, y: Option<$var_type>, z: Option<$var_type>) -> PyResult<Self> {
                if y.is_none() ^ z.is_none() {
                    return Err(PyValueError::new_err(
                        "Either set all values dvec3(1.0, 1.0, 1.0) or only first value to set xyz to same value: dvec3(1.0)",
                    ));
                }

                let inner = <$glam_class_name>::new(x, y.unwrap_or(x), z.unwrap_or(x));
                Ok($py_class_name(inner))
            }

            #[getter]
            fn get_x(&self) -> $var_type {
                return self.0.x;
            }
            #[getter]
            fn get_y(&self) -> $var_type {
                return self.0.y;
            }
            #[getter]
            fn get_z(&self) -> $var_type {
                return self.0.z;
            }
            #[setter]
            fn set_x(&mut self, x: $var_type) {
                self.0.x = x;
            }
            #[setter]
            fn set_y(&mut self, y: $var_type) {
                self.0.y = y;
            }
            #[setter]
            fn set_z(&mut self, z: $var_type) {
                self.0.z = z;
            }

            fn __add__(&mut self, rhs: Bound<'_, PyAny>) -> PyResult<$py_class_name> {
                // this + rhs
                let this = self.0;
                match rhs.extract::<Vec3ScaleOpsEnum>() {
                    Ok(Vec3ScaleOpsEnum::Float(scalar)) => {
                        return Ok($py_class_name::new(this + scalar as $var_type));
                    }
                    Ok(Vec3ScaleOpsEnum::Int(i)) => {
                        return Ok($py_class_name::new(this + i as $var_type));
                    }
                    Ok(Vec3ScaleOpsEnum::DVec3(vec)) => {
                        return Ok($py_class_name::new(this + <$glam_class_name>::new(vec.x as $var_type, vec.y as $var_type, vec.z as $var_type)));
                    }
                    Ok(Vec3ScaleOpsEnum::Vec3(vec)) => {
                        return Ok($py_class_name::new(this + <$glam_class_name>::new(vec.x as $var_type, vec.y as $var_type, vec.z as $var_type)));
                    }
                    Err(e) => {
                        return Err(PyNotImplementedError::new_err(e));
                    }
                }
            }
            fn __radd__(&mut self, lhs: Bound<'_, PyAny>) -> PyResult<$py_class_name> {
                // lhs + this
                return self.__add__(lhs);
            }
            fn __sub__(&mut self, rhs: Bound<'_, PyAny>) -> PyResult<$py_class_name> {
                // this - rhs
                let this = self.0;
                match rhs.extract::<Vec3ScaleOpsEnum>() {
                    Ok(Vec3ScaleOpsEnum::Float(scalar)) => {
                        return Ok($py_class_name::new(this - scalar as $var_type));
                    }
                    Ok(Vec3ScaleOpsEnum::Int(i)) => {
                        return Ok($py_class_name::new(this - i as $var_type));
                    }
                    Ok(Vec3ScaleOpsEnum::DVec3(vec)) => {
                        return Ok($py_class_name::new(this - <$glam_class_name>::new(vec.x as $var_type, vec.y as $var_type, vec.z as $var_type)));
                    }
                    Ok(Vec3ScaleOpsEnum::Vec3(vec)) => {
                        return Ok($py_class_name::new(this - <$glam_class_name>::new(vec.x as $var_type, vec.y as $var_type, vec.z as $var_type)));
                    }
                    Err(e) => {
                        return Err(PyNotImplementedError::new_err(e));
                    }
                }
            }
            fn __rsub__(&mut self, lhs: Bound<'_, PyAny>) -> PyResult<$py_class_name> {
                // lhs - this
                let this = self.0;
                match lhs.extract::<Vec3ScaleOpsEnum>() {
                    Ok(Vec3ScaleOpsEnum::Float(scalar)) => {
                        return Ok($py_class_name::new(scalar as $var_type - this));
                    }
                    Ok(Vec3ScaleOpsEnum::Int(i)) => {
                        return Ok($py_class_name::new(i as $var_type - this));
                    }
                    Ok(Vec3ScaleOpsEnum::DVec3(vec)) => {
                        return Ok($py_class_name::new(<$glam_class_name>::new(vec.x as $var_type, vec.y as $var_type, vec.z as $var_type) - this));
                    }
                    Ok(Vec3ScaleOpsEnum::Vec3(vec)) => {
                        return Ok($py_class_name::new(<$glam_class_name>::new(vec.x as $var_type, vec.y as $var_type, vec.z as $var_type) - this));
                    }
                    Err(e) => {
                        return Err(PyNotImplementedError::new_err(e));
                    }
                }
            }
            fn __mul__(&mut self, rhs: Bound<'_, PyAny>) -> PyResult<$py_class_name> {
                // this * rhs
                let this = self.0;
                match rhs.extract::<Vec3ScaleOpsEnum>() {
                    Ok(Vec3ScaleOpsEnum::Float(scalar)) => {
                        return Ok($py_class_name::new(this * scalar as $var_type));
                    }
                    Ok(Vec3ScaleOpsEnum::Int(i)) => {
                        return Ok($py_class_name::new(this * i as $var_type));
                    }
                    Ok(Vec3ScaleOpsEnum::DVec3(vec)) => {
                        return Ok($py_class_name::new(this * <$glam_class_name>::new(vec.x as $var_type, vec.y as $var_type, vec.z as $var_type)));
                    }
                    Ok(Vec3ScaleOpsEnum::Vec3(vec)) => {
                        return Ok($py_class_name::new(this * <$glam_class_name>::new(vec.x as $var_type, vec.y as $var_type, vec.z as $var_type)));
                    }
                    Err(e) => {
                        return Err(PyNotImplementedError::new_err(e));
                    }
                }
            }
            fn __rmul__(&mut self, lhs: Bound<'_, PyAny>) -> PyResult<$py_class_name> {
                // lhs * this
                return self.__mul__(lhs);
            }
            fn __truediv__(&mut self, rhs: Bound<'_, PyAny>) -> PyResult<$py_class_name> {
                // this / rhs
                let this = self.0;
                match rhs.extract::<Vec3ScaleOpsEnum>() {
                    Ok(Vec3ScaleOpsEnum::Float(scalar)) => {
                        return Ok($py_class_name::new(this / scalar as $var_type));
                    }
                    Ok(Vec3ScaleOpsEnum::Int(i)) => {
                        return Ok($py_class_name::new(this / i as f64 as $var_type));
                    }
                    Ok(Vec3ScaleOpsEnum::DVec3(vec)) => {
                        return Ok($py_class_name::new(this / <$glam_class_name>::new(vec.x as $var_type, vec.y as $var_type, vec.z as $var_type)));
                    }
                    Ok(Vec3ScaleOpsEnum::Vec3(vec)) => {
                        return Ok($py_class_name::new(this / <$glam_class_name>::new(vec.x as $var_type, vec.y as $var_type, vec.z as $var_type)));
                    }
                    Err(e) => {
                        return Err(PyNotImplementedError::new_err(e));
                    }
                }
            }
            fn __rtruediv__(&mut self, lhs: Bound<'_, PyAny>) -> PyResult<$py_class_name> {
                // lhs / this
                let this = self.0;
                match lhs.extract::<Vec3ScaleOpsEnum>() {
                    Ok(Vec3ScaleOpsEnum::Float(scalar)) => {
                        return Ok($py_class_name::new(scalar as $var_type / this));
                    }
                    Ok(Vec3ScaleOpsEnum::Int(i)) => {
                        return Ok($py_class_name::new(i as $var_type / this));
                    }
                    Ok(Vec3ScaleOpsEnum::DVec3(vec)) => {
                        return Ok($py_class_name::new(<$glam_class_name>::new(vec.x as $var_type, vec.y as $var_type, vec.z as $var_type) / this));
                    }
                    Ok(Vec3ScaleOpsEnum::Vec3(vec)) => {
                        return Ok($py_class_name::new(<$glam_class_name>::new(vec.x as $var_type, vec.y as $var_type, vec.z as $var_type) / this));
                    }
                    Err(e) => {
                        return Err(PyNotImplementedError::new_err(e));
                    }
                }
            }

            fn __iadd__(&mut self, rhs: Bound<'_, PyAny>) -> PyResult<()> {
                // this += rhs
                match rhs.extract::<Vec3ScaleOpsEnum>() {
                    Ok(Vec3ScaleOpsEnum::Float(scalar)) => {
                        self.0 += scalar as $var_type;
                        return Ok(());
                    }
                    Ok(Vec3ScaleOpsEnum::Int(i)) => {
                        self.0 += i as $var_type;
                        return Ok(());
                    }
                    Ok(Vec3ScaleOpsEnum::DVec3(vec)) => {
                        self.0 += <$glam_class_name>::new(vec.x as $var_type, vec.y as $var_type, vec.z as $var_type);
                        return Ok(());
                    }
                    Ok(Vec3ScaleOpsEnum::Vec3(vec)) => {
                        self.0 += <$glam_class_name>::new(vec.x as $var_type, vec.y as $var_type, vec.z as $var_type);
                        return Ok(());
                    }
                    Err(e) => {
                        return Err(PyNotImplementedError::new_err(e));
                    }
                }
            }
            fn __isub__(&mut self, rhs: Bound<'_, PyAny>) -> PyResult<()> {
                // this -= rhs
                match rhs.extract::<Vec3ScaleOpsEnum>() {
                    Ok(Vec3ScaleOpsEnum::Float(scalar)) => {
                        self.0 -= scalar as $var_type;
                        return Ok(());
                    }
                    Ok(Vec3ScaleOpsEnum::Int(i)) => {
                        self.0 -= i as $var_type;
                        return Ok(());
                    }
                    Ok(Vec3ScaleOpsEnum::DVec3(vec)) => {
                        self.0 -= <$glam_class_name>::new(vec.x as $var_type, vec.y as $var_type, vec.z as $var_type);
                        return Ok(());
                    }
                    Ok(Vec3ScaleOpsEnum::Vec3(vec)) => {
                        self.0 -= <$glam_class_name>::new(vec.x as $var_type, vec.y as $var_type, vec.z as $var_type);
                        return Ok(());
                    }
                    Err(e) => {
                        return Err(PyNotImplementedError::new_err(e));
                    }
                }
            }
            fn __imul__(&mut self, rhs: Bound<'_, PyAny>) -> PyResult<()> {
                // this *= rhs
                match rhs.extract::<Vec3ScaleOpsEnum>() {
                    Ok(Vec3ScaleOpsEnum::Float(scalar)) => {
                        self.0 *= scalar as $var_type;
                        return Ok(());
                    }
                    Ok(Vec3ScaleOpsEnum::Int(i)) => {
                        self.0 *= i as $var_type;
                        return Ok(());
                    }
                    Ok(Vec3ScaleOpsEnum::DVec3(vec)) => {
                        self.0 *= <$glam_class_name>::new(vec.x as $var_type, vec.y as $var_type, vec.z as $var_type);
                        return Ok(());
                    }
                    Ok(Vec3ScaleOpsEnum::Vec3(vec)) => {
                        self.0 *= <$glam_class_name>::new(vec.x as $var_type, vec.y as $var_type, vec.z as $var_type);
                        return Ok(());
                    }
                    Err(e) => {
                        return Err(PyNotImplementedError::new_err(e));
                    }
                }
            }
            fn __itruediv__(&mut self, rhs: Bound<'_, PyAny>) -> PyResult<()> {
                // this *= rhs
                match rhs.extract::<Vec3ScaleOpsEnum>() {
                    Ok(Vec3ScaleOpsEnum::Float(scalar)) => {
                        self.0 /= scalar as $var_type;
                        return Ok(());
                    }
                    Ok(Vec3ScaleOpsEnum::Int(i)) => {
                        self.0 /= i as $var_type;
                        return Ok(());
                    }
                    Ok(Vec3ScaleOpsEnum::DVec3(vec)) => {
                        self.0 /= <$glam_class_name>::new(vec.x as $var_type, vec.y as $var_type, vec.z as $var_type);
                        return Ok(());
                    }
                    Ok(Vec3ScaleOpsEnum::Vec3(vec)) => {
                        self.0 /= <$glam_class_name>::new(vec.x as $var_type, vec.y as $var_type, vec.z as $var_type);
                        return Ok(());
                    }
                    Err(e) => {
                        return Err(PyNotImplementedError::new_err(e));
                    }
                }
            }
            fn __neg__(&mut self) -> $py_class_name {
                // -this
                return $py_class_name::new(-self.0);
            }

            fn normalize(&self) -> $py_class_name {
                return $py_class_name::new(self.0.normalize());
            }
            fn length(&self) -> $var_type {
                return self.0.length();
            }
            fn dot(&self, rhs: Bound<'_, PyAny>) -> PyResult<$var_type> {
                match rhs.extract::<Vec3VecOpsEnum>() {
                    Ok(Vec3VecOpsEnum::DVec3(vec)) => {
                        return Ok(self.0.dot(<$glam_class_name>::new(vec.x as $var_type, vec.y as $var_type, vec.z as $var_type)));
                    }
                    Ok(Vec3VecOpsEnum::Vec3(vec)) => {
                        return Ok(self.0.dot(<$glam_class_name>::new(vec.x as $var_type, vec.y as $var_type, vec.z as $var_type)));
                    }
                    Err(e) => {
                        return Err(PyNotImplementedError::new_err(e));
                    }
                }
            }
            fn cross(&self, rhs: Bound<'_, PyAny>) -> PyResult<$py_class_name> {
                match rhs.extract::<Vec3VecOpsEnum>() {
                    Ok(Vec3VecOpsEnum::DVec3(vec)) => {
                        return Ok($py_class_name::new(self.0.cross(<$glam_class_name>::new(vec.x as $var_type, vec.y as $var_type, vec.z as $var_type))));
                    }
                    Ok(Vec3VecOpsEnum::Vec3(vec)) => {
                        return Ok($py_class_name::new(self.0.cross(<$glam_class_name>::new(vec.x as $var_type, vec.y as $var_type, vec.z as $var_type))));
                    }
                    Err(e) => {
                        return Err(PyNotImplementedError::new_err(e));
                    }
                }
            }
        }

        impl Into<$glam_class_name> for $py_class_name {
            fn into(self) -> $glam_class_name {
                self.0
            }
        }
        impl Deref for $py_class_name {
            type Target = $glam_class_name;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl DerefMut for $py_class_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    }
}
vec3_glam_wrapper!(DVec3, glam::DVec3, f64);
vec3_glam_wrapper!(Vec3, glam::Vec3, f32);

#[cfg(test)]
mod test_vec3 {
    use super::*;

    mod test_dvec3 {
        use super::*;

        #[test]
        fn test_deref() {
            let actual = DVec3(glam::DVec3::splat(0.));
            assert_eq!(actual.x, 0.);
        }

        #[test]
        fn test_into() {
            let dvec3 = DVec3(glam::DVec3::splat(0.));

            let actual: glam::DVec3 = dvec3.into();
            assert_eq!(actual.x, 0.);
        }
    }
}
