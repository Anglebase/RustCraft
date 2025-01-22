use rustcraft::{app::*, debug, log::*, model::*, shader::*, texture::*, utils::Mat4};

fn render_init() {
    debug!("render::init()", "正在载入着色器...");
    SHADER_MANAGER.load_from("shader/");
    debug!("render::init()", "正在载入纹理...");
    TEXTURE_MANAGER.load_from("texture/");

    let vertices = vec![
        0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, -0.5,
        -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -0.5, 0.5, 0.0, 1.0, 1.0, 1.0, 0.0, 1.0,
    ];
    let indices = vec![0, 1, 2, 2, 3, 0];
    MODEL_MANAGER.add_model("Face", vertices, indices, "3f;3f;2f");
}

fn render_loop() {
    unsafe {
        gl::ClearColor(0.3, 0.5, 0.4, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        let shader = SHADER_MANAGER.get("test").unwrap();
        shader.use_program();
        let mut mat4 = Mat4::<f32>::new();
        mat4[0][3] = 0.25;
        mat4[1][3] = 0.25;
        shader.set_uniform("trans", mat4);
        TEXTURE_MANAGER.bind("container2", 0);
        MODEL_MANAGER.draw_model("Face");
    }
}

fn main() {
    Log::set_level(Level::Debug);
    App::set_render_init_callback(render_init);
    App::set_render_loop_callback(render_loop);
    let mut app = App::new(800, 600, "RustCraft");
    app.exec();
}
