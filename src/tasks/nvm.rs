use std::process::Command;

pub fn setup() {
    println!("\n");
    println!("📦 Installing NVM");
    // Install nvm
    let output = Command::new("curl")
        .arg("-o-")
        .arg("https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh")
        .arg("|")
        .arg("bash")
        .output()
        .expect("❌ Failed to install nvm");
    // Check if the command ran successfully
    match output.status.code() {
        Some(_code) => println!("✅ Successfully installed nvm 0.39.7"),
        None => println!("❌ Failed to install nvm"),
    }
}
