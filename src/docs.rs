use utoipa::OpenApi;

use crate::models::blueprint::{ApiRouteBlueprint, BlueprintResponse, PageBlueprint};
use crate::models::file_plan::{FilePlanItem, FrontendPlanResponse};
use crate::models::intent::{IntentRequest, IntentResponse};

use crate::routes::ai::{
    __path_detect_intent, __path_generate_blueprint, __path_generate_frontend_plan,
};
use crate::routes::health::__path_health_check;

#[derive(OpenApi)]
#[openapi(
    paths(
        health_check,
        detect_intent,
        generate_blueprint,
        generate_frontend_plan
    ),
    components(
        schemas(
            IntentRequest,
            IntentResponse,
            PageBlueprint,
            ApiRouteBlueprint,
            BlueprintResponse,
            FilePlanItem,
            FrontendPlanResponse
        )
    ),
    tags(
        (name = "Health", description = "Backend health check endpoints"),
        (name = "BuildX AI", description = "AI planning endpoints for BuildX")
    )
)]

pub struct ApiDoc;
