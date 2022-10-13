add_library(imgui_vulkan ${IMGUI_PATH}/imgui.cpp ${IMGUI_PATH}/imgui_draw.cpp ${IMGUI_PATH}/imgui_demo.cpp ${IMGUI_PATH}/imgui_widgets.cpp ${IMGUI_PATH}/imgui_tables.cpp ${IMGUI_PATH}/backends/imgui_impl_glfw.cpp ${IMGUI_PATH}/backends/imgui_impl_vulkan.cpp)
target_include_directories(imgui_vulkan PUBLIC ${IMGUI_PATH} ${IMGUI_PATH}/backends)
target_include_directories(imgui_vulkan PRIVATE ${Vulkan_INCLUDE_DIRS})
target_link_libraries(imgui_vulkan PRIVATE glfw)

set_target_properties(imgui_vulkan PROPERTIES FOLDER "3rdparty")
