use rbxcloud::rbx::{error::Error, types::UniverseId, v2::Client};

async fn restart_servers() -> Result<(), Error> {
    // Inputs:
    let api_key = "MY_API_KEY";
    let universe_id = 9876543210;

    let client = Client::new(api_key);
    let universe_client = client.universe(UniverseId(universe_id));

    universe_client.restart_servers().await
}

#[tokio::main]
async fn main() {
    let restart_result = restart_servers().await;

    match restart_result {
        Ok(()) => {
            println!("Servers restarted");
        }
        Err(e) => {
            eprintln!("{e:?}");
        }
    }
}
