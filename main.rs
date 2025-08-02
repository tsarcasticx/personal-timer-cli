use std::{io::{self, Write}, process::{exit, Command}, thread::sleep, time::Duration, u64, env};

fn check_err_int(msg: &str) -> Result<u64, &str>{
    print!("{msg}");
    let mut _data_entered = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut _data_entered).unwrap();

    let checker: Result<u64,_> = _data_entered.trim().parse();
    let positive: u64;
    let negative: &str;
    match checker {
        Ok(t) => {positive = t; return Ok(positive);},
        Err(_e) => {negative = "You must input the integer, not any"; return Err(negative);}
    }
}
fn durasi(msg: &str) -> u64{
    let number = check_err_int(msg);
    if let Err(failed) = number {
        eprintln!("{failed}");
        exit(1);
    }
    return number.unwrap();
}

fn main() {
    let current_dir = env::current_dir();
    let detiks = durasi("Enter the duration in second: ");
    let _jeda = Duration::from_secs(detiks);
    sleep(_jeda);
    // let _alarm = Command::new("mpv")
    let _notification = Command::new("notify-send").args(["󰔛 Time's up", format!("You've reached {detiks} seconds").as_str()]).spawn().expect("could not run the command");
    let _alarm = Command::new("nohup").args(["mpv", format!("{}/alarm.wav",current_dir.expect("the file must be the same as executable's directory").display()).as_str(), ">", "/dev/null", "2>&1", "&"]).
        spawn().expect("could not run the command");
    println!("\n\n\nYou've reached {detiks} seconds\n\n");
}
