//! Lemmy stuff

use std::{collections::HashMap, thread, time::Duration};

use anyhow::Result;
use lemmy_client::{
    lemmy_api_common::{
        community::GetCommunity, lemmy_db_schema::newtypes::CommunityId, person::Login,
        post::CreatePost,
    },
    ClientOptions, LemmyClient, LemmyRequest,
};

use crate::{
    cli::Args,
    common::{get_env_var, CommonPost, Platform},
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

/// Submit a post on Lemmy
pub async fn submit_post(args: &Args, post: CommonPost, li: &LemmyInfo) -> Result<()> {
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

    let post = CreatePost {
        community_id: li.community_id,
        name: title.clone(),
        body: Some(body),
        nsfw: Some(nsfw_flag),
        url,
        ..Default::default()
    };
    if args.dry_run {
        tracing::info!("{post:#?}");
    } else {
        loop {
            match li.me.create_post(post.clone()).await {
                Ok(post_info) => {
                    tracing::info!(
                        "Successfully posted on Lemmy!\n\
                                    Post's info: {post_info:#?}"
                    );
                    break;
                }
                Err(e) => {
                    tracing::error!(
                        "\
                                    Error when posting Reddit post with title: \"{title}\".\n\
                                    Error encountered: {e} \n\
                                    Retrying after {} seconds...",
                        args.retry_time
                    );
                    thread::sleep(Duration::from_secs(args.retry_time));
                }
            };
        }
    }

    Ok(())
}
