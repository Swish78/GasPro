mod commands;
use clap::{Arg, Command};
use commands::music;

fn main() {
    let matches = Command::new("DevCLI")
        .version("1.0")
        .author("Swayam Patil")
        .about("Developer CLI Tool for macOS")
        .subcommand(
            Command::new("music")
                .about("Control Apple Music")
                .subcommand(Command::new("play").arg(Arg::new("playlist").required(true)))
                .subcommand(Command::new("pause"))
                .subcommand(Command::new("next"))
                .subcommand(Command::new("previous"))
                .subcommand(Command::new("current"))
                .subcommand(Command::new("list").about("List all playlists"))
                .subcommand(Command::new("list-detailed").about("List all playlists with details")),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("music") {
        match matches.subcommand() {
            Some(("play", sub_matches)) => {
                let playlist = sub_matches.get_one::<String>("playlist").unwrap();
                if let Err(e) = music::play_playlist(playlist) {
                    eprintln!("Error playing playlist: {}", e);
                }
            }
            Some(("pause", _)) => {  // Fixed asterisk to underscore
                if let Err(e) = music::pause_music() {  // Fixed function name
                    eprintln!("Error pausing music: {}", e);
                }
            }
            Some(("next", _)) => {
                if let Err(e) = music::next_track() {  // Fixed function name
                    eprintln!("Error skipping track: {}", e);
                }
            }
            Some(("previous", _)) => {
                if let Err(e) = music::previous_track() {  // Fixed function name
                    eprintln!("Error playing previous track: {}", e);
                }
            }
            Some(("current", _)) => {
                match music::current_track() {  // Fixed function name
                    Ok(info) => println!("Currently Playing: {}", info),
                    Err(e) => eprintln!("Error getting current track: {}", e),
                }
            }
            Some(("list", _)) => {
                if let Err(e) = music::list_playlists() {
                    eprintln!("Error listing playlists: {}", e);
                }
            }
            Some(("list-detailed", _)) => {
                if let Err(e) = music::list_detailed_playlists() {
                    eprintln!("Error listing playlists: {}", e);
                }
            }
            _ => println!("Use `devcli music --help` for more options"),
        }
    }
}