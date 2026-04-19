use steamworks;
use std::sync::mpsc;
use std::time::Duration;
use std::thread;

use crate::colors;

/// Create a workshop item and return its PublishedFileId
/// Returns Ok(published_id) on success, or an error message on failure
pub fn create_item(
    client: &steamworks::Client,
    ugc: &steamworks::UGC,
    appid: u32,
    dry_run: bool,
) -> Result<steamworks::PublishedFileId, String> {
    let (tx, rx) = mpsc::channel();

    // dry run
    // just print a message that we would create a new workshop item, without actually creating anything
    // and return a dummy PublishedFileId for the rest of the process to work
    if dry_run {
        colors::info("Dry run enabled. Using dummy Workshop ID and skipping actual item creation.");
        return Ok(steamworks::PublishedFileId(0)); // Return a dummy ID for dry run
    }

    // creating a new workshop item
    // make sure you change the appid to the specified game
    ugc.create_item(
        steamworks::AppId(appid),
        steamworks::FileType::Community,
        move |create_result| {
            // handle the result
            match create_result {
                Ok((published_id, needs_to_agree_to_terms)) => {
                    // if the user needs to agree to the terms of use, they will need to do that before you can upload any files
                    // in any case, make sure you save the published_id somewhere, like a manifest file.
                    // it is needed for all further calls
                    if needs_to_agree_to_terms {
                        let _ = tx.send(Err("You need to agree to the terms of use before you can upload any files".to_string()));
                    } else {
                        let _ = tx.send(Ok(published_id));
                    }
                }
                Err(e) => {
                    // an error occurred, usually because the app is not authorized to create items
                    // or the user is banned from the community
                    let _ = tx.send(Err(e.to_string()));
                }
            }
        },
    );

    // Wait for the result, pumping callbacks while we wait
    let deadline = std::time::Instant::now() + Duration::from_secs(30);
    loop {
        // Try to receive the result (non-blocking)
        match rx.try_recv() {
            Ok(result) => return result,
            Err(mpsc::TryRecvError::Empty) => {
                // No result yet, pump the client callbacks
                client.run_callbacks();
                
                // Check if we've exceeded the timeout
                if std::time::Instant::now() > deadline {
                    return Err("Timeout waiting for workshop item creation".to_string());
                }
                
                // Sleep briefly to avoid busy-waiting
                thread::sleep(Duration::from_millis(100));
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                return Err("Channel disconnected while waiting for creation result".to_string());
            }
        }
    }
}