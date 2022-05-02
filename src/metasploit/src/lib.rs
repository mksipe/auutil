use std::process::Command;

// Linux only command no need for windows compatibility.

fn update() -> i8 {
    match Command::new("sh").args(&["-c","apt update -y"]).status() {
        Err(_e)         => return 1,
        Ok(_process)    => return 0,
    };
}

fn upgrade() -> i8 {
    match Command::new("sh").args(&["-c","apt install metasploit-framework -y"]).status() {
        Err(_e)         => return 1,
        Ok(_process)    => return 0,
    };
}

fn reinitdb() -> i8 {
    match Command::new("sh").args(&["-c","msfdb reinit"]).status() {
        Err(_e)         => return 1,
        Ok(_process)    => return 0,
    };
}

fn autoremove() -> i8 {
    match Command::new("sh").args(&["-c","apt autoremove -y"]).status() {
        Err(_e)         => return 1,
        Ok(_process)    => return 0,
    };
}


fn autoclean() -> i8 {
    match Command::new("sh").args(&["-c","apt autoclean -y"]).status() {
        Err(_e)         => return 1,
        Ok(_process)    => return 0,
    };
}
pub fn checkinstallation(input: bool) {
    if input == true {
        capresult();
    }
}
fn capresult() -> [i8; 5] {
    let outarr: [i8; 5] = [update(), upgrade(), reinitdb(), autoremove(), autoclean()];
    return outarr
}