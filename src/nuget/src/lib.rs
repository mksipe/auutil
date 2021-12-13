use std::process::Command;

// Linux only command no need for windows compatibility.

fn update() -> i8 {
    match Command::new("sh").args(&["-c","nuget update -self"]).status() {
        Err(_e)         => return 1,
        Ok(_process)    => return 0,
    };
}

fn upgrade() -> i8 {
    match Command::new("sh").args(&["-c","nuget update"]).status() {
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
    let nugarr: [i8; 2] = [update(), upgrade()];
    return nugarr
}
