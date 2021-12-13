use std::process::Command;

// Linux only command no need for windows compatibility.

fn update() -> i8 {
    match Command::new("sh").args(&["-c","nix-channel --update"]).status() {
        Err(_e)         => return 1,
        Ok(_process)    => return 0,
    };
}

fn upgrade() -> i8 {
    match Command::new("sh").args(&["-c","nixos-rebuild switch"]).status() {
        Err(_e)         => return 1,
        Ok(_process)    => return 0,
    };
}

fn clean() -> i8 {
    match Command::new("sh").args(&["-c", "nix-store --verify --check-contents"]).status(){
        Err(_e)         => return 1,
        Ok(_process)    => return 0,        
    }
}

pub fn checkinstallation(input: bool) {
    if input == true {
        capresult();
    }
}


fn capresult() -> [i8; 3] {
    let nixarr: [i8; 3] = [update(), upgrade(), clean()];
    return nixarr
}
