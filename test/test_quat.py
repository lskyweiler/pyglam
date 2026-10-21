import numpy as np
import pyglam

class TestDQuat:
    def test_construct(self):
        actual = pyglam.DQuat.identity()
        np.testing.assert_allclose(actual.x, 0)
        np.testing.assert_allclose(actual.y, 0)
        np.testing.assert_allclose(actual.z, 0)
        np.testing.assert_allclose(actual.w, 1)

    def test_np_consume(self):
        arr = np.array([pyglam.DQuat(100,100,100,100), pyglam.DQuat(100,100,100,100), pyglam.DQuat(100,100,100,100)])
        assert arr.shape == (3, 4)
        np.testing.assert_allclose(arr, 100.)