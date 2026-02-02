use glam;
use pyo3::{
    exceptions::{PyNotImplementedError, PyValueError},
    prelude::*,
};
use pyo3_stub_gen::derive::*;
use std::ops::{Add, Deref, DerefMut, Div, Mul, Sub};

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
enum Vec3VecOpsEnum {
    DVec3(DVec3),
    #[cfg(feature = "f32")]
    Vec3(Vec3),
}

macro_rules! vec3_glam_wrapper {
    ($py_class_name: ident, $glam_class_name: ty, $var_type: ty) => {
        /// 3 Component vector xyz
        #[repr(transparent)]
        #[gen_stub_pyclass]
        #[pyclass]
        #[derive(Clone, Copy)]
        pub struct $py_class_name(pub(crate) $glam_class_name);

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

            /// Convert this vector to a 3 component tuple
            /// 
            /// # Returns
            /// 
            /// - `(float, float, float)` - XYZ tuple
            /// 
            fn to_tuple(&self) -> ($var_type, $var_type, $var_type) {
                (self.x, self.y, self.z)
            }

            pub fn __add__(&mut self, rhs: Bound<'_, PyAny>) -> PyResult<$py_class_name> {
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
            pub fn __radd__(&mut self, lhs: Bound<'_, PyAny>) -> PyResult<$py_class_name> {
                // lhs + this
                return self.__add__(lhs);
            }
            pub fn __sub__(&mut self, rhs: Bound<'_, PyAny>) -> PyResult<$py_class_name> {
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
            pub fn __rsub__(&mut self, lhs: Bound<'_, PyAny>) -> PyResult<$py_class_name> {
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
            pub fn __mul__(&mut self, rhs: Bound<'_, PyAny>) -> PyResult<$py_class_name> {
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
            pub fn __rmul__(&mut self, lhs: Bound<'_, PyAny>) -> PyResult<$py_class_name> {
                // lhs * this
                return self.__mul__(lhs);
            }
            pub fn __truediv__(&mut self, rhs: Bound<'_, PyAny>) -> PyResult<$py_class_name> {
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
            pub fn __rtruediv__(&mut self, lhs: Bound<'_, PyAny>) -> PyResult<$py_class_name> {
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

            pub fn __iadd__(&mut self, rhs: Bound<'_, PyAny>) -> PyResult<()> {
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
            pub fn __isub__(&mut self, rhs: Bound<'_, PyAny>) -> PyResult<()> {
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
            pub fn __imul__(&mut self, rhs: Bound<'_, PyAny>) -> PyResult<()> {
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
            pub fn __itruediv__(&mut self, rhs: Bound<'_, PyAny>) -> PyResult<()> {
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
            pub fn __neg__(&mut self) -> $py_class_name {
                // -this
                return $py_class_name::new(-self.0);
            }

            pub fn normalize(&self) -> $py_class_name {
                return $py_class_name::new(self.0.normalize());
            }
            pub fn length(&self) -> $var_type {
                return self.0.length();
            }
            pub fn dot(&self, rhs: Bound<'_, PyAny>) -> PyResult<$var_type> {
                match rhs.extract::<Vec3VecOpsEnum>() {
                    Ok(Vec3VecOpsEnum::DVec3(vec)) => {
                        return Ok(self.0.dot(<$glam_class_name>::new(vec.x as $var_type, vec.y as $var_type, vec.z as $var_type)));
                    }
                    #[cfg(feature = "f32")]
                    Ok(Vec3VecOpsEnum::Vec3(vec)) => {
                        return Ok(self.0.dot(<$glam_class_name>::new(vec.x as $var_type, vec.y as $var_type, vec.z as $var_type)));
                    }
                    Err(e) => {
                        return Err(PyNotImplementedError::new_err(e));
                    }
                }
            }
            pub fn cross(&self, rhs: Bound<'_, PyAny>) -> PyResult<$py_class_name> {
                match rhs.extract::<Vec3VecOpsEnum>() {
                    Ok(Vec3VecOpsEnum::DVec3(vec)) => {
                        return Ok($py_class_name::new(self.0.cross(<$glam_class_name>::new(vec.x as $var_type, vec.y as $var_type, vec.z as $var_type))));
                    }
                    #[cfg(feature = "f32")]
                    Ok(Vec3VecOpsEnum::Vec3(vec)) => {
                        return Ok($py_class_name::new(self.0.cross(<$glam_class_name>::new(vec.x as $var_type, vec.y as $var_type, vec.z as $var_type))));
                    }
                    Err(e) => {
                        return Err(PyNotImplementedError::new_err(e));
                    }
                }
            }
        }

        macro_rules! into_glam {
            ($a:ty, $b:ty) => {
                impl Into<$a> for $b {
                    fn into(self) -> $a {
                        self.0
                    }
                }
            };
        }
        into_glam!($glam_class_name, $py_class_name);
        into_glam!($glam_class_name, &$py_class_name);
        macro_rules! from_glam {
            ($a:ty, $b:ty) => {
                impl From<$a> for $b {
                    fn from(value: $a) -> Self {
                        Self(value.clone())
                    }
                }
            };
        }
        from_glam!($glam_class_name, $py_class_name);
        from_glam!(&$glam_class_name, $py_class_name);

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

        macro_rules! ops_with_self {
            ($a:ty, $b:ty) => {
                impl Add<$a> for $b {
                    type Output = $py_class_name;

                    fn add(self, rhs: $a) -> Self::Output {
                        $py_class_name(self.0 + rhs.0)
                    }
                }
                impl Sub<$a> for $b {
                    type Output = $py_class_name;

                    fn sub(self, rhs: $a) -> Self::Output {
                        $py_class_name(self.0 - rhs.0)
                    }
                }
                impl Mul<$a> for $b {
                    type Output = $py_class_name;

                    fn mul(self, rhs: $a) -> Self::Output {
                        $py_class_name(self.0 * rhs.0)
                    }
                }
                impl Div<$a> for $b {
                    type Output = $py_class_name;

                    fn div(self, rhs: $a) -> Self::Output {
                        $py_class_name(self.0 / rhs.0)
                    }
                }
            };
        }
        ops_with_self!($py_class_name, $py_class_name);
        ops_with_self!($py_class_name, &$py_class_name);
        ops_with_self!(&$py_class_name, $py_class_name);
        ops_with_self!(&$py_class_name, &$py_class_name);

        macro_rules! ops_with_glam {
            ($a:ty, $b:ty) => {
                impl Add<$a> for $b {
                    type Output = $py_class_name;

                    fn add(self, rhs: $a) -> Self::Output {
                        $py_class_name(self.0 + rhs)
                    }
                }
                impl Sub<$a> for $b {
                    type Output = $py_class_name;

                    fn sub(self, rhs: $a) -> Self::Output {
                        $py_class_name(self.0 - rhs)
                    }
                }
                impl Mul<$a> for $b {
                    type Output = $py_class_name;

                    fn mul(self, rhs: $a) -> Self::Output {
                        $py_class_name(self.0 * rhs)
                    }
                }
                impl Div<$a> for $b {
                    type Output = $py_class_name;

                    fn div(self, rhs: $a) -> Self::Output {
                        $py_class_name(self.0 / rhs)
                    }
                }
            };
        }
        ops_with_glam!($glam_class_name, $py_class_name);
        ops_with_glam!(&$glam_class_name, $py_class_name);
        ops_with_glam!($glam_class_name, &$py_class_name);
        ops_with_glam!(&$glam_class_name, &$py_class_name);
        ops_with_glam!($var_type, $py_class_name);
        ops_with_glam!(&$var_type, $py_class_name);
        ops_with_glam!($var_type, &$py_class_name);
        ops_with_glam!(&$var_type, &$py_class_name);
    }
}
vec3_glam_wrapper!(DVec3, glam::DVec3, f64);
#[cfg(feature = "f32")]
vec3_glam_wrapper!(Vec3, glam::Vec3, f32);

/// Creates a 3-dimensional f64 vector
#[inline(always)]
#[pyfunction]
pub fn dvec3(x: f64, y: f64, z: f64) -> DVec3 {
    DVec3::new(glam::dvec3(x, y, z))
}
#[cfg(feature = "f32")]
/// Creates a 3-dimensional f32 vector
#[inline(always)]
#[pyfunction]
pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(glam::vec3(x, y, z))
}

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

        #[test]
        fn test_simple_dvec3_api() {
            let actual = dvec3(10., 10., 10.);
            assert_eq!(actual.x, 10.);
        }
        #[cfg(feature = "f32")]
        #[test]
        fn test_simple_vec3_api() {
            let actual = vec3(10., 10., 10.);
            assert_eq!(actual.x, 10.);
        }

        #[test]
        fn test_add() {
            let actual = dvec3(10., 10., 10.) + glam::dvec3(10., 10., 10.);
            assert_eq!(actual.x, 20.);
        }
        #[test]
        fn test_sub() {
            let actual = dvec3(10., 10., 10.) - glam::dvec3(10., 10., 10.);
            assert_eq!(actual.x, 0.);
        }
        #[test]
        fn test_mul() {
            let actual = dvec3(10., 10., 10.) * glam::dvec3(10., 10., 10.);
            assert_eq!(actual.x, 100.);
        }
        #[test]
        fn test_div() {
            let actual = dvec3(10., 10., 10.) / glam::dvec3(10., 10., 10.);
            assert_eq!(actual.x, 1.);
        }
    }
}
