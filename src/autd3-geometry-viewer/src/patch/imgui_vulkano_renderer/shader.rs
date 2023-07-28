pub mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/patch/imgui_vulkano_renderer/shaders/shader.vert",
    }
}

pub mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/patch/imgui_vulkano_renderer/shaders/shader.frag",
    }
}
