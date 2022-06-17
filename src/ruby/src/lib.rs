
use std::process::Command;


// Cross system. 


fn update() -> i8 {
    match Command::new("gem").args(&["update","--verbose"]).status() {
        Ok(_process)    => return 0,
        Err(_e)         => return 1
    }

}

pub fn checkinstallation(input: bool) -> Vec<i8> {
    if input == true {
        return vec![update()];
    } else {
        return vec![-5,-5,-5,-5];
    }
}