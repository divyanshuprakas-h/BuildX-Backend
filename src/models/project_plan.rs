use serde::Serialize;
use utoipa::ToSchema;

use crate::models::backend_plan::BackendPlanResponse;
use crate::models::blueprint::BlueprintResponse;
use crate::models::file_plan::FrontendPlanResponse;
use crate::models::intent::IntentResponse;

#[derive(Debug, Serialize, ToSchema)]
pub struct ProjectPlanResponse {
    pub intent: IntentResponse,
    pub blueprint: BlueprintResponse,
    pub frontend_plan: FrontendPlanResponse,
    pub backend_plan: BackendPlanResponse,
    pub summary: String,
}
