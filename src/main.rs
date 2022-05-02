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

fn update_and_display_profile<'a> (app: SoftwareProfile<'a>){
    if app.installed == true {
        println!("Profile: {} Loaded. ", app.title);
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




    println!("{}{:-^150}{}", "|", "Detected Software Summary".red(),"+");
    // I am aware this is cringe I will get to it lol.
    display(app1.installed, app1.title);
    display(app2.installed, app2.title);
    display(app3.installed, app3.title);
    display(app4.installed, app4.title);
    display(app5.installed, app5.title);
    display(app6.installed, app6.title);
    println!("\n{:>152}","|".red());
    display(app7.installed, app7.title);
    display(app8.installed, app8.title);
    display(app9.installed, app9.title);
    display(app10.installed, app10.title);
    display(app11.installed, app11.title);
    display(app12.installed, app12.title);
    println!("\n{:>152}","|".red());
    display(app13.installed, app13.title);
    display(app14.installed, app14.title);
    display(app15.installed, app15.title);
    display(app16.installed, app16.title);
    display(app17.installed, app17.title);
    display(app18.installed, app18.title);
    println!("\n{:>152}","|".red());
    display(app19.installed, app19.title);
    display(app20.installed, app20.title);
    display(app21.installed, app21.title);
    display(app22.installed, app22.title);
    display(app23.installed, app23.title);
    display(app24.installed, app24.title);

    println!("\n{}{:-^150}{}", "|", "Processes ".red(),"+");

    
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
        print!("[{}]: {:<20}", "X".white().bold(),name.yellow());
    } else {
        print!("[ ]: {:<20}", name);
    };
}

