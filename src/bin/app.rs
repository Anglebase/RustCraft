use rustcraft::{app::*, log::*, model::*, shader::*, texture::*, debug};

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

        SHADER_MANAGER.get("test").unwrap().use_program();
        TEXTURE_MANAGER.bind("container2", 0);
        MODEL_MANAGER.draw_model("Face");
    }
}

fn main() {
    set_level(Level::Debug);
    App::set_render_init_callback(render_init);
    App::set_render_loop_callback(render_loop);
    let mut app = App::new(800, 600, "RustCraft");
    app.exec();
}
