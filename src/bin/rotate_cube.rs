use glfw::*;
use mats::*;
use rustcraft::{debug, log::*, *};

pub fn key_callback(window: &mut Window, key: Key, scancode: i32, action: Action, mods: Modifiers) {
    match (key, action) {
        (Key::Escape, Action::Press) => {
            debug!("Events", "ESC 被按下，程序将退出");
            window.set_should_close(true)
        }
        _ => {}
    }
    let _ = (window, key, scancode, action, mods);
}

fn render_init() {
    debug!("render::init()", "正在载入着色器...");
    SHADER_MANAGER.load_from("shader/");
    debug!("render::init()", "正在载入纹理...");
    TEXTURE_MANAGER.load_from("texture/");

    MODEL_MANAGER.load_from_file("model/cube.json");
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }
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
    let (w, h) = App::window_size();
    let proj = perspective(radian(45.0), w as f32 / h as f32, 0.1, 100.0);

    let shader = SHADER_MANAGER.get("cube").unwrap();
    let model: Mat4<f32> = rotate3(radian(App::time() * 100.0), Vec3::from([1.0, 1.0, 0.0]));
    let model = tranlate3(Vec3::from([0.0, 0.0, -1.0])) * model;
    shader.use_program();
    shader.set_uniform("model", model);
    shader.set_uniform("view", view);
    shader.set_uniform("projection", proj);
    TEXTURE_MANAGER.bind("container2", 0);
    shader.set_uniform("texture0", 0);
    MODEL_MANAGER.draw_model("cube");
}

fn main() {
    Log::set_level(Level::Debug);

    let mut app = AppBuilder::new(1600, 900, "RustCraft")
        .set_render_init_callback(render_init)
        .set_render_loop_callback(render_loop)
        .set_key_callback(key_callback)
        .build();
    app.exec();
}
