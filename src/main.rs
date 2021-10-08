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
    cargo:      bool,
    go:         bool,
    maven:      bool,
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
        cargo:      testsoftware("cargo"),
        go:         testsoftware("go"),
        maven:      testsoftware("mvn"),
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
    display(software.cargo, "CARGO");
    display(software.go, "GO");
    display(software.maven, "MAVEN");
    println!("{}","| \n".red());
    display(software.npm, "NPM");
    display(software.nuget, "NUGET");
    display(software.rubygems, "GEM");
    display(software.exploitdb, "EXDB");
    println!("{}","| \n".red());

    println!("{}","Updating Software ----------------------------------------------+\n".red());

    use std::process::Command;

    if software.apt == true {
        println!("Updating: {}", "APT".green());
        Command::new("apt").args(["update", "-y"]).output().expect("Failed to update APT.");
        Command::new("apt").args(["autoclean", "-y"]).output().expect("Failed to upgrade APT.");
        Command::new("apt").args(["full-upgrade", "-y"]).output().expect("Failed to upgrade APT.");
        println!("\nRemoving unnesessary files.\n");
        Command::new("apt").args(["autoremove", "-y"]).output().expect("Failed to update APT.");
    };
    if software.yum == true {
        println!("Updating: {}", "YUM".green());
        Command::new("yum").arg("check-update").output().expect("Failed to update YUM.");
        Command::new("yum").arg("update").output().expect("Failed to upgrade YUM.");
    };
    if software.pacman == true {
        println!("Updating: {}", "PACMAN".green());
        Command::new("pacman").arg("-Syy").output().expect("Failed to update PACMAN.");
        Command::new("pacman").arg("-Syu").output().expect("Failed to upgrade PACMAN.");
    };
    if software.dnf == true {
        println!("Updating: {}", "DNF".green());
        Command::new("dnf").args(["upgrade", "--refresh"]).output().expect("Failed to update DNF.");
        Command::new("dnf").args(["install", "dnf-plugin-system-upgrade"]).output().expect("Failed to install system update package.");
        Command::new("dnf").args(["system-upgrade", "download", "--releasever=34"]).output().expect("Failed to upgrade DNF.");
    };
    if software.zypper == true {
        println!("Updating: {}", "ZYPPER".green());
        Command::new("zypper").arg("refresh").output().expect("Failed to update ZYPPER.");
        Command::new("zypper").arg("update").output().expect("Failed to upgrade DNF.");
    };
    if software.snap == true {
        println!("Updating: {}", "SNAP".green());
        Command::new("snap").arg("refresh").output().expect("Failed to update SNAP.");
    };
    if software.brew == true {
        println!("Updating: {}", "BREW".green());
        Command::new("brew").arg("update").output().expect("Failed to update BREW.");
        Command::new("brew").arg("upgrade").output().expect("Failed to upgrade BREW.");
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
        print!("[{}]: {} \t", "X".green(),name);
    } else {
        print!("[ ]: {} \t", name);
    };
}


