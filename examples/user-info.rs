use rbxcloud::rbx::{
    error::Error,
    types::RobloxUserId,
    v2::{user::GetUserResponse, Client},
};

async fn user_info() -> Result<GetUserResponse, Error> {
    // Inputs:
    let api_key = "MY_API_KEY";
    let user_id = 308165;

    let client = Client::new(api_key);
    let user_client = client.user();

    user_client.get_user(RobloxUserId(user_id)).await
}

#[tokio::main]
async fn main() {
    let user_info_res = user_info().await;

    match user_info_res {
        Ok(user_info) => {
            println!("{:?}", user_info);
        }
        Err(e) => {
            eprintln!("{e:?}");
        }
    }
}
