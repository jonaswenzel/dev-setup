use std::process::Command;

pub fn setup() {
    println!("\n");
    println!("📦 Updating apt and Installing Neovim");

    // Add neovim ppa and install neovim
    let output = Command::new("sudo")
        .arg("add-apt-repository")
        .arg("ppa:neovim-ppa/stable")
        .output()
        .expect("❌ Failed to add neovim ppa");

    match output.status.code() {
        Some(_code) => println!("✅ Successfully added neovim ppa"),
        None => println!("❌ Failed to add neovim ppa"),
    }

    // Update apt
    let output = Command::new("sudo")
        .arg("apt")
        .arg("update")
        .output()
        .expect("❌ Failed to update apt");

    match output.status.code() {
        Some(_code) => println!("✅ Successfully updated apt"),
        None => println!("❌ Failed to update apt"),
    }

    // Install neovim
    let output = Command::new("sudo")
        .arg("apt")
        .arg("install")
        .arg("neovim")
        .output()
        .expect("❌ Failed to install neovim");

    match output.status.code() {
        Some(_code) => println!("✅ Successfully installed neovim"),
        None => println!("❌ Failed to install neovim"),
    }
}
