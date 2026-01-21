use crate::vec3;
use either::Either;
use glam;
use pyo3::{exceptions::PyNotImplementedError, prelude::*};
use pyo3_stub_gen::derive::*;
use std::ops::{Deref, DerefMut, Mul};

/// Supported types for vector operations on other vecs where scalars don't make sense
/// example: dot, cross
#[derive(FromPyObject)]
enum QuatOpsEnum {
    DVec3(vec3::DVec3),
    Vec3(vec3::Vec3),
    DQuat(DQuat),
    Quat(Quat),
}

macro_rules! vec3_glam_wrapper {
    ($py_class_name: ident, $py_vec_class_name: ty, $glam_class_name: ty,$glam_vec_class_name: ty, $var_type: ty) => {
        /// 4 Component Quaternion wxyz
        #[repr(transparent)]
        #[gen_stub_pyclass]
        #[pyclass]
        #[derive(Clone, Copy)]
        pub struct $py_class_name($glam_class_name);

        impl $py_class_name {
            pub fn new(quat: $glam_class_name) -> Self {
                Self(quat)
            }
        }

        #[gen_stub_pymethods]
        #[pymethods]
        impl $py_class_name {
            /// Create a new quaternion from components.
            /// Usually you want `from_axis_angle` or `from_rotation_arc` instead of this
            ///
            /// # Arguments
            ///
            /// - `x` (`float`) - x component
            /// - `y` (`float`) - y component
            /// - `z` (`float`) - z component
            /// - `w` (`float`) - scalar component
            ///
            /// # Returns
            ///
            /// - `PyResult<Self>` - Describe the return value.
            ///
            #[new]
            pub fn py_new(x: $var_type, y: $var_type, z: $var_type, w: $var_type) -> Self {
                let inner = <$glam_class_name>::from_xyzw(x, y, z, w);
                $py_class_name(inner)
            }

            /// Create a new quaternion from an axis and angle
            ///
            /// # Arguments
            ///
            /// - `axis` (`vec3`) - Axis, should be normalized
            /// - `angle` (`float`) - Angle in radians
            ///
            /// # Returns
            ///
            /// - `PyResult<Self>` - Quaternion describing this axis/angle rotation
            ///
            #[staticmethod]
            pub fn from_axis_angle(axis: &$py_vec_class_name, angle: $var_type) -> Self {
                let inner = <$glam_class_name>::from_axis_angle(axis.into(), angle);
                $py_class_name(inner)
            }

            /// Gets the minimal rotation for transforming `from` to `to`.  The rotation is in the
            /// plane spanned by the two vectors.  Will rotate at most 180 degrees.
            ///
            /// `from_rotation_arc(from, to) * from ≈ to`.
            ///
            /// For near-singular cases (from≈to and from≈-to) the current implementation
            /// is only accurate to about 0.001 (for `f32`).
            ///
            /// # Arguments
            ///
            /// - `from_` (`vec3`) - starting vector. Must be a unit vector
            /// - `to` (`vec3`) - ending vector. Must be a unit vector
            ///
            /// # Returns
            ///
            /// - `PyResult<Self>` - Returns a quaternion that would rotate vector from onto to
            ///
            #[staticmethod]
            pub fn from_rotation_arc(from_: &$py_vec_class_name, to: &$py_vec_class_name) -> Self {
                let inner = <$glam_class_name>::from_rotation_arc(from_.into(), to.into());
                $py_class_name(inner)
            }

            /// Multiply this quaternion with either another quaternion or a vector
            ///
            /// A quaternion multiplication means combining two rotations into a single rotation
            /// A vector multiplication yields a rotated vec3
            ///
            /// # Arguments
            ///
            /// - `rhs` (`Bound<'_, PyAny>`) - Quat or Vec to multiply
            ///
            /// # Returns
            ///
            /// - `PyResult<Either<, >>` - Either a new rotation or a rotated vector
            ///
            fn __mul__(
                &mut self,
                rhs: Bound<'_, PyAny>,
            ) -> PyResult<Either<$py_class_name, $py_vec_class_name>> {
                // this * rhs
                let this = self.0;
                match rhs.extract::<QuatOpsEnum>() {
                    Ok(QuatOpsEnum::DQuat(dquat)) => {
                        return Ok(Either::Left($py_class_name::new(
                            this * <$glam_class_name>::from_xyzw(
                                dquat.x as $var_type,
                                dquat.y as $var_type,
                                dquat.z as $var_type,
                                dquat.w as $var_type,
                            ),
                        )));
                    }
                    Ok(QuatOpsEnum::Quat(quat)) => {
                        return Ok(Either::Left($py_class_name::new(
                            this * <$glam_class_name>::from_xyzw(
                                quat.x as $var_type,
                                quat.y as $var_type,
                                quat.z as $var_type,
                                quat.w as $var_type,
                            ),
                        )));
                    }
                    Ok(QuatOpsEnum::DVec3(vec)) => {
                        return Ok(Either::Right(<$py_vec_class_name>::new(
                            this * <$glam_vec_class_name>::new(
                                vec.x as $var_type,
                                vec.y as $var_type,
                                vec.z as $var_type,
                            ),
                        )));
                    }
                    Ok(QuatOpsEnum::Vec3(vec)) => {
                        return Ok(Either::Right(<$py_vec_class_name>::new(
                            this * <$glam_vec_class_name>::new(
                                vec.x as $var_type,
                                vec.y as $var_type,
                                vec.z as $var_type,
                            ),
                        )));
                    }
                    Err(e) => {
                        return Err(PyNotImplementedError::new_err(e));
                    }
                }
            }
            /// Multiply this quaternion with either another quaternion or a vec3
            /// 
            /// Order doesn't matter for multiplying a vector and a quat, but order matters for multiplying two quats
            /// 
            /// # Arguments
            /// 
            /// - `lhs` (`Bound<'_, PyAny>`) - left hand side multiplicand
            /// 
            /// # Returns
            /// 
            /// - `PyResult<Either<, >>` - Either a quat equivalent to the combined rotation or a rotated vector
            /// 
            fn __rmul__(
                &mut self,
                lhs: Bound<'_, PyAny>,
            ) -> PyResult<Either<$py_class_name, $py_vec_class_name>> {
                // lhs * this
                return self.__mul__(lhs);
            }

            /// Normalize this quaternion into a unit quat
            fn normalize(&self) -> $py_class_name {
                $py_class_name::new(self.0.normalize())
            }
            /// Compute the conjugate of this quat.
            /// If this is a unit quat, the conjugate is equal to the inverse of the rotation
            fn conjugate(&self) -> $py_class_name {
                $py_class_name::new(self.0.conjugate())
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
                impl Mul<$a> for $b {
                    type Output = $py_class_name;

                    fn mul(self, rhs: $a) -> Self::Output {
                        $py_class_name(self.0 * rhs.0)
                    }
                }
            };
        }
        ops_with_self!($py_class_name, $py_class_name);
        ops_with_self!($py_class_name, &$py_class_name);
        ops_with_self!(&$py_class_name, $py_class_name);
        ops_with_self!(&$py_class_name, &$py_class_name);

        macro_rules! vec3_ops_with_self {
            ($a:ty, $b:ty) => {
                impl Mul<$a> for $b {
                    type Output = $py_vec_class_name;

                    fn mul(self, rhs: $a) -> Self::Output {
                        <$py_vec_class_name>::new(self.0 * rhs.0)
                    }
                }
            };
        }
        vec3_ops_with_self!($py_vec_class_name, $py_class_name);
        vec3_ops_with_self!($py_vec_class_name, &$py_class_name);
        vec3_ops_with_self!(&$py_vec_class_name, $py_class_name);
        vec3_ops_with_self!(&$py_vec_class_name, &$py_class_name);

        macro_rules! ops_with_glam {
            ($a:ty, $b:ty) => {
                impl Mul<$a> for $b {
                    type Output = $py_class_name;

                    fn mul(self, rhs: $a) -> Self::Output {
                        $py_class_name(self.0 * rhs)
                    }
                }
            };
        }
        ops_with_glam!($glam_class_name, $py_class_name);
        ops_with_glam!(&$glam_class_name, $py_class_name);
        ops_with_glam!($glam_class_name, &$py_class_name);
        ops_with_glam!(&$glam_class_name, &$py_class_name);
        macro_rules! vec3_ops_with_glam {
            ($a:ty, $b:ty) => {
                impl Mul<$a> for $b {
                    type Output = $py_vec_class_name;

                    fn mul(self, rhs: $a) -> Self::Output {
                        <$py_vec_class_name>::new(self.0 * rhs)
                    }
                }
            };
        }
        vec3_ops_with_glam!($glam_vec_class_name, $py_class_name);
        vec3_ops_with_glam!(&$glam_vec_class_name, $py_class_name);
        vec3_ops_with_glam!($glam_vec_class_name, &$py_class_name);
        vec3_ops_with_glam!(&$glam_vec_class_name, &$py_class_name);
    };
}
vec3_glam_wrapper!(DQuat, vec3::DVec3, glam::DQuat, glam::DVec3, f64);
vec3_glam_wrapper!(Quat, vec3::Vec3, glam::Quat, glam::Vec3, f32);

/// Creates a 4-dimensional f64 quaternion
#[inline(always)]
#[pyfunction]
pub fn dquat(x: f64, y: f64, z: f64, w: f64) -> DQuat {
    DQuat::new(glam::dquat(x, y, z, w))
}
/// Creates a 4-dimensional f32 quaternion
#[inline(always)]
#[pyfunction]
pub fn quat(x: f32, y: f32, z: f32, w: f32) -> Quat {
    Quat::new(glam::quat(x, y, z, w))
}
