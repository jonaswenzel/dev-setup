use std::process::Command;

pub fn setup() {
    println!("\n");
    println!("📦 Installing Rustup");

    let output = Command::new("curl")
        .arg("--proto")
        .arg("'=https'")
        .arg("--tlsv1.2")
        .arg("-sSf")
        .arg("https://sh.rustup.rs")
        .arg("|")
        .arg("sh")
        .output()
        .expect("❌ Failed to install tmux");

    // Check if command ran correctlty

    match output.status.code() {
        Some(_code) => println!("✅ Successfully installed Rust"),
        None => println!("❌ Failed to install rust"),
    }
}
