use std::process::Command;

pub fn setup() {
    println!("\n");
    println!("📦 Installing Tmux");

    // Install tmux
    let output = Command::new("sudo")
        .arg("apt")
        .arg("install")
        .arg("tmux")
        .output()
        .expect("❌ Failed to install tmux");
    // Check if the command ran successfully
    match output.status.code() {
        Some(_code) => println!("✅ Successfully installed tmux"),
        None => println!("❌ Failed to install tmux"),
    }
}
