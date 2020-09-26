use std::fmt;
use std::env::var;
use std::process::Command;

pub mod colors;
pub use crate::colors::Colors;

#[derive(Debug)]
pub struct Title {
	username: String,
	hostname: String
}

pub fn username() -> String {
	var("USERNAME").unwrap()
}

#[cfg(target_family = "unix")]
pub fn hostname() -> Result<String, bool> {
	Ok(var("HOSTNAME").unwrap());
}

#[cfg(target_family = "windows")]
pub fn hostname() -> Result<String, bool> {
	let output = Command::new("hostname")
		.output();

	match output {
		Ok(output) => return Ok(String::from_utf8(output.stdout).unwrap()),
		Err(_) => return Err(true),
	}
}

pub fn title() -> Title {
	let username = username();
	let hostname = hostname().unwrap_or("Unknown".to_string());

	Title {
		username,
		hostname
	}
}

impl fmt::Display for Title {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}@{}", self.username, self.hostname)
	}
}

#[macro_export]
macro_rules! printr {
    () => (print!("\x1b[0m"));
    ($($arg:tt)*) => ({
        print!("{}\x1b[0m", format_args!($($arg)*));
    })
}

#[macro_export]
macro_rules! printlnr {
	() => (println!("\x1b[0m"));
	($($arg:tt)*) => ({
		println!("{}\x1b[0m", format_args!($($arg)*));
	})
}