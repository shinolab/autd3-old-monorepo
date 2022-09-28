// File: window_handler.hpp
// Project: include
// Created Date: 23/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 28/09/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>

#include <utility>

namespace autd3::extra::geometry_viewer {

class WindowHandler {
 public:
  WindowHandler(const int32_t width, const int32_t height) noexcept : _width(width), _height(height) {}
  ~WindowHandler() {
    glfwDestroyWindow(_window);
    glfwTerminate();
  }
  WindowHandler(const WindowHandler& v) = delete;
  WindowHandler& operator=(const WindowHandler& obj) = delete;
  WindowHandler(WindowHandler&& obj) = default;
  WindowHandler& operator=(WindowHandler&& obj) = default;

  void init(void* renderer_pointer, const GLFWframebuffersizefun framebuffer_size_callback, const GLFWwindowposfun pos_callback) {
    glfwInit();
    glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
    _window = glfwCreateWindow(_width, _height, "Geometry Viewer", nullptr, nullptr);
    glfwSetWindowUserPointer(_window, renderer_pointer);
    glfwSetFramebufferSizeCallback(_window, framebuffer_size_callback);
    glfwSetWindowPosCallback(_window, pos_callback);
  }

  [[nodiscard]] bool should_close() const { return glfwWindowShouldClose(_window); }

  static void poll_events() { glfwPollEvents(); }

  [[nodiscard]] std::pair<float, float> scale() const {
    float x_scale, y_scale;
    auto* monitor = get_current_monitor(_window);
    glfwGetMonitorContentScale(monitor, &x_scale, &y_scale);
    return {x_scale, y_scale};
  }

  GLFWmonitor* get_current_monitor(GLFWwindow* const window) const {
    int max_overlap = 0;
    GLFWmonitor* current_monitor = nullptr;
    int window_x, window_y, window_width, window_height;
    glfwGetWindowPos(window, &window_x, &window_y);
    glfwGetWindowSize(window, &window_width, &window_height);
    int num_monitors;
    auto** monitors = glfwGetMonitors(&num_monitors);
    for (int i = 0; i < num_monitors; i++) {
      const GLFWvidmode* mode = glfwGetVideoMode(monitors[i]);
      int monitor_x, monitor_y;
      glfwGetMonitorPos(monitors[i], &monitor_x, &monitor_y);
      const int monitor_width = mode->width;
      const int monitor_height = mode->height;
      if (const int overlap = std::max(0, std::min(window_x + window_width, monitor_x + monitor_width) - std::max(window_x, monitor_x)) *
                              std::max(0, std::min(window_y + window_height, monitor_y + monitor_height) - std::max(window_y, monitor_y));
          max_overlap < overlap) {
        max_overlap = overlap;
        current_monitor = monitors[i];
      }
    }
    return current_monitor;
  }

  [[nodiscard]] GLFWwindow* window() const { return _window; }

 private:
  int32_t _width;
  int32_t _height;
  GLFWwindow* _window{};
};

}  // namespace autd3::extra::geometry_viewer