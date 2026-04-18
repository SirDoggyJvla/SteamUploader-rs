use steamworks;
use colored::Colorize;

pub fn load_steam(appid: u32) -> (steamworks::Client, steamworks::UGC) {
    make_appidtxt(appid);

    // create a client pair
    let client = steamworks::Client::init().expect("Steam is not running or has not been detected");

    // get a handle to Steam's UGC module (user-generated content)
    let ugc = client.ugc();
    delete_appidtxt();


    (client, ugc)
}


/// Create the appid.txt file in the current directory with the specified appid. 
/// This is needed for the Steamworks API to function properly.
pub fn make_appidtxt(appid: u32) {
    match std::fs::write("steam_appid.txt", appid.to_string()) {
        Ok(_) => {},
        Err(e) => eprintln!("{}", format!("Failed to write steam_appid.txt file: {}", e).bright_red()),
    }
}

pub fn delete_appidtxt() {
    match std::fs::remove_file("steam_appid.txt") {
        Ok(_) => {},
        Err(e) => eprintln!("{}", format!("Failed to delete steam_appid.txt file: {}", e).bright_red()),
    }
}
