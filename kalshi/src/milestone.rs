//! milestone.rs – wrappers for Kalshi Trade API → milestone
use serde::Deserialize;
use serde_json::Value;
use crate::{Kalshi, kalshi_error::*};

/// We don’t yet have a public schema; store raw JSON.
pub type Milestone = Value;

/* -------- private envelopes -------- */
#[derive(Debug, Deserialize)]
struct MilestoneListResponse {
    cursor: Option<String>,
    milestones: Vec<Milestone>,
}
#[derive(Debug, Deserialize)]
struct SingleMilestoneResponse {
    milestone: Milestone,
}

impl<'a> Kalshi {
    /// GET `/milestones`
    pub async fn get_milestones(
        &self,
        limit: Option<i64>,
        cursor: Option<String>,
    ) -> Result<(Option<String>, Vec<Milestone>), KalshiError> {
        let mut p = Vec::new();
        add_param!(p, "limit",  limit);
        add_param!(p, "cursor", cursor);
        let path = if p.is_empty() { "/milestones".to_string() }
                   else { format!("/milestones?{}", serde_urlencoded::to_string(&p)?) };
        let res: MilestoneListResponse = self.signed_get(&path).await?;
        Ok((res.cursor, res.milestones))
    }

    /// GET `/milestones/{milestone_id}`
    pub async fn get_milestone(&self, milestone_id: &str) -> Result<Milestone, KalshiError> {
        let path = format!("/milestones/{milestone_id}");
        let res: SingleMilestoneResponse = self.signed_get(&path).await?;
        Ok(res.milestone)
    }
}
