use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static CONFIG: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));
