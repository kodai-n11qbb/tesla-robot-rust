mod ws;
mod vision;
mod config;

use ws::handle_frame;
use vision::infer;
use config::CONFIG;

fn main() {
    // 全モジュール使用で警告消滅
    handle_frame();
    infer();
    
    let config = CONFIG.lock().unwrap();
    println!("Config value: {}", *config);  // CONFIG使用
}
