use rbxcloud::rbx::{
    error::Error,
    v2::{group::GroupId, RbxCloud},
};

async fn get_group_shout() -> Result<String, Error> {
    // Inputs:
    let api_key = "MY_API_KEY";
    let group_id = 9876543210;

    let cloud = RbxCloud::new(api_key);
    let group = cloud.group(GroupId(group_id));

    // Get the shout's content:
    group.get_shout().await.map(|r| r.content)
}

#[tokio::main]
async fn main() {
    let shout_res = get_group_shout().await;
    match shout_res {
        Ok(shout) => println!("{shout}"),
        Err(e) => eprintln!("{e:?}"),
    }
}
