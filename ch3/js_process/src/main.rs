use std::process::{Command, Stdio};

fn main() {
    let _top_child = Command::new("/home/wangmingjie/develop/osc/ch3/js_process/ltr_process")
        .stdin(Stdio::inherit())
        .stdout(Stdio::null())
        .spawn()
        .expect("top command failed to start");
    println!("Hello, world!");
}
