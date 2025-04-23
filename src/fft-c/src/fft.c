#define PY_SSIZE_T_CLEAN
#include <Python.h>
#include <math.h>
#include <complex.h>

static complex double dft_X_k(complex double *x, int k, int N) {
    complex double sum = 0.0;
    for (int n = 0; n < N; n++) {
        double theta = -2.0 * Py_MATH_PI * k * n / N;
        sum += x[n] * cexp(I * theta);
    }

    return sum;
}

PyObject* convert_c_complex_to_python(complex double c) {
    double real_part = creal(c);
    double imag_part = cimag(c);

    PyObject* py_complex = PyComplex_FromDoubles(real_part, imag_part);

    if (py_complex == NULL) {
        return NULL;
    }

    return py_complex;
}

static PyObject *fft_calculate_fft(PyObject *self, PyObject *args)
{
    PyObject *input_list;
    Py_ssize_t n;
    PyObject *output_list;

    if (!PyArg_ParseTuple(args, "O!", &PyList_Type, &input_list)) {
        return NULL;
    }

    n = PyList_Size(input_list);
    output_list = PyList_New(n);
    if (!output_list) {
        return NULL;
    };

    // Loop through input list
    for (Py_ssize_t k = 0; k < n; k++) {
        complex double *x = malloc(n * sizeof(complex double));
        if (!x) {
            Py_DECREF(output_list);
            return NULL;
        };

        // Fill the array with input values
        for (Py_ssize_t j = 0; j < n; j++) {
            PyObject *item = PyList_GetItem(input_list, j);
            if (!PyComplex_Check(item)) {
                free(x);
                Py_DECREF(output_list);
                return NULL;
            }
            x[j] = PyComplex_RealAsDouble(item) + I * PyComplex_ImagAsDouble(item);
        }

        // Calculate DFT
        complex double X_k = dft_X_k(x, k, n);
        free(x);

        // Convert to Python complex object
        PyObject* py_complex = convert_c_complex_to_python(X_k); 

        if (py_complex == NULL) {
            Py_DECREF(output_list);
            return NULL;
        }

        // Set the output list item
        if (PyList_SetItem(output_list, k, py_complex) < 0) {
            Py_DECREF(output_list);
            return NULL;
        }
    }

    // Return the output list
    if (PyList_Check(output_list)) {
        return output_list;
    } else {
        Py_DECREF(output_list);
        return NULL;
    }
}

static PyMethodDef FftMethods[] = {
    {"calculate_fft", fft_calculate_fft, METH_VARARGS,
     "Calculates FFT from sample points"},
    {NULL, NULL, 0, NULL} /* Sentinel */
};

static struct PyModuleDef fft_module = {
    PyModuleDef_HEAD_INIT,
    "fft",
    NULL,
    -1,
    FftMethods
};

PyMODINIT_FUNC PyInit_fft_module(void) {
    return PyModule_Create(&fft_module);
}