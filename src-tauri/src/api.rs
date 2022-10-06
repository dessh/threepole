use std::error::Error;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    hash::Hash,
};

use async_trait::async_trait;
use tokio::sync::Mutex;

use self::{
    requests::{make_request, BungieRequest, BungieResponseError},
    responses::{
        ActivityInfo, BungieProfile, CharacterActivityHistory, ProfileCurrentActivities,
        ProfileInfo,
    },
};
use crate::config::profiles::Profile;

pub mod requests;
pub mod responses;

#[derive(Debug)]
pub enum ApiError {
    ResponseDeserializeError(serde_json::Error),
    ResponseError(BungieResponseError),
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::ResponseDeserializeError(e) => {
                write!(f, "Failed to parse response object: {}", e)
            }
            ApiError::ResponseError(e) => e.fmt(f),
        }
    }
}

impl Error for ApiError {}

#[async_trait]
pub trait Source<K: Hash + Eq + Clone + Send, V: Clone + Send> {
    async fn get(&mut self, key: K) -> Result<V, ApiError>
    where
        K: 'async_trait,
    {
        let cache = self.cache();

        if cache.contains_key(&key) {
            return Ok(cache.get(&key).unwrap().clone());
        }

        let value_fut = Self::get_value(key.clone());

        let value = value_fut.await?;

        cache.insert(key, value.clone());

        Ok(value)
    }

    async fn get_value(key: K) -> Result<V, ApiError>;

    fn cache(&mut self) -> &mut HashMap<K, V>;
}

#[derive(Default)]
pub struct ProfileInfoSource {
    cache: HashMap<Profile, ProfileInfo>,
}

impl ProfileInfoSource {
    pub fn set_characters(&mut self, profile: &Profile, characters: Vec<String>) {
        if let Some(p) = self.cache.get_mut(profile) {
            p.character_ids = characters;
        }
    }
}

#[async_trait]
impl Source<Profile, ProfileInfo> for ProfileInfoSource {
    async fn get_value(profile: Profile) -> Result<ProfileInfo, ApiError> {
        let res_val = make_request(BungieRequest::GetProfile {
            membership_type: profile.account_platform,
            membership_id: &profile.account_id,
            component: 100,
        })
        .await
        .map_err(|e| ApiError::ResponseError(e))?;

        serde_json::from_value(res_val).map_err(|e| ApiError::ResponseDeserializeError(e))
    }

    fn cache(&mut self) -> &mut HashMap<Profile, ProfileInfo> {
        &mut self.cache
    }
}

#[derive(Default)]
pub struct ActivityInfoSource {
    cache: HashMap<usize, ActivityInfo>,
}

#[async_trait]
impl Source<usize, ActivityInfo> for ActivityInfoSource {
    async fn get_value(activity_hash: usize) -> Result<ActivityInfo, ApiError> {
        let res_val = make_request(BungieRequest::GetDestinyActivityDefinition { activity_hash })
            .await
            .map_err(|e| ApiError::ResponseError(e))?;

        serde_json::from_value(res_val).map_err(|e| ApiError::ResponseDeserializeError(e))
    }

    fn cache(&mut self) -> &mut HashMap<usize, ActivityInfo> {
        &mut self.cache
    }
}

#[derive(Default)]
pub struct Api {
    pub profile_info_source: Mutex<ProfileInfoSource>,
    pub activity_info_source: Mutex<ActivityInfoSource>,
}

impl Api {
    pub async fn search_profile(
        display_name: &String,
        display_name_code: usize,
    ) -> Result<Vec<BungieProfile>, ApiError> {
        let res_val = make_request(BungieRequest::SearchDestinyPlayerByBungieName {
            display_name: display_name,
            display_name_code,
        })
        .await
        .map_err(|e| ApiError::ResponseError(e))?;

        serde_json::from_value(res_val).map_err(|e| ApiError::ResponseDeserializeError(e))
    }

    pub async fn get_profile_activities(
        profile: &Profile,
    ) -> Result<ProfileCurrentActivities, ApiError> {
        let res_val = make_request(BungieRequest::GetProfile {
            membership_type: profile.account_platform,
            membership_id: &profile.account_id,
            component: 204,
        })
        .await
        .map_err(|e| ApiError::ResponseError(e))?;

        serde_json::from_value(res_val).map_err(|e| ApiError::ResponseDeserializeError(e))
    }

    pub async fn get_activity_history(
        profile: &Profile,
        character_id: &String,
        page: usize,
    ) -> Result<CharacterActivityHistory, ApiError> {
        let res_val = make_request(BungieRequest::GetActivityHistory {
            membership_type: profile.account_platform,
            membership_id: &profile.account_id,
            character_id: character_id,
            page,
        })
        .await
        .map_err(|e| ApiError::ResponseError(e))?;

        serde_json::from_value(res_val).map_err(|e| ApiError::ResponseDeserializeError(e))
    }
}
