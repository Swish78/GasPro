use std::process::Command;
use std::error::Error;

/// Run an AppleScript command
fn run_applescript(script: &str) -> Result<(), Box<dyn Error>> {
    Command::new("osascript")
        .arg("-e")
        .arg(script)
        .status()?;
    Ok(())
}

/// Play a specific Apple Music playlist
pub fn play_playlist(playlist: &str) -> Result<(), Box<dyn Error>> {
    let script = format!(
        r#"tell application "Music"
            if not running then
                activate
                delay 1
            end if

            try
                if exists playlist "{}" then
                    play playlist "{}"
                else
                    display dialog "Playlist '{}' does not exist." buttons {{"OK"}} default button "OK"
                end if
            on error errMsg
                display dialog errMsg buttons {{"OK"}} default button "OK"
            end try
        end tell"#,
        playlist, playlist, playlist
    );
    run_applescript(&script)
}

/// Pause the currently playing track
pub fn pause_music() -> Result<(), Box<dyn Error>> {
    let script = r#"
        tell application "Music"
            if player state is playing then
                pause
            end if
        end tell"#;
    run_applescript(script)
}

/// Skip to the next track
pub fn next_track() -> Result<(), Box<dyn Error>> {
    let script = r#"
        tell application "Music"
            if player state is playing then
                next track
            end if
        end tell"#;
    run_applescript(script)
}

/// Play the previous track
pub fn previous_track() -> Result<(), Box<dyn Error>> {
    let script = r#"
        tell application "Music"
            if player state is playing then
                previous track
            end if
        end tell"#;
    run_applescript(script)
}

/// Get the current track information (artist - song title)
pub fn current_track() -> Result<String, Box<dyn Error>> {
    let script = r#"
        tell application "Music"
            try
                if player state is playing then
                    set trackName to name of current track
                    set artistName to artist of current track
                    return artistName & " - " & trackName
                else
                    return "No track is currently playing"
                end if
            on error
                return "Unable to get track information"
            end try
        end tell"#;
    run_applescript_with_output(script)
}

/// Helper function to run AppleScript commands with output
fn run_applescript_with_output(script: &str) -> Result<String, Box<dyn Error>> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()?;

    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

// function to list all playlists (helpful for debugging)
pub fn list_playlists() -> Result<(), Box<dyn Error>> {
    let script = r#"
        tell application "Music"
            if not running then
                activate
                delay 1
            end if

            set output to ""
            repeat with p in user playlists
                try
                    set pname to (get name of p)
                    set pcount to (get count of tracks of p)
                    set output to output & pname & " (" & pcount & " tracks)" & return
                on error
                    continue
                end try
            end repeat
            return output
        end tell"#;

    let result = run_applescript_with_output(script)?;
    if result.is_empty() {
        println!("No playlists found or unable to access playlists.");
    } else {
        println!("Available playlists:\n{}", result);
    }
    Ok(())
}

pub fn list_detailed_playlists() -> Result<(), Box<dyn Error>> {
    let script = r#"
        tell application "Music"
            if not running then
                activate
                delay 1
            end if

            set output to ""
            repeat with p in user playlists
                try
                    set pname to (get name of p)
                    set pcount to (get count of tracks of p)
                    set pduration to (get duration of p)
                    set ptime to my format_time(pduration)
                    set output to output & "Playlist: " & pname & return
                    set output to output & "Tracks: " & pcount & return
                    set output to output & "Duration: " & ptime & return
                    set output to output & "----------------------------------------" & return
                on error errMsg
                    continue
                end try
            end repeat
            return output
        end tell

        on format_time(timeInSeconds)
            set hours to timeInSeconds div (60 * 60)
            set minutes to (timeInSeconds mod (60 * 60)) div 60
            if hours > 0 then
                return hours & " hours, " & minutes & " minutes"
            else
                return minutes & " minutes"
            end if
        end format_time"#;

    let result = run_applescript_with_output(script)?;
    if result.is_empty() {
        println!("No playlists found or unable to access playlists.");
    } else {
        println!("Available playlists:\n{}", result);
    }
    Ok(())
}

pub fn list_all_playlists() -> Result<(), Box<dyn Error>> {
    let script = r#"
        tell application "Music"
            if not running then
                activate
                delay 1
            end if

            set output to ""
            repeat with p in user playlists
                try
                    set pname to (get name of p)
                    set pcount to (get count of tracks of p)
                    set output to output & pname & " (" & pcount & " tracks)" & return
                on error
                    -- Skip any playlist that causes an error
                    continue
                end try
            end repeat
            return output
        end tell"#;

    let result = run_applescript_with_output(script)?;
    if result.is_empty() {
        println!("No playlists found or unable to access playlists.");
    } else {
        println!("Available playlists:\n{}", result);
    }
    Ok(())
}