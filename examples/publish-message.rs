use rbxcloud::rbx::{error::Error, types::UniverseId, v1::RbxCloud};

async fn publish_message() -> Result<(), Error> {
    // Inputs:
    let api_key = "MY_API_KEY";
    let universe_id = 9876543210;
    let topic = "MyTopic";

    let message = "Hello, this is my message";

    // Define RbxCloud Messaging instance:
    let cloud = RbxCloud::new(api_key);
    let messaging = cloud.messaging(UniverseId(universe_id), topic);

    // Publish a message:
    messaging.publish(message).await
}

#[tokio::main]
async fn main() {
    // Publish a message:
    let message_result = publish_message().await;
    match message_result {
        Ok(()) => {
            println!("Message successfully published");
        }
        Err(e) => {
            eprintln!("{e:?}");
        }
    }

    /*
    From a Lua script within a Roblox experience:

    local MessagingService = game:GetService("MessagingService")
    MessagingService:SubscribeAsync("MyTopic"):Connect(function(message)
        print(message)
        --> {"message": "Hello, this is my message"}
    end)
     */
}
