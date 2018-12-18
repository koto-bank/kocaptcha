use std::sync::RwLock;
use lazy_static::lazy_static;
lazy_static! {
    static ref captchas_generated: RwLock<i64> = RwLock::new(0);
}

pub fn get() -> i64 {
    return *captchas_generated.read().unwrap();
}

pub fn inc() {
    let mut gend = captchas_generated.write().unwrap();
    *gend += 1;
}
