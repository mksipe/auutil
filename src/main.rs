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
use nix::checkinstallation          as check10;
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

struct SoftwareProfile<'a> {
    
    // software title
    title: &'a str,
    
    //is it installed?
    installed: bool,

    //software tyoe
    stype: &'a str,

    //description
    desc: &'a str,
}

//use futures::executor::block_on;

fn create_software_profile<'a> (name: &'a str, swtype: &'a str, description: &'a str) -> SoftwareProfile<'a> {
    SoftwareProfile {
        title: name,
        installed: testsoftware(name),
        stype: swtype,
        desc: description,
    }
}



fn main() {

    //package managers
    let app1 = create_software_profile("apt", "Package Manager", "APT is a management system for software packages.");
    let app2 = create_software_profile("yum", "Pacakge Manager", "yum is an interactive, rpm based, package manager.");
    let app3 = create_software_profile("pacman", "Package Manager", "Manages Archlinux packages ");
    let app4 = create_software_profile("rpm", "Package Manager", "rpm is a powerful Package Manager, which can be used to build, install, query, verify, update, and erase individual software packages. ");
    let app5 = create_software_profile("dpkg", "Package Manager", "dpkg is a tool to install, build, remove and manage Debian packages.");
    let app6 = create_software_profile("dnf", "Package Manager", "DNF is a software package manager that installs, updates, and removes packages on Fedora and is the successor to YUM ");
    let app7 = create_software_profile("zypper", "Package Manager", "zypper is a command-line interface to ZYpp system management library (libzypp).");
    let app8 = create_software_profile("snap", "Package Manager", " The snap tool interacts with the snapd daemon to control the snappy software platform. ");
    let app9 = create_software_profile("brew", "Package Manager", "Homebrew is the easiest and most flexible way to install the UNIX tools Apple didn't include with macOS. ");
    let app10 = create_software_profile("emaint", "Package Manager", "The emaint program provides a command line interface to package management health checks and maintenance.  ");
    let app11 = create_software_profile("nix-env", "Package Manager", "The command nix-env is used to manipulate Nix user environments.");
    let app12 = create_software_profile("flatpak", "Package Manager", "flatpak is a tool for managing applications and the runtimes they use.");

    //language package managers

    let app13 = create_software_profile("pip", "Language", "Python is an interpreted, interactive, object-oriented programming language that combines remarkable power with very clear syntax. ");
    let app14 = create_software_profile("pip3", "Language", "Python is an interpreted, interactive, object-oriented programming language that combines remarkable power with very clear syntax. ");
    let app15 = create_software_profile("rustup", "Language", " rustup is an installer for the systems programming language Rust ");
    let app16 = create_software_profile("npm", "Language", "npm is the package manager for the Node JavaScript platform.");
    let app17 = create_software_profile("nuget", "Language", "The NuGet Command Line Interface (CLI), nuget.exe, provides the full extent of NuGet functionality to install, create, publish, and manage packages without making any changes to project files.");
    let app18 = create_software_profile("gem", "Language", "Ruby is an interpreted scripting language for quick and easy object-oriented programming.");

    //application specific

    let app19 = create_software_profile("gvmd", "Application", "The OpenVAS Security Scanner is a security auditing tool made up of two parts: a server, and a client. The server, openvasd is in charge of the attacks, whereas the client OpenVAS-Client provides an interface to the user. ");
    let app20 = create_software_profile("searchsploit", "Application", "searchsploit, a command line search tool for Exploit-DB ");
    let app21 = create_software_profile("clamav", "Application", "clamscan is a command line anti-virus scanner. ");
    let app22 = create_software_profile("msfconsole", "Application", "MSFconsole provides a command line interface to access and work with the Metasploit Framework. ");

    //system specific

    let app23 = create_software_profile("softwareupdate", "System", "Apple update utility");


    //create an array of each profile 
    let profiles: [SoftwareProfile; 23] = [
        app1,  app2,  app3,  app4,  app5,  app6,  app7,  app8,  app9, 
        app10, app11, app12, app13, app14, app15, app16, app17, app18, app19, 
        app20, app21, app22, app23
    ];

    println!("{}{:-^150}{}", "|", "Detected Software Summary".red(),"+");

    for i in &profiles {
 
        display(i.installed, i.title);
        print!("{:>5} ", i.stype);
        println!("{:>5}", i.desc);

    }

    println!("\n{}{:-^150}{}", "|", "Processes".red(),"+");


    for i in &profiles {
        match i.title {
            "apt"           => check1(  i.installed),
            "rustup"        => check2(  i.installed),
            "yum"           => check3(  i.installed),
            "pacman"        => check4(  i.installed),
            "dnf"           => check5(  i.installed),
            "zypper"        => check6(  i.installed),
            "snap"          => check7(  i.installed),
            "brew"          => check8(  i.installed),
            "emaint"        => check9(  i.installed),
            "nix-env"       => check10( i.installed),
            "pip"           => check11( i.installed),
            "pip3"          => check12( i.installed),
            "npm"           => check13( i.installed),
            "nuget"         => check14( i.installed),
            "gem"           => check15( i.installed),
            "searchsploit"  => check16( i.installed),
            "gvmd"          => check17( i.installed),
            "softwareupdate"=> check18( i.installed),
            "clamav"        => check19( i.installed),
            "flatpak"       => check20( i.installed),
            "msfconsole"    => check21( i.installed),
            _               => print!(""),
        }
    }


    println!("{}{:-^150}{}", "|", "Software Update Complete".red(),"+");


    
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
        print!("[{}]: {:<15}", "X".white().bold(),name.yellow());
    } else {
        print!("[ ]: {:<15}", name);
    };
}

