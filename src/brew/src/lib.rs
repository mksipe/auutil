use std::process::Command;

// Cross system no need for windows support.

fn update() -> i8 {
    match Command::new("sh").args(&["-c","brew update"]).status() {
        Err(_e)         => return 1,
        Ok(_process)    => return 0,
    };
}

fn upgrade() -> i8 {
    match Command::new("sh").args(&["-c","brew updgrade"]).status() {
        Err(_e)         => return 1,
        Ok(_process)    => return 0,
    };
}

pub fn checkinstallation(input: bool) {
    if input == true {
        capresult();
    }
}


fn capresult() -> [i8; 2] {
    let aptarr: [i8; 2] = [update(), upgrade(),];
    return aptarr
}
