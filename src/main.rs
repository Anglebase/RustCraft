use lazy_static::lazy_static;
use rustcraft::*;

lazy_static! {
    static ref DATA: RustCraftWrapper<i32> = RustCraftWrapper::new(10);
}

fn main() {
    DATA.apply(|data| {
        *data = 100;
        DATA.apply(|data| {
            *data = 200;
        });
    });
    DATA.apply(|data| {
        println!("data: {}", data);
    });
}