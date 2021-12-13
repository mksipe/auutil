use std::process::Command;

// Linux only command no need for windows compatibility.

fn update() -> i8 {
    match Command::new("sh").args(&["-c","dnf upgrade --refresh"]).status() {
        Err(_e)         => return 1,
        Ok(_process)    => return 0,
    };
}

fn install() -> i8 {
    match Command::new("sh").args(&["-c","dnf install dnf-plugin-system-upgrade"]).status() {
        Err(_e)         => return 1,
        Ok(_process)    => return 0,
    };
}

fn upgrade() -> i8 {
    match Command::new("sh").args(&["-c","dnf system-upgrade download releasever=34"]).status() {
        Err(_e)         => return 1,
        Ok(_process)    => return 0,
    };
}


pub fn checkinstallation(input: bool) {
    if input == true {
        capresult();
    }
}
fn capresult() -> [i8; 3] {
    let dnfarr: [i8; 3] = [update(), install(), upgrade()];
    return dnfarr
}