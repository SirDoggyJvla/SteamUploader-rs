use std;

use crate::colors;

fn visibility2enum(visibility: u32) -> Result<steamworks::PublishedFileVisibility, String> {
    match visibility {
        0 => Ok(steamworks::PublishedFileVisibility::Public),
        1 => Ok(steamworks::PublishedFileVisibility::FriendsOnly),
        2 => Ok(steamworks::PublishedFileVisibility::Private),
        3 => Ok(steamworks::PublishedFileVisibility::Unlisted),
        _ => {
            Err(format!("Invalid visibility value: {}. Must be 0 (Public), 1 (Friends Only), 2 (Private), or 3 (Unlisted)", visibility))
        }
    }
}


/// Uploads a workshop item with the given parameters
pub fn upload_item_content(
    ugc: &steamworks::UGC, appid: u32,
    published_id: steamworks::PublishedFileId,
    content: &str, preview: &str,
    title: &str, description: &str,
    visibility: u32, patchnote: Option<&str>,
    dry_run: bool,
) {
    // Validate visibility before proceeding
    let visibility_enum = match visibility2enum(visibility) {
        Ok(vis) => vis,
        Err(e) => {
            colors::error(&format!("Error: {}", e));
            return;
        }
    };

    // dry run
    // just print the values that would be used for the upload, without actually uploading anything
    if dry_run {
        colors::info("Dry run enabled. The following item would be uploaded:");
        colors::info(&format!("Title: {}", title));
        colors::info(&format!("Description: {}", description));
        colors::info(&format!("Content path: {}", content));
        colors::info(&format!("Preview path: {}", preview));
        colors::info(&format!("Visibility: {:?}", visibility_enum));
        if let Some(patchnote) = patchnote {
            colors::info(&format!("Patchnote: {}", patchnote));
        }
        return;
    }

    // uploading the content of the workshop item
    // this process uses a builder pattern to set properties of the item
    // mandatory properties are:
    // - title
    // - description
    // - preview_path
    // - content_path
    // - visibility
    // after setting the properties, call .submit() to start uploading the item
    // this function is unique in that it returns a handle to the upload, which can be used to
    // monitor the progress of the upload and needs a closure to be called when the upload is done
    // in this example, the watch handle is ignored for simplicity
    //
    // notes:
    // - once an upload is started, it cannot be cancelled!
    // - content_path is the path to a folder which houses the content you wish to upload
    let _upload_handle = ugc
        .start_item_update(steamworks::AppId(appid), published_id)
        .content_path(std::path::Path::new(content))
        .preview_path(std::path::Path::new(preview))
        .title(title)
        .description(description)
        .tags(Vec::<String>::new(), false)
        .visibility(visibility_enum)
        .submit(patchnote, |upload_result| {
            // handle the result
            match upload_result {
                Ok((published_id, needs_to_agree_to_terms)) => {
                    if needs_to_agree_to_terms {
                        // as stated in the create_item function, if the user needs to agree to the terms of use,
                        // the upload did NOT succeed, despite the result being Ok
                        colors::error(
                            "You need to agree to the terms of use before you can upload any files"
                        );
                    } else {
                        // this is the definite indicator that an item was uploaded successfully
                        // the watch handle is NOT an accurate indicator whether the upload is done
                        // the progress on the other hand IS accurate and can simply be used to monitor the upload
                        colors::success(&format!("Uploaded item with id {:?}", published_id));
                    }
                }
                Err(e) => {
                    // the upload failed
                    // the exact reason can be found in the error type
                    colors::error(&format!("Error uploading item: {:?}", e));
                }
            }
        });
}