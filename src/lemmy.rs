//! Lemmy stuff

use std::collections::HashMap;

use lemmy_client::{
    lemmy_api_common::{
        community::GetCommunity, lemmy_db_schema::newtypes::CommunityId, person::Login,
    },
    ClientOptions, LemmyClient, LemmyRequest,
};

use crate::common::get_env_var;

pub struct LemmyInfo {
    pub me: LemmyClient,
    pub community_name: String,
    pub community_id: CommunityId,
}

impl LemmyInfo {
    pub fn new(me: LemmyClient, community: impl Into<String>, community_id: CommunityId) -> Self {
        Self {
            me,
            community_name: community.into(),
            community_id,
        }
    }
}

/// Get the required veraibles from the environment
/// and setup a usable lemmy client.
///
/// Panics if the variables are not given or correct,
/// or if there are connection issues.
pub async fn initialize_lemmy_client() -> LemmyInfo {
    tracing::info!("Setting up and validating the Lemmy client...");

    // TODO: We could do this by using a serde serialized yaml file too.
    // Get Lemmy related env variables
    let lemmy_instance = get_env_var("LEMMY_INSTANCE");
    let lemmy_username = get_env_var("LEMMY_USERNAME");
    let lemmy_password = get_env_var("LEMMY_PASSWORD");
    let lemmy_community_name = get_env_var("LEMMY_COMMUNITY");

    // Setup the Lemmy client
    let mut lemmy_client = LemmyClient::new(ClientOptions {
        domain: lemmy_instance,
        secure: true,
    });

    let response = lemmy_client.get_site(()).await;

    if let Err(e) = response {
        tracing::error!("Error instantiating Lemmy client: {e}");
        std::process::exit(1);
    }

    let login_info = Login {
        username_or_email: lemmy_username.into(),
        password: lemmy_password.into(),
        totp_2fa_token: None,
    };

    let login_response = lemmy_client
        .login(login_info)
        .await
        .expect("Couldn't login to Lemmy with provided credentials");
    let jwt = login_response.jwt.expect("Error getting jwt after login");
    lemmy_client.headers_mut().insert(
        "Authorization".into(),
        format!("Bearer {}", jwt.to_string()),
    );

    let lemmy_community_data = lemmy_client
        .get_community(GetCommunity {
            name: Some(lemmy_community_name.clone()),
            ..Default::default()
        })
        .await
        .expect("Couldn't fetch Lemmy community info");
    let lemmy_community_id = lemmy_community_data.community_view.community.id;

    LemmyInfo::new(lemmy_client, lemmy_community_name, lemmy_community_id)
}
