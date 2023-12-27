use home;
use std::fs;
use std::process::Command;

pub fn setup() {
    let home_dir = home::home_dir().unwrap();

    // create bin folder in home directory
    let output = Command::new("mkdir")
        .arg("-p")
        .arg(home_dir.join("bin"))
        .output()
        .expect("❌ Failed to create bin folder");

    match output.status.code() {
        Some(_code) => println!("✅ Successfully created bin folder"),
        None => println!("✅ Bin folder already exists"),
    }

    // run ohmyposh install script

    let output = Command::new("curl")
        .arg("-s")
        .arg("https://ohmyposh.dev/install.sh")
        .arg("|")
        .arg("bash")
        .arg("-s")
        .arg("--")
        .arg("-d")
        .arg(home_dir.join("bin"))
        .output()
        .expect("❌ Failed to install ohmyposh");

    match output.status.code() {
        Some(_code) => println!("✅ Successfully installed ohmyposh"),
        None => println!("❌ Failed to install ohmyposh"),
    }

    // Add ohmyposh to .bashrc in home directory
    let bash_rc_config = "#oh-my-posh 
        eval $(~/bin/oh-my-posh init bash --config 'https://raw.githubusercontent.com/JanDeDobbeleer/oh-my-posh/main/themes/space.omp.json')";

    let bash_rc_path = home_dir.join(".bashrc");

    let mut bash_rc_file = fs::read_to_string(&bash_rc_path).expect("❌ Failed to read .bashrc");
    bash_rc_file.push_str(bash_rc_config);

    fs::write(&bash_rc_path, bash_rc_file).expect("❌ Failed to write to .bashrc");

    println!("✅ Successfully added ohmyposh to .bashrc");
}
