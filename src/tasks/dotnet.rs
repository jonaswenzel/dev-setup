use std::process::Command;

pub fn setup() {
    println!("\n");
    println!("📦 Installing Dotnet 6.0");

    // Install dotnet
    let output = Command::new("sudo")
        .arg("apt")
        .arg("update")
        .output()
        .expect("❌ Failed to update apt");

    match output.status.code() {
        Some(_code) => println!("✅ Successfully updated apt"),
        None => println!("❌ Failed to update apt"),
    }

    let output = Command::new("sudo")
        .arg("apt")
        .arg("install")
        .arg("dotnet-sdk-6.0")
        .output()
        .expect("❌ Failed to install dotnet");

    match output.status.code() {
        Some(_code) => println!("✅ Successfully installed dotnet"),
        None => println!("❌ Failed to install dotnet"),
    }
}
