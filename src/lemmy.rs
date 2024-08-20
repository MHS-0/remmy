//! Lemmy stuff

use std::{collections::HashMap, thread, time::Duration};

use anyhow::Result;
use lemmy_client::{
    lemmy_api_common::{
        community::GetCommunity,
        lemmy_db_schema::newtypes::CommunityId,
        person::Login,
        post::{CreatePost, PostResponse},
        LemmyErrorType,
    },
    ClientOptions, LemmyClient, LemmyRequest,
};

use crate::{
    cli::Args,
    common::{get_env_var, CommonPost, Platform},
    config::Config,
};

/// Struct containing info needed to do operations
/// with Lemmy
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
pub async fn initialize_lemmy_client(config: Config) -> LemmyInfo {
    tracing::info!("Setting up and validating the Lemmy client...");

    // TODO: We could do this by using a serde serialized yaml file too.
    // Get Lemmy related env variables
    let lemmy_instance = config.lemmy.instance;
    let lemmy_username = config.lemmy.username;
    let lemmy_password = config.lemmy.password;
    let lemmy_community_name = config.lemmy.community;

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

/// Converts a CommonPost struct to a CreatePost Lemmy
/// request
pub fn convert_to_lemmy_post(post: CommonPost, li: &LemmyInfo) -> CreatePost {
    let title = post.title;
    let author = post.author;
    let platform = match post.platform {
        Platform::Reddit => "Reddit",
        Platform::Lemmy => "Lemmy",
    };
    let body = post.body
        + &format!(
            "\n\n\
                        This post was authored by: {author} on {platform}\n\n\
                        If you liked this post, give them a visit!"
        );
    let nsfw_flag = post.nsfw;
    let url = post.url;

    CreatePost {
        community_id: li.community_id,
        name: title.clone(),
        body: Some(body),
        nsfw: Some(nsfw_flag),
        url,
        ..Default::default()
    }
}

/// Submit a post on Lemmy
///
/// Returns the result of the attempt at submitting
/// the post
pub async fn submit_post(post: CreatePost, li: &LemmyInfo) -> Result<PostResponse, LemmyErrorType> {
    li.me.create_post(post.clone()).await
}
