use std::{ collections::HashMap, io::{ BufRead, BufReader }, process::{ Stdio, Command } };
use log::{ debug };
use regex::Regex;
use mpris::{ PlayerFinder, PlaybackStatus };

fn parse_event(test_string: &str, target_string: &str) -> Option<String> {
	let re = Regex::new(test_string).ok()?;
	let caps = re.captures(target_string)?;
	return Some(String::from(&caps[1]));
}

fn exec_cmds(cmds_vec: Vec<(&str, &[&str])>) {
	for cmd_vec in cmds_vec {
		let (cmd, args) = cmd_vec;
		Command::new(cmd)
			.args(args)
			.status()
			.ok();
	}
}

fn respond_to_event(event_type: &str, player_status: &mut HashMap<String, i32>) {
	// Find all players
	let players = PlayerFinder::new()
		.unwrap()
		.find_all()
		.unwrap();

	// Respond based on event type
	match event_type {
		"plug" => {
			// Unmute everything
			exec_cmds(vec![("amixer", &["-q", "set", "Master", "unmute"]), ("amixer", &["-q", "set", "Speaker", "unmute"]), ("amixer", &["-q", "set", "Headphone", "unmute"])]);
			debug!("Unmuting...");

			// Loop through players and play the ones that were playing before
			for player in players {
				match player_status.get(&player.bus_name().to_string()) {
					Some(status) => {
						if status == &1 {
							player.play().unwrap();
							debug!("Playing player {}...", player.bus_name());
						}
					}
					None => {
						player_status.insert(player.bus_name().to_string(), 0);
					}
				}
			}
		}
		"unplug" => {
			// Mute master, which also mutes everything else
			exec_cmds(vec![("amixer", &["-q", "set", "Master", "mute"])]);
			debug!("Muting...");

			// Loop through players, pause them and note which ones were playing
			for player in players {
				if player.get_playback_status().unwrap() == PlaybackStatus::Playing {
					*player_status.entry(player.bus_name().to_string()).or_insert(1) = 1;
				}
				else {
					*player_status.entry(player.bus_name().to_string()).or_insert(0) = 0;
				}
				player.pause().unwrap();
				debug!("Pausing player {}...", player.bus_name());
			}
		}
		_ => {}
	}
}

fn main() {
	env_logger::init();

	let mut player_status: HashMap<String, i32> = HashMap::new();

	// Spawn a child process with the acpi_listen command
	let stdout_acpi_listen = Command::new("acpi_listen")
		.stdout(Stdio::piped())
		.spawn()
		.unwrap()
		.stdout
		.unwrap();

	let reader = BufReader::new(stdout_acpi_listen);

	// Listen for new lines in stdout of the child process
	reader
		.lines()
		.filter_map(|line| line.ok())
		.for_each(|line| {
			match parse_event("HEADPHONE (plug|unplug)", &line) {
				Some(event_type) => {
					respond_to_event(&event_type, &mut player_status);
					debug!("Player status: {:?}", player_status);
				}
				None => {}
			}
		});
}
