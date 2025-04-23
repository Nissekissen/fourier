import unittest
import fft

class TestFFTModule(unittest.TestCase):

    def test_fft(self):
        # Test the FFT function with a simple input
        input_data = [0, 1, 2, 3]
        expected_output = [6.0, -2.0, 0.0, 0.0]  # Replace with the expected output
        result = fft.calculate_fft(input_data)
        self.assertEqual(result, expected_output)
    