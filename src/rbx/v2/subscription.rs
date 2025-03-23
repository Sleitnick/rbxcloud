use serde::{Deserialize, Serialize};

use crate::rbx::{error::Error, types::UniverseId, util::QueryString};

use super::http_err::handle_http_err;

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum SubscriptionView {
    Basic,
    Full,
}

impl std::fmt::Display for SubscriptionView {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{:?}",
            match self {
                Self::Basic => "BASIC",
                Self::Full => "FULL",
            }
        )
    }
}

pub struct GetSubscriptionParams {
    pub api_key: String,
    pub universe_id: UniverseId,
    pub subscription_product: String,
    pub subscription: String,
    pub view: Option<SubscriptionView>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetSubscriptionResponse {
    path: String,
    create_time: String,
    update_time: String,
    active: bool,
    will_renew: bool,
    last_billing_time: String,
    next_renew_time: String,
    expire_time: String,
    state: SubscriptionState,
    expiration_details: SubscriptionExpirationDetails,
    purchase_platform: SubscriptionPurchasePlatform,
    payment_provider: SubscriptionPaymentProvider,
    user: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionExpirationDetails {
    reason: SubscriptionExpirationReason,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubscriptionState {
    StateUnspecified,
    SubscribedWillRenew,
    SubscribedWillNotRenew,
    SubscribedRenewalPaymentPending,
    Expired,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubscriptionExpirationReason {
    ExpirationReasonUnspecified,
    ProductInactive,
    ProductDeleted,
    SubscriberCancelled,
    SubscriberRefunded,
    Lapsed,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubscriptionPurchasePlatform {
    PurchasePlatformUnspecified,
    Desktop,
    Mobile,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubscriptionPaymentProvider {
    PaymentProviderUnspecified,
    Stripe,
    Apple,
    Google,
    RobloxCredit,
}

pub async fn get_subscription(
    params: &GetSubscriptionParams,
) -> Result<GetSubscriptionResponse, Error> {
    let client = reqwest::Client::new();

    let url = format!(
        "https://apis.roblox.com/cloud/v2/universes/{universeId}/subscription-products/{subscription}",
        universeId = &params.universe_id,
		subscription = &params.subscription_product,
    );

    let mut query: QueryString = vec![];
    if let Some(view) = &params.view {
        query.push(("view", view.to_string()));
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

    let body = res.json::<GetSubscriptionResponse>().await?;
    Ok(body)
}
