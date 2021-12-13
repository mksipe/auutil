use std::process::Command;

fn update() -> i8 {

    match Command::new("rustup").arg("update").status() {
        Err(_e)         => return 1,
        Ok(_process)    => return 0,
    };
}

pub fn checkinstallation(input: bool)  {
    if input == true {
        update();
    }
}
