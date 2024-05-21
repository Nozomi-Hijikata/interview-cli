use std::env;

pub fn init() {
    init_logger();
}

fn init_logger() {
    let rust_log = match env::var("PROFILE").as_deref() {
        Ok("release") => "info",
        _ => "debug",
    };

    env::set_var("RUST_LOG", rust_log);
    env_logger::init();
}
