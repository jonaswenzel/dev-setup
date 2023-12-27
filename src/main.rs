mod tasks;

fn main() {
    tasks::lazyvim::setup();
    tasks::neovim::setup();
    tasks::tmux::setup();
    tasks::tmuxpluginmanager::setup();
    tasks::rust::setup();
    tasks::nvm::setup();
    tasks::ohmyposh::setup();

    // All done
    println!("\n");
    println!("---------------------------------------");
    println!("🤓 Your dev env is ready! Happy coding");
}
