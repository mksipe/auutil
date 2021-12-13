
use std::process::Command;


// Cross system. 


fn update() -> i8 {
    match Command::new("gem").args(&["update","--verbose"]).status() {
        Ok(_process)    => return 0,
        Err(_e)         => return 1
    }

}

pub fn checkinstallation(input: bool) {
    if input == true {
        capresult();
    } 
}


fn capresult() -> [i8; 1] {
    let gemarr: [i8; 1] = [update()];
    return gemarr
}


