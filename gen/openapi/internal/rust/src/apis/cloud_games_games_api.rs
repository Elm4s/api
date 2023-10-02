/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`cloud_games_games_create_game`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloudGamesGamesCreateGameError {
    Status400(crate::models::ErrorBody),
    Status403(crate::models::ErrorBody),
    Status404(crate::models::ErrorBody),
    Status408(crate::models::ErrorBody),
    Status429(crate::models::ErrorBody),
    Status500(crate::models::ErrorBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cloud_games_games_game_banner_upload_complete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloudGamesGamesGameBannerUploadCompleteError {
    Status400(crate::models::ErrorBody),
    Status403(crate::models::ErrorBody),
    Status404(crate::models::ErrorBody),
    Status408(crate::models::ErrorBody),
    Status429(crate::models::ErrorBody),
    Status500(crate::models::ErrorBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cloud_games_games_game_banner_upload_prepare`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloudGamesGamesGameBannerUploadPrepareError {
    Status400(crate::models::ErrorBody),
    Status403(crate::models::ErrorBody),
    Status404(crate::models::ErrorBody),
    Status408(crate::models::ErrorBody),
    Status429(crate::models::ErrorBody),
    Status500(crate::models::ErrorBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cloud_games_games_game_logo_upload_complete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloudGamesGamesGameLogoUploadCompleteError {
    Status400(crate::models::ErrorBody),
    Status403(crate::models::ErrorBody),
    Status404(crate::models::ErrorBody),
    Status408(crate::models::ErrorBody),
    Status429(crate::models::ErrorBody),
    Status500(crate::models::ErrorBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cloud_games_games_game_logo_upload_prepare`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloudGamesGamesGameLogoUploadPrepareError {
    Status400(crate::models::ErrorBody),
    Status403(crate::models::ErrorBody),
    Status404(crate::models::ErrorBody),
    Status408(crate::models::ErrorBody),
    Status429(crate::models::ErrorBody),
    Status500(crate::models::ErrorBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cloud_games_games_get_game_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloudGamesGamesGetGameByIdError {
    Status400(crate::models::ErrorBody),
    Status403(crate::models::ErrorBody),
    Status404(crate::models::ErrorBody),
    Status408(crate::models::ErrorBody),
    Status429(crate::models::ErrorBody),
    Status500(crate::models::ErrorBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cloud_games_games_get_games`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloudGamesGamesGetGamesError {
    Status400(crate::models::ErrorBody),
    Status403(crate::models::ErrorBody),
    Status404(crate::models::ErrorBody),
    Status408(crate::models::ErrorBody),
    Status429(crate::models::ErrorBody),
    Status500(crate::models::ErrorBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cloud_games_games_validate_game`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloudGamesGamesValidateGameError {
    Status400(crate::models::ErrorBody),
    Status403(crate::models::ErrorBody),
    Status404(crate::models::ErrorBody),
    Status408(crate::models::ErrorBody),
    Status429(crate::models::ErrorBody),
    Status500(crate::models::ErrorBody),
    UnknownValue(serde_json::Value),
}


/// Creates a new game.
pub async fn cloud_games_games_create_game(configuration: &configuration::Configuration, cloud_games_create_game_request: crate::models::CloudGamesCreateGameRequest) -> Result<crate::models::CloudGamesCreateGameResponse, Error<CloudGamesGamesCreateGameError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cloud/games", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&cloud_games_create_game_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CloudGamesGamesCreateGameError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Completes an game banner image upload. Must be called after the file upload process completes.
pub async fn cloud_games_games_game_banner_upload_complete(configuration: &configuration::Configuration, game_id: &str, upload_id: &str) -> Result<(), Error<CloudGamesGamesGameBannerUploadCompleteError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cloud/games/{game_id}/banner-upload/{upload_id}/complete", local_var_configuration.base_path, game_id=crate::apis::urlencode(game_id), upload_id=crate::apis::urlencode(upload_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<CloudGamesGamesGameBannerUploadCompleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Prepares a game banner image upload.
pub async fn cloud_games_games_game_banner_upload_prepare(configuration: &configuration::Configuration, game_id: &str, cloud_games_game_banner_upload_prepare_request: crate::models::CloudGamesGameBannerUploadPrepareRequest) -> Result<crate::models::CloudGamesGameBannerUploadPrepareResponse, Error<CloudGamesGamesGameBannerUploadPrepareError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cloud/games/{game_id}/banner-upload/prepare", local_var_configuration.base_path, game_id=crate::apis::urlencode(game_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&cloud_games_game_banner_upload_prepare_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CloudGamesGamesGameBannerUploadPrepareError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Completes a game logo image upload. Must be called after the file upload process completes.
pub async fn cloud_games_games_game_logo_upload_complete(configuration: &configuration::Configuration, game_id: &str, upload_id: &str) -> Result<(), Error<CloudGamesGamesGameLogoUploadCompleteError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cloud/games/{game_id}/logo-upload/{upload_id}/complete", local_var_configuration.base_path, game_id=crate::apis::urlencode(game_id), upload_id=crate::apis::urlencode(upload_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<CloudGamesGamesGameLogoUploadCompleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Prepares a game logo image upload.
pub async fn cloud_games_games_game_logo_upload_prepare(configuration: &configuration::Configuration, game_id: &str, cloud_games_game_logo_upload_prepare_request: crate::models::CloudGamesGameLogoUploadPrepareRequest) -> Result<crate::models::CloudGamesGameLogoUploadPrepareResponse, Error<CloudGamesGamesGameLogoUploadPrepareError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cloud/games/{game_id}/logo-upload/prepare", local_var_configuration.base_path, game_id=crate::apis::urlencode(game_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&cloud_games_game_logo_upload_prepare_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CloudGamesGamesGameLogoUploadPrepareError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a game by its game id.
pub async fn cloud_games_games_get_game_by_id(configuration: &configuration::Configuration, game_id: &str, watch_index: Option<&str>) -> Result<crate::models::CloudGamesGetGameByIdResponse, Error<CloudGamesGamesGetGameByIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cloud/games/{game_id}", local_var_configuration.base_path, game_id=crate::apis::urlencode(game_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = watch_index {
        local_var_req_builder = local_var_req_builder.query(&[("watch_index", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CloudGamesGamesGetGameByIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a list of games in which the current identity is a group member of its development team.
pub async fn cloud_games_games_get_games(configuration: &configuration::Configuration, watch_index: Option<&str>) -> Result<crate::models::CloudGamesGetGamesResponse, Error<CloudGamesGamesGetGamesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cloud/games", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = watch_index {
        local_var_req_builder = local_var_req_builder.query(&[("watch_index", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CloudGamesGamesGetGamesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Validates information used to create a new game.
pub async fn cloud_games_games_validate_game(configuration: &configuration::Configuration, cloud_games_validate_game_request: crate::models::CloudGamesValidateGameRequest) -> Result<crate::models::CloudGamesValidateGameResponse, Error<CloudGamesGamesValidateGameError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cloud/games/validate", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&cloud_games_validate_game_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CloudGamesGamesValidateGameError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

