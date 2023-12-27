use home;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::process::Command;

static TMUX_CONFIG_PATH: &str = "configs/tmux.conf";

pub fn setup() {
    println!("\n");
    println!("📦 Installing Tmux Plugin Manager");

    let tmux_plugins_tpm_path = home::home_dir().unwrap().join("tmux/plugins/tpm");

    // Clone tmux plugin manager
    let output = Command::new("git")
        .arg("clone")
        .arg("https://github.com/tmux-plugins/tpm")
        .arg(tmux_plugins_tpm_path)
        .output()
        .expect("❌ Failed to clone Tmux Plugin Manager");

    match output.status.code() {
        Some(_code) => println!("✅ Successfully installed Tmux Plugin Manager"),
        None => println!("❌ Failed to install Tmux Plugin Manager"),
    }

    // Create .tmux.conf if it doesn't exist.
    println!("\n");
    println!("📝 Creating .tmux.conf");
    let tmux_conf_path = home::home_dir().unwrap().join(".tmux.conf");

    if !tmux_conf_path.exists() {
        let current_dir = env::current_dir().unwrap();
        let default_config_path = current_dir.join(TMUX_CONFIG_PATH);

        let file_content =
            fs::read_to_string(default_config_path).expect("❌ Failed to read tmux default config");

        let mut file = fs::File::create(tmux_conf_path).expect("❌ Failed to create .tmux.conf");

        file.write_all(file_content.as_bytes())
            .expect("❌ Failed to write to .tmux.conf");

        println!("✅ .tmux.conf created");
    } else {
        println!("⚠️ .tmux.conf already exists");
        println!("ℹ️ Please configure it manually:  https://github.com/tmux-plugins/tpm");
    }
}
