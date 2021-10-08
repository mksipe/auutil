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

    println!("\nDetected Software :\n");

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


