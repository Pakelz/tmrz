use std::{io::Write, thread, time::Duration};

use notify_rust::Notification;

pub struct Config {
    pub duration: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Please insert right command");
        }

        let duration = args[1].clone();

        Ok(Config { duration })
    }
}

pub fn run(config: Config) -> Result<(), &'static str> {
    let duration = config.duration.clone().parse::<i32>();
    match duration {
        Ok(n) => countdown(n),
        Err(_) => {
            return Err("Please insert number input!");
        }
    }
    Ok(())
}

pub fn countdown(duration: i32) {
    for i in (0..=duration).rev() {
        print!("\r{i}    ");
        std::io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
    println!("\rTimer Done");
    Notification::new()
        .summary("Timer Done")
        .body("Well Done")
        .sound_name("message-new-instant")
        .show()
        .unwrap();
}
