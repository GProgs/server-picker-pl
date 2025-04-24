use crate::menu::Menu;
use serde::Deserialize;
use std::error::Error;
use std::time::Duration;

/*
keys_mvp = '88882'
key_chat = 'y'
dt_launch = 30s
dt = 250ms

[menu]
addrs = ['95.216.32.28:27015', '95.216.32.28:27016', '95.216.32.28:27019', '95.216.32.28:27017', '95.216.32.28:27018']
names = ['Suomipubla', 'Retake', 'Retake #2', 'Vankilapako', 'Awikka']
*/

#[derive(Deserialize)]
pub struct Config {
	pub keys_mvp: String, // What keys for your MVP (i.e. 3 for Mirella, or 88882 for smthing else)
	pub key_chat: char, // What key you use for chat (i.e. y)
	#[serde(with = "humantime_serde")]
	pub dt_launch: Duration,
	#[serde(with = "humantime_serde")]
	pub dt: Duration,
	pub menu: Menu,
}

impl Config {

	// Read a toml file into a config.
	pub fn read_file(path: String) -> Result<Self,Box<dyn Error>> {
		let cont = std::fs::read_to_string(path)?;
		Ok(toml::from_str(&cont)?)
	}
}