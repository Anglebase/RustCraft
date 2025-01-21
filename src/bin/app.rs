use rustcraft::{app::*, log::*};

fn main() {
    set_level(Level::Debug);
    let mut app = App::new(800,600, "Rustcraft");
    app.exec();
} 