use colored::*;

struct SystemTools {
    // Pacakage managers
    apt:        bool,
    yum:        bool,
    pacman:     bool,
    rpm:        bool,
    dpkg:       bool,
    dnf:        bool,
    zypper:     bool,
    snap:       bool,
    brew:       bool,
    emerge:     bool,
    nix:        bool,
    
    // Language package managers
    pip:        bool,
    pip3:       bool,
    rustup:     bool,
    npm:        bool,
    nuget:      bool,
    rubygems:   bool,

    // Applications 
    exploitdb: bool,


}


fn main() {

    let software = SystemTools {
        // Pacakage managers
        apt:        testsoftware("apt"),
        yum:        testsoftware("yum"),
        pacman:     testsoftware("pacman"),
        rpm:        testsoftware("rpm"),
        dpkg:       testsoftware("dpkg"),
        dnf:        testsoftware("dnf"),
        zypper:     testsoftware("sypper"),
        snap:       testsoftware("snap"),
        brew:       testsoftware("brew"),
        emerge:     testsoftware("emaint"),
        nix:        testsoftware("nix-env"),
        
        // Language package managers
        pip:        testsoftware("pip"),
        pip3:       testsoftware("pip3"),
        rustup:     testsoftware("rustup"),
        npm:        testsoftware("npm"),
        nuget:      testsoftware("nuget"),
        rubygems:   testsoftware("gem"),
    
        // Applications 
        exploitdb: testsoftware("searchsploit"),
    
    };
    println!("{}","Detected Software ----------------------------------------------+\n".red());
        // I am aware this is cringe I will get to it lol.
    display(software.apt, "APT");
    display(software.yum, "YUM");
    display(software.pacman, "PACMAN");
    display(software.rpm, "RPM");
    println!("{}","| \n".red());
    display(software.dpkg, "DPKG");
    display(software.dnf, "DNF");
    display(software.zypper, "ZYPPER");
    display(software.snap, "SNAP");
    println!("{}","| \n".red());
    display(software.brew, "BREW");
    display(software.emerge, "EMERGE");
    display(software.nix, "NIX");
    display(software.pip, "PIP");
    println!("{}","| \n".red());
    display(software.pip3, "PIP3");
    display(software.rustup, "RUSTUP");
    display(software.npm, "NPM");
    display(software.nuget, "NUGET");
    println!("{}","| \n".red());
    display(software.rubygems, "GEM");
    display(software.exploitdb, "EXDB\t\t\t\t");
    println!("{}","| \n".red());

    println!("{}","Updating Software ----------------------------------------------+\n".red());
    use indicatif::ProgressBar;
    use std::process::Command;
    let p = ProgressBar::new_spinner();
        if software.apt == true {
            p.set_message("Processing ... APT\t:\tUpdating");
            Command::new("apt").args(["update", "-y"]).output().expect("Failed to update APT.");
            Command::new("apt").args(["autoclean", "-y"]).output().expect("Failed to upgrade APT.");
            p.set_message("Processing ... APT\t:\tUpgrading");
            Command::new("apt").args(["full-upgrade", "-y"]).output().expect("Failed to upgrade APT.");
            p.set_message("Processing ... APT\t:\tCleaning");
            Command::new("apt").args(["autoremove", "-y"]).output().expect("Failed to update APT.");
            p.set_message("Processing ... APT\t:\tDone");

        };
        if software.yum == true {
            p.set_message("Processing ... YUM\t:\tUpdate");
            Command::new("yum").arg("check-update").output().expect("Failed to update YUM.");
            p.set_message("Processing ... YUM\t:\tUpgrade");
            Command::new("yum").arg("update").output().expect("Failed to upgrade YUM.");
        };
        if software.pacman == true {
            p.set_message("Processing ... PACMAN\t:\tUpdate");
            Command::new("pacman").arg("-Syy").output().expect("Failed to update PACMAN.");
            p.set_message("Processing ... PACMAN\t:\tUpgrade");
            Command::new("pacman").arg("-Syu").output().expect("Failed to upgrade PACMAN.");
            p.set_message("Processing ... PACMAN\t:\tDONE");
        };
        if software.dnf == true {
            p.set_message("Processing ... DNF\t:\tupdate");
            Command::new("dnf").args(["upgrade", "--refresh"]).output().expect("Failed to update DNF.");
            p.set_message("Processing ... DNF\t:\tInstall");
            Command::new("dnf").args(["install", "dnf-plugin-system-upgrade"]).output().expect("Failed to install system update package.");
            p.set_message("Processing ... DNF\t:\tUpgrade");
            Command::new("dnf").args(["system-upgrade", "download", "--releasever=34"]).output().expect("Failed to upgrade DNF.");
            p.set_message("Processing ... DNF\t:\tDone");
        };
        if software.zypper == true {
            p.set_message("Processing ... ZYPPER\t:\tRefresh");
            Command::new("zypper").arg("refresh").output().expect("Failed to update ZYPPER.");
            p.set_message("Processing ... ZYPPER\t:\tUpdate");
            Command::new("zypper").arg("update").output().expect("Failed to upgrade DNF.");
            p.set_message("Processing ... ZYPPER\t:\tDone");
        };
        if software.snap == true {
            p.set_message("Processing ... SNAP\t:\tUpdate");
            Command::new("snap").arg("refresh").output().expect("Failed to update SNAP.");
            p.set_message("Processing ... SNAP\t:\tDone");
        };
        if software.brew == true {
            p.set_message("Processing ... BREW\t:\tUpdate");
            Command::new("brew").arg("update").output().expect("Failed to update BREW.");
            p.set_message("Processing ... BREW\t:\tUpgrade");
            Command::new("brew").arg("upgrade").output().expect("Failed to upgrade BREW.");
            p.set_message("Processing ... BREW\t:\tDone");
        };
        if software.emerge == true {
            p.set_message("Processing ... EMERGE\t:\tUpdate");
            Command::new("emaint").args(["--auto", "sync"]).output().expect("Failed to update EMERGE.");
            p.set_message("Processing ... EMERGE\t:\tDone");
        };
        if software.nix == true {
            p.set_message("Processing ... NIX\t:\tUpdate");
            Command::new("nix-channel").arg("--update").output().expect("Failed to update NIX.");
            p.set_message("Processing ... NIX\t:\tUpgrade");
            Command::new("nixos-rebuild").arg("switch").output().expect("Failed to upgrade NIX.");
            p.set_message("Processing ... NIX\t:\tClean");
            Command::new("nix-store").args(["--verify", "--check-contents"]).output().expect("Failed to verify dependencied for NIX.");
            p.set_message("Processing ... NIX\t:\tDone");
        };
        if software.pip == true {
            p.set_message("Processing ... PIP\t:\tUpdate");
            Command::new("python").args(["-m", "pip", "install", "--upgrade", "pip"]).output().expect("Failed to update PIP");
            p.set_message("Processing ... PIP\t:\tDone");
        };
        if software.pip3 == true {
           p.set_message("Processing ... PIP3\t:\tUpdate");
            Command::new("python").args(["-m", "pip3", "install", "--upgrade", "pip3"]).output().expect("Failed to update PIP3.");
            p.set_message("Processing ... PIP3\t:\tDone");
        };
        if software.rustup == true {
            p.set_message("Processing ... RUSTUP\t:\tUpdate");
            Command::new("rustup").arg("update").output().expect("Failed to update RUSTUP.");
            p.set_message("Processing ... RUSTUP\t:\tDone");
        };
        if software.npm == true {
            p.set_message("Processing ... NPM\t:\tUpdate");
            Command::new("npm").args(["install", "npm@latest"]).output().expect("Failed to update NPM.");
            p.set_message("Processing ... NPM\t:\tDONE");
        };
        if software.nuget == true {
            p.set_message("Processing ... NUGET\t:\tUpdate Executable");
            Command::new("nuget").arg("update").output().expect("Failed to update NUGET executable.");
            p.set_message("Processing ... NUGET\t:\tUpdate Software");
            Command::new("Update-Package").output().expect("Failed to update NUGET packages.");
            p.set_message("Processing ... NUGET\t:\tDone");
        };


}



fn testsoftware(input: &str) -> bool  {

    let result = which::which(&input);
    let out = match &result {
        Ok(_)   => true,
        Err(_)  => false,
    };

    return out

}

fn display(input: bool, name: &str){
    if input == true {
        print!("[{}]: {} \t", "^".yellow(),name);
    } else {
        print!("[ ]: {} \t", name);
    };
}


