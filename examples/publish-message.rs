use rbxcloud::rbx::RbxCloud;

#[tokio::main]
async fn main() {
    // Inputs:
    let api_key = String::from("MY_API_KEY");
    let universe_id = 9876543210;
    let topic = "MyTopic";

    let message = String::from("Hello, this is my message");

    // Define RbxCloud Messaging instance:
    let cloud = RbxCloud::new(api_key, universe_id);
    let messaging = cloud.messaging(topic);

    // Publish a message:
    let message_result = messaging.publish(message).await;
    match message_result {
        Ok(()) => {
            println!("Message successfully published");
        }
        Err(e) => {
            eprintln!("{:?}", e);
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
