use crate::debug;
use glfw::*;

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
