use super::Kalshi;
use crate::kalshi_error::*;
use serde::Deserialize;

pub use crate::generated::types::LiveData;

impl Kalshi {
    /// Retrieves live data for a specific milestone.
    ///
    /// This method provides real-time data feeds for a specific milestone
    /// without requiring a WebSocket connection.
    ///
    /// # Arguments
    ///
    /// * `data_type` - The type of live data to retrieve.
    /// * `milestone_id` - The milestone ID to get live data for.
    ///
    /// # Returns
    ///
    /// - `Ok(LiveData)`: The live data on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let live_data = kalshi_instance.get_live_data("some_type", "milestone-123").await.unwrap();
    /// ```
    ///
    pub async fn get_live_data(
        &self,
        data_type: &str,
        milestone_id: &str,
    ) -> Result<LiveData, KalshiError> {
        let path = format!("/live_data/{}/milestone/{}", data_type, milestone_id);
        self.signed_get(&path).await
    }

    /// Retrieves live data for multiple milestones at once.
    ///
    /// This method provides a batch endpoint for fetching real-time data
    /// for multiple milestones in a single request.
    ///
    /// # Arguments
    ///
    /// * `milestone_ids` - A vector of milestone IDs to get live data for.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<LiveData>)`: A vector of live data for the requested milestones on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let milestone_ids = vec!["milestone-1".to_string(), "milestone-2".to_string()];
    /// let live_data_batch = kalshi_instance.get_live_data_batch(milestone_ids).await.unwrap();
    /// ```
    ///
    /// Retrieves live data for a specific milestone (preferred endpoint).
    ///
    /// # Arguments
    /// * `milestone_id` - The milestone ID to get live data for.
    /// * `include_player_stats` - When true, includes player-level statistics (for supported sports).
    pub async fn get_live_data_by_milestone(
        &self,
        milestone_id: &str,
        include_player_stats: Option<bool>,
    ) -> Result<LiveData, KalshiError> {
        let mut path = format!("/live_data/milestone/{}", milestone_id);
        if let Some(true) = include_player_stats {
            path.push_str("?include_player_stats=true");
        }
        self.signed_get(&path).await
    }

    /// Retrieves game stats (play-by-play) for a specific milestone.
    ///
    /// # Arguments
    /// * `milestone_id` - The milestone ID to get game stats for.
    pub async fn get_game_stats(
        &self,
        milestone_id: &str,
    ) -> Result<serde_json::Value, KalshiError> {
        let path = format!("/live_data/milestone/{}/game_stats", milestone_id);
        self.signed_get(&path).await
    }

    pub async fn get_live_data_batch(
        &self,
        milestone_ids: Vec<String>,
    ) -> Result<Vec<LiveData>, KalshiError> {
        let path = "/live_data/batch";
        let mut params = vec![];
        
        // Add each milestone_id as a separate query parameter
        for id in milestone_ids {
            params.push(("milestone_ids".to_string(), id));
        }

        let url = format!("{}{}", self.base_url, path);
        let final_url = reqwest::Url::parse_with_params(&url, &params)?;
        let res: LiveDataBatchResponse = self.client.get(final_url).send().await?.json().await?;
        Ok(res.live_datas)
    }
}

// -------- Response wrappers --------

#[derive(Debug, Deserialize)]
struct LiveDataBatchResponse {
    live_datas: Vec<LiveData>,
}


