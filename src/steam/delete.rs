use steamworks;

pub fn delete_item(ugc: &steamworks::UGC, published_id: steamworks::PublishedFileId) {
    // deleting an item
    ugc.delete_item(published_id, move |delete_result| {
        match delete_result {
            Ok(()) => {
                // item has been deleted
                println!("Deleted item with id {:?}", published_id);
            }
            Err(e) => {
                // the item could not be deleted
                // usually because it is not owned by the user or it doesn't exist in the first place
                println!("Error deleting item: {:?}", e);
            }
        }
    })
}
