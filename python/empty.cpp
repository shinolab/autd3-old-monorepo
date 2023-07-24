/*
 * File: empty.cpp
 * Project: python
 * Created Date: 24/07/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#include <Python.h>

static PyMethodDef dummy_methods[] = {{nullptr, nullptr, 0, nullptr}};

static PyModuleDef dummy_module = {PyModuleDef_HEAD_INIT, "empty", "Provides nothing", 0, dummy_methods};

PyMODINIT_FUNC PyInit_dummy() { return PyModule_Create(&dummy_module); }
