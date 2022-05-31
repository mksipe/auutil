use std::process::Command;

fn upgrade() -> i8 {
    match Command::new("sh").args(&["-c","cpan -u"]).status() {
        Err(_e)         => return 1,
        Ok(_process)    => return 0,
    };
}

pub fn checkinstallation(input: bool) {
    if input == true {
        capresult();
    }
}
fn capresult() -> [i8; 1] {
    let cpanarr: [i8; 1] = [upgrade()];
    return cpanarr
}