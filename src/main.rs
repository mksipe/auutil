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

use futures::executor::block_on;

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
    let app1 = create_software_profile("apt", "PM", "APT");
    let app2 = create_software_profile("yum", "PM", "YUM");
    let app3 = create_software_profile("pacman", "PM", "PACMAN");
    let app4 = create_software_profile("rpm", "PM", "RPM");
    let app5 = create_software_profile("dpkg", "PM", "DPKG");
    let app6 = create_software_profile("dnf", "PM", "DNF");
    let app7 = create_software_profile("zypper", "PM", "ZYPPER");
    let app8 = create_software_profile("snap", "PM", "SNAP");
    let app9 = create_software_profile("brew", "PM", "BREW");
    let app10 = create_software_profile("emaint", "PM", "EMERGE");
    let app11 = create_software_profile("nix-env", "PM", "NIX");
    let app12 = create_software_profile("flatpak", "PM", "FLATPAK");

    //language package managers

    let app13 = create_software_profile("pip", "LANG", "PYTHON");
    let app14 = create_software_profile("pip3", "LANG", "PYTHON3");
    let app15 = create_software_profile("rustup", "LANG", "RUSTUP");
    let app16 = create_software_profile("npm", "LANG", "NPM");
    let app17 = create_software_profile("nuget", "LANG", "NUGET");
    let app18 = create_software_profile("gem", "LANG", "RUBY");

    //application specific

    let app19 = create_software_profile("searchsploit", "APP", "EXPLOITDB");
    let app20 = create_software_profile("gvmd", "APP", "OpenVAS");
    let app21 = create_software_profile("searchsploit", "APP", "EXPLOITDB");
    let app22 = create_software_profile("clamav", "APP", "CLAMAV");
    let app23 = create_software_profile("msfconsole", "APP", "METASPLOIT");

    //system specific

    let app24 = create_software_profile("softwareupdate", "SYS", "APPLEOSX");


    //create an array of each profile 
    let profiles: [SoftwareProfile; 24] = [
        app1,  app2,  app3,  app4,  app5,  app6,  app7,  app8,  app9, 
        app10, app11, app12, app13, app14, app15, app16, app17, app18, app19, 
        app20, app21, app22, app23, app24
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
            "rust"          => check2(  i.installed),
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

