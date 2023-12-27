use std::env;
use std::process::Command;

pub fn setup() {
    println!("\n");
    println!("📦 Installing LazyVim");

    // Install dependencies
    let output = Command::new("sudo")
        .arg("apt")
        .arg("install")
        .arg("gcc")
        .arg("g++")
        .arg("make")
        .arg("git")
        .output()
        .expect("❌ Failed to install dependencies");

    match output.status.code() {
        Some(_code) => println!("✅ Successfully installed dependencies"),
        None => println!("❌ Failed to install dependencies"),
    }

    // Clone lazyvim
    let nvim_config_path = home::home_dir().unwrap().join(".config/nvim");
    let output = Command::new("git")
        .arg("clone")
        .arg("https://github.com/LazyVim/starter")
        .arg(nvim_config_path)
        .output()
        .expect("❌ Failed to clone LazyVim starter");

    match output.status.code() {
        Some(_code) => println!("✅ Successfully cloned LazyVim starter"),
        None => println!("❌ Failed to clone LazyVim starter"),
    }

    // Remove .git folder
    let nvim_config_git_path = home::home_dir().unwrap().join(".config/nvim/.git");
    let output = Command::new("rm")
        .arg("-rf")
        .arg(nvim_config_git_path)
        .output()
        .expect("❌ Failed to remove LazyVim git-folder");

    match output.status.code() {
        Some(_code) => println!("✅ Successfully removed LazyVim git-folder"),
        None => println!("❌ Failed to remove LazyVim git-folder"),
    }

    // Copy lazyvim lazy.lua
    let lazy_config_path = env::current_dir().unwrap().join("configs/lazy.lua");
    let lazyvim_config_path = home::home_dir().unwrap().join(".config/nvim/lua/lazy.lua");
    let output = Command::new("cp")
        .arg("-r")
        .arg(lazy_config_path)
        .arg(lazyvim_config_path)
        .output()
        .expect("❌ Failed to copy nvim config");

    match output.status.code() {
        Some(_code) => println!("✅ Successfully copied lazy config"),
        None => println!("❌ Failed to copy lazy config"),
    }
}
