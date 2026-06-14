use utoipa::OpenApi;

use crate::models::backend_plan::{BackendPlanItem, BackendPlanResponse};
use crate::models::blueprint::{ApiRouteBlueprint, BlueprintResponse, PageBlueprint};
use crate::models::file_plan::{FilePlanItem, FrontendPlanResponse};
use crate::models::generated_code::{CodePreviewResponse, GeneratedFile};
use crate::models::intent::{IntentRequest, IntentResponse};
use crate::models::project_plan::ProjectPlanResponse;
use crate::models::generated_project::GeneratedProjectResponse;

use crate::routes::ai::{
    __path_detect_intent, __path_generate_backend_plan, __path_generate_blueprint,
    __path_generate_code_preview, __path_generate_frontend_plan, __path_generate_project_plan,
    __path_generate_project,
};
use crate::routes::health::__path_health_check;

#[derive(OpenApi)]
#[openapi(
    paths(
        health_check,
        detect_intent,
        generate_blueprint,
        generate_frontend_plan,
        generate_backend_plan,
        generate_project_plan,
        generate_code_preview,
        generate_project,
    ),
    components(
        schemas(
            IntentRequest,
            IntentResponse,
            PageBlueprint,
            ApiRouteBlueprint,
            BlueprintResponse,
            FilePlanItem,
            FrontendPlanResponse,
            BackendPlanItem,
            BackendPlanResponse,
            ProjectPlanResponse,
            GeneratedFile,
            CodePreviewResponse,
            GeneratedProjectResponse,
        )
    ),
    tags(
        (name = "Health", description = "Backend health check endpoints"),
        (name = "BuildX AI", description = "AI planning endpoints for BuildX")
    )
)]

pub struct ApiDoc;
