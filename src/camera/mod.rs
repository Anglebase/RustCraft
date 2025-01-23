use crate::utils::Mat4;

pub trait Camera {
    fn view_matrix(&self) -> Mat4<f32>;
    fn update(&mut self, window: &mut glfw::Window);
    fn mouse_move(&mut self, xpos: f64, ypos: f64);
}
