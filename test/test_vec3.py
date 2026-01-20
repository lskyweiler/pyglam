import numpy as np
import pyglam

class TestDVec3:
    def test_construct(self):
        actual = pyglam.DVec3(100.)
        np.testing.assert_allclose(actual.x, 100.)
        np.testing.assert_allclose(actual.y, 100.)
        np.testing.assert_allclose(actual.z, 100.)