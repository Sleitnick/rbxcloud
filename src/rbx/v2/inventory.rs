use serde::{Deserialize, Serialize};

use crate::rbx::{error::Error, types::RobloxUserId, util::QueryString};

use super::http_err::handle_http_err;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListInventoryItemsParams {
    pub api_key: String,
    pub user_id: RobloxUserId,
    pub max_page_size: Option<u32>,
    pub page_token: Option<String>,
    pub filter: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItems {
    inventory_items: Vec<InventoryItem>,
    next_page_token: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItem {
    path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    asset_details: Option<InventoryItemAssetDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    badge_details: Option<InventoryItemBadgeDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    game_pass_details: Option<InventoryItemGamePassDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_server_details: Option<InventoryItemPrivateServerDetails>,
    add_time: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItemAssetDetails {
    asset_id: String,
    instance_id: String,
    inventory_item_asset_type: InventoryItemAssetType,
    collectible_details: Option<InventoryItemCollectibleDetails>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItemBadgeDetails {
    badge_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItemGamePassDetails {
    game_pass_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItemPrivateServerDetails {
    private_server_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItemCollectibleDetails {
    item_id: String,
    instance_id: String,
    instance_state: InventoryItemInstanceState,
    serial_number: u64,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InventoryItemInstanceState {
    CollectibleItemInstanceStateUnspecified,
    Available,
    Hold,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InventoryItemAssetType {
    ClassicTshirt,
    Audio,
    Hat,
    Model,
    ClassicShirt,
    ClassicPants,
    Decal,
    ClassicHead,
    Face,
    Gear,
    Animation,
    Torso,
    RightArm,
    LeftArm,
    LeftLeg,
    RightLeg,
    Package,
    Plugin,
    MeshPart,
    HairAccessory,
    FaceAccessory,
    NeckAccessory,
    ShoulderAccessory,
    FrontAccessory,
    BackAccessory,
    WaistAccessory,
    ClimbAnimation,
    DeathAnimation,
    FallAnimation,
    IdleAnimation,
    JumpAnimation,
    RunAnimation,
    SwimAnimation,
    WalkAnimation,
    PoseAnimation,
    EmoteAnimation,
    Video,
    TshirtAccessory,
    ShirtAccessory,
    PantsAccessory,
    JacketAccessory,
    SweaterAccessory,
    ShortsAccessory,
    LeftShoeAccessory,
    RightShoeAccessory,
    DressSkirtAccessory,
    EyebrowAccessory,
    EyelashAccessory,
    MoodAnimation,
    DynamicHead,
    CreatedPlace,
    PurchasedPlace,
}

pub async fn list_inventory_items(
    params: &ListInventoryItemsParams,
) -> Result<InventoryItems, Error> {
    let client = reqwest::Client::new();

    let url = format!(
        "https://apis.roblox.com/cloud/v2/users/{userId}/inventory-items",
        userId = params.user_id,
    );

    let mut query: QueryString = vec![];
    if let Some(max_page_size) = params.max_page_size {
        query.push(("maxPageSize", format!("{max_page_size}")));
    }
    if let Some(page_token) = &params.page_token {
        query.push(("pageToken", page_token.clone()));
    }
    if let Some(filter) = &params.filter {
        query.push(("filter", filter.clone()));
    }

    let res = client
        .get(url)
        .header("x-api-key", &params.api_key)
        .query(&query)
        .send()
        .await?;

    let status = res.status();

    if !status.is_success() {
        let code = status.as_u16();
        return handle_http_err(code);
    }

    let body = res.json::<InventoryItems>().await?;
    Ok(body)
}
