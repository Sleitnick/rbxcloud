use std::collections::HashMap;

use rbxcloud::rbx::{
    error::Error,
    types::{RobloxUserId, UniverseId},
    v2::{
        notification::{
            JoinExperience, Notification, NotificationPayload, NotificationResponse,
            NotificationSource, NotificationType, Parameter,
        },
        Client,
    },
};

async fn send_notification() -> Result<NotificationResponse, Error> {
    // Inputs:
    let api_key = "MY_API_KEY";
    let message_id = "MY_MESSAGE_ID";
    let universe_id = 9876543210;
    let user_id = 308165;

    let client = Client::new(api_key);
    let notification_client = client.notification(UniverseId(universe_id));

    let notification = Notification {
        source: NotificationSource {
            universe: format!("universe/{}", universe_id),
        },
        payload: NotificationPayload {
            message_id: message_id.to_string(),
            notification_type: NotificationType::TypeUnspecified,
            join_experience: Some(JoinExperience {
                launch_data: "Some launch data here".to_string(),
            }),
            analytics_data: Some(HashMap::from([(
                "category".to_string(),
                "Bronze egg hatched".to_string(),
            )])),
            parameters: Some(HashMap::from([(
                "key".to_string(),
                Parameter {
                    string_value: Some("bronze egg".to_string()),
                    int64_value: None,
                },
            )])),
        },
    };

    notification_client
        .send(RobloxUserId(user_id), notification)
        .await
}

#[tokio::main]
async fn main() {
    let send_result = send_notification().await;

    match send_result {
        Ok(result) => {
            println!("Notification sent: {:?}", result);
        }
        Err(e) => {
            eprintln!("{e:?}");
        }
    }
}
