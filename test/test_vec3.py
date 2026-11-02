import numpy as np
import pyglam

class TestDVec3:
    def test_construct(self):
        actual = pyglam.DVec3(100.)
        np.testing.assert_allclose(actual.x, 100.)
        np.testing.assert_allclose(actual.y, 100.)
        np.testing.assert_allclose(actual.z, 100.)

    def test_np_consume(self):
        arr = np.array([pyglam.DVec3(100), pyglam.DVec3(100), pyglam.DVec3(100)])
        assert arr.shape == (3, 3)
        np.testing.assert_allclose(arr, 100.)