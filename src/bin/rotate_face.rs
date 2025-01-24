use mats::*;
use rustcraft::{debug, log::*, *};

fn render_init() {
    debug!("render::init()", "正在载入着色器...");
    SHADER_MANAGER.load_from("shader/");
    debug!("render::init()", "正在载入纹理...");
    TEXTURE_MANAGER.load_from("texture/");

    MODEL_MANAGER.load_from_file("model/face.json");
}

fn render_loop() {
    unsafe {
        gl::ClearColor(0.3, 0.5, 0.4, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
    let view = look_at(
        Vec3::from([0.0, 0.0, 3.0]),
        Vec3::from([0.0, 0.0, 0.0]),
        Vec3::from([0.0, 1.0, 0.0]),
    );
    let proj = perspective(radian(45.0), 800.0 / 600.0, 0.1, 100.0);

    let shader = SHADER_MANAGER.get("face").unwrap();
    let model: Mat4<f32> = rotate3(radian(App::time() * 100.0), Vec3::from([0.0, 0.0, 1.0]));
    shader.use_program();
    shader.set_uniform("trans", model);
    shader.set_uniform("view", view);
    shader.set_uniform("proj", proj);
    TEXTURE_MANAGER.bind("container2", 0);
    shader.set_uniform("texture1", 0);
    MODEL_MANAGER.draw_model("face");
}

fn main() {
    Log::set_level(Level::Debug);
    let mut app = AppBuilder::new(800, 600, "RustCraft")
        .set_render_init_callback(render_init)
        .set_render_loop_callback(render_loop)
        .build();
    app.exec();
}
