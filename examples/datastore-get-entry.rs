use rbxcloud::rbx::{DataStoreGetEntry, RbxCloud, UniverseId};

#[tokio::main]
async fn main() {
    // Inputs:
    let api_key = "MY_API_KEY";
    let universe_id = 9876543210;
    let datastore_name = String::from("my_datastore");
    let key = String::from("my_key");

    // Define RbxCloud DataStore instance:
    let cloud = RbxCloud::new(api_key);
    let datastore = cloud.datastore(UniverseId(universe_id));

    // Get entry:
    let entry_result = datastore
        .get_entry_string(&DataStoreGetEntry {
            name: datastore_name,
            scope: None,
            key,
        })
        .await;

    // Print entry result or error:
    match entry_result {
        Ok(result) => {
            println!("{result}");
        }
        Err(e) => {
            eprintln!("{e:?}");
        }
    }
}
