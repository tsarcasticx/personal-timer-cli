use std::{env::{self}, io::{self, Write}, process::{exit, Command}, thread::sleep, time::Duration, u64};


fn responduser(msg: &str) -> String {
    print!("{msg}") ;
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let throw = input.trim();
    return throw.to_string();
}
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
    let alrdir = env::current_exe();
    let options = responduser("What time would you want to set? \n [1] Second\n [2] Minute\n [3] Hour \nEnter the options: ");
    let optionstr: &str = options.as_str();

    let duration: u64;
    let _jeda;
    match optionstr {
        "1" => {duration = durasi("Enter the duration in second: ");
        _jeda = Duration::from_secs(duration)},
        "2" => {duration = durasi("Enter the duration in minute: ");
        _jeda = Duration::from_secs(duration * 60)},
        "3" => {duration = durasi("Enter the duration in hour: ");
        _jeda = Duration::from_secs(duration * 3600)},
        _ => {eprintln!("You must choose one of them by entering the options");
            exit(1)},
    }

    sleep(_jeda);
    let _notification = Command::new("notify-send").args(["󰔛 Time's up", &format!("You've reached {duration} seconds")]).spawn().expect("could not run the command");
    let _alarm = Command::new("nohup").args(["mpv", format!("{}/alarm.wav",alrdir.expect("the file must be the same as executable's directory").display()).as_str(), ">", "/dev/null", "2>&1", "&"]).
        spawn().expect("could not run the command");
    println!("\n\n\nYou've reached {duration} seconds\n\n");
}
