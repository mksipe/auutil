use colored::*;
use apt::checkinstallation          as check1;
use rust::checkinstallation         as check2;
use yum::checkinstallation          as check3; 
use pacman::checkinstallation       as check4;
use dnf::checkinstallation          as check5;
use zypper::checkinstallation       as check6;
use snap::checkinstallation         as check7;
use brew::checkinstallation         as check8;
use emerge::checkinstallation       as check9;
use emerge::checkinstallation       as check10;
use pip::checkinstallation          as check11;
use pip3::checkinstallation         as check12;
use npm::checkinstallation          as check13;
use nuget::checkinstallation        as check14;
use ruby::checkinstallation         as check15;
use exdb::checkinstallation         as check16;
use gvm::checkinstallation          as check17;
use osx::checkinstallation          as check18;
use clamav::checkinstallation       as check19;
use flatpak::checkinstallation      as check20;
use metasploit::checkinstallation   as check21;

struct SystemTools {

    // System
    osx:       bool,

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
    flatpak:    bool,
    metasploit: bool,
    
    // Language package managers
    pip:        bool,
    pip3:       bool,
    rustup:     bool,
    npm:        bool,
    nuget:      bool,
    rubygems:   bool,

    // Applications 
    exploitdb: bool,
    gvm:       bool,
    clamav:    bool,
}


use futures::executor::block_on;

async fn updatefunction() {
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
        flatpak:    testsoftware("flatpak"),
        metasploit: testsoftware("msfconsole"),
        
        // Language package managers
        pip:        testsoftware("pip"),
        pip3:       testsoftware("pip3"),
        rustup:     testsoftware("rustup"),
        npm:        testsoftware("npm"),
        nuget:      testsoftware("nuget"),
        rubygems:   testsoftware("gem"),
    
        // Applications 
        exploitdb:  testsoftware("searchsploit"),
        gvm:        testsoftware("gvmd"),
        clamav:     testsoftware("clamav"),
        // System
        osx:        testsoftware("softwareupdate"),
    };
    check1(software.apt);
    check2(software.rustup);
    check3(software.yum);
    check4(software.pacman);
    check5(software.dnf);
    check6(software.zypper);
    check7(software.snap);
    check8(software.brew);
    check9(software.emerge);
    check10(software.nix);
    check11(software.pip);
    check12(software.pip3);
    check13(software.npm);
    check14(software.nuget);
    check15(software.rubygems);
    check16(software.exploitdb);
    check17(software.gvm);
    check18(software.osx);
    check19(software.clamav);
    check20(software.flatpak);
    check21(software.metasploit);
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
        flatpak:    testsoftware("flatpak"),
        
        // Language package managers
        pip:        testsoftware("pip"),
        pip3:       testsoftware("pip3"),
        rustup:     testsoftware("rustup"),
        npm:        testsoftware("npm"),
        nuget:      testsoftware("nuget"),
        rubygems:   testsoftware("gem"),
    
        // Applications 
        exploitdb:  testsoftware("searchsploit"),
        gvm:        testsoftware("gvmd"),
        clamav:     testsoftware("clamav"),
        metasploit: testsoftware("msfconsole"),

        // System
        osx:        testsoftware("softwareupdate"),

    
    };
    println!("{}","Detected Software ----------------------------------------------+\n".red());
        // I am aware this is cringe I will get to it lol.
    display(software.apt, "APT");display(software.yum, "YUM");display(software.pacman, "PACMAN");display(software.rpm, "RPM");
    println!("{}","| \n".red());
    display(software.dpkg, "DPKG");display(software.dnf, "DNF");display(software.zypper, "ZYPPER");display(software.snap, "SNAP");
    println!("{}","| \n".red());
    display(software.brew, "BREW");display(software.emerge, "EMERGE");display(software.nix, "NIX");display(software.pip, "PIP");
    println!("{}","| \n".red());
    display(software.pip3, "PIP3");display(software.rustup, "RUSTUP");display(software.npm, "NPM");display(software.nuget, "NUGET");
    println!("{}","| \n".red());
    display(software.rubygems, "GEM");display(software.exploitdb, "EXDB");display(software.gvm, "GVM");display(software.osx, "OSX");
    println!("{}","| \n".red());
    display(software.clamav, "CLAMAV");
    display(software.metasploit, "MSF");
    display(software.flatpak, "FLATPAK\t\t");
    println!("{}","| \n".red());
    println!("{}","Updating Software ----------------------------------------------+\n".red());
    block_on(updatefunction());
    println!("{}","\nSoftware Update Complete ---------------------------------------+\n".red());

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
        print!("[{}]: {} \t", "X".white().bold(),name.yellow());
    } else {
        print!("[ ]: {} \t", name);
    };
}

