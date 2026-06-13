use crate::models::blueprint::{ApiRouteBlueprint, BlueprintResponse, PageBlueprint};

use crate::models::intent::IntentResponse;

pub fn build_intent_response(prompt_input: &str) -> IntentResponse {
    let prompt = prompt_input.to_lowercase();

    let needs_auth = has_auth(&prompt);
    let needs_database = has_database(&prompt);
    let needs_backend = has_backend(&prompt);

    let project_type = detect_project_type(&prompt);
    let complexity = detect_complexity(needs_auth, needs_database, needs_backend);
    let intent = detect_action_intent(&prompt);

    IntentResponse {
        intent: intent.to_string(),
        project_type: project_type.to_string(),
        complexity: complexity.to_string(),
        needs_backend,
        needs_auth,
        needs_database,
        message: format!(
            "BuildX understood your idea as a {} with {} complexity.",
            project_type, complexity
        ),
    }
}

pub fn build_blueprint_response(prompt_input: &str) -> BlueprintResponse {
    let prompt = prompt_input.to_lowercase();

    let needs_auth = has_auth(&prompt);
    let needs_database = has_database(&prompt);
    let needs_backend = has_backend(&prompt);

    let project_type = detect_project_type(&prompt);
    let complexity = detect_complexity(needs_auth, needs_database, needs_backend);

    let pages = generate_pages(&prompt, project_type);
    let components = generate_components(&prompt, project_type);
    let features = generate_features(&prompt, project_type);
    let api_routes = generate_api_routes(&prompt);
    let database_tables = generate_database_tables(&prompt, project_type);

    BlueprintResponse {
        project_name: "BuildX Generated App".to_string(),
        project_type: project_type.to_string(),
        complexity: complexity.to_string(),
        pages,
        components,
        features,
        api_routes,
        database_tables,
        summary: "Project blueprint generated successfully.".to_string(),
    }
}

fn has_auth(prompt: &str) -> bool {
    prompt.contains("login")
        || prompt.contains("signup")
        || prompt.contains("auth")
        || prompt.contains("user account")
}

fn has_database(prompt: &str) -> bool {
    prompt.contains("database")
        || prompt.contains("save")
        || prompt.contains("store")
        || prompt.contains("dashboard")
        || prompt.contains("crud")
        || prompt.contains("todo")
        || prompt.contains("blog")
}

fn has_backend(prompt: &str) -> bool {
    has_auth(prompt)
        || has_database(prompt)
        || prompt.contains("api")
        || prompt.contains("backend")
        || prompt.contains("server")
}

fn detect_project_type(prompt: &str) -> &'static str {
    if prompt.contains("todo") {
        "todo_app"
    } else if prompt.contains("blog") {
        "blog_app"
    } else if prompt.contains("dashboard") {
        "dashboard"
    } else if prompt.contains("ecommerce") || prompt.contains("shop") {
        "ecommerce"
    } else if prompt.contains("portfolio") {
        "portfolio"
    } else if prompt.contains("landing page") {
        "landing_page"
    } else {
        "custom_app"
    }
}

fn detect_complexity(needs_auth: bool, needs_database: bool, needs_backend: bool) -> &'static str {
    if needs_auth && needs_database && needs_backend {
        "high"
    } else if needs_backend || needs_database {
        "medium"
    } else {
        "low"
    }
}

fn detect_action_intent(prompt: &str) -> &'static str {
    if prompt.contains("create")
        || prompt.contains("build")
        || prompt.contains("make")
        || prompt.contains("generate")
    {
        "generate_project"
    } else {
        "unknown"
    }
}

fn generate_pages(prompt: &str, project_type: &str) -> Vec<PageBlueprint> {
    let mut pages = Vec::new();

    pages.push(PageBlueprint {
        name: "Home".to_string(),
        path: "/".to_string(),
        purpose: "Main page of the application".to_string(),
    });

    if has_auth(prompt) {
        pages.push(PageBlueprint {
            name: "Login".to_string(),
            path: "/login".to_string(),
            purpose: "Allows users to login".to_string(),
        });

        pages.push(PageBlueprint {
            name: "Signup".to_string(),
            path: "/signup".to_string(),
            purpose: "Allows new users to create an account".to_string(),
        });
    }

    if project_type == "dashboard" {
        pages.push(PageBlueprint {
            name: "Dashboard".to_string(),
            path: "/dashboard".to_string(),
            purpose: "Shows user data, stats, and analytics".to_string(),
        });
    }

    if project_type == "todo_app" {
        pages.push(PageBlueprint {
            name: "Tasks".to_string(),
            path: "/tasks".to_string(),
            purpose: "Allows users to manage todo tasks".to_string(),
        });
    }

    if project_type == "blog_app" {
        pages.push(PageBlueprint {
            name: "Blog".to_string(),
            path: "/blog".to_string(),
            purpose: "Shows all blog posts".to_string(),
        });

        pages.push(PageBlueprint {
            name: "Create Post".to_string(),
            path: "/blog/new".to_string(),
            purpose: "Allows users to create a new blog post".to_string(),
        });
    }

    if project_type == "ecommerce" {
        pages.push(PageBlueprint {
            name: "Products".to_string(),
            path: "/products".to_string(),
            purpose: "Shows products for sale".to_string(),
        });

        pages.push(PageBlueprint {
            name: "Cart".to_string(),
            path: "/cart".to_string(),
            purpose: "Shows selected cart items".to_string(),
        });
    }

    if project_type == "portfolio" {
        pages.push(PageBlueprint {
            name: "Projects".to_string(),
            path: "/projects".to_string(),
            purpose: "Shows portfolio projects".to_string(),
        });

        pages.push(PageBlueprint {
            name: "Contact".to_string(),
            path: "/contact".to_string(),
            purpose: "Allows visitors to contact the owner".to_string(),
        });
    }

    pages
}

fn generate_components(prompt: &str, project_type: &str) -> Vec<String> {
    let mut components = Vec::new();

    components.push("Navbar".to_string());
    components.push("HeroSection".to_string());
    components.push("Footer".to_string());

    if has_auth(prompt) {
        components.push("LoginForm".to_string());
        components.push("SignupForm".to_string());
    }

    if project_type == "dashboard" {
        components.push("Sidebar".to_string());
        components.push("DashboardCards".to_string());
        components.push("AnalyticsChart".to_string());
    }

    if project_type == "todo_app" {
        components.push("TaskInput".to_string());
        components.push("TaskList".to_string());
        components.push("TaskItem".to_string());
    }

    if project_type == "blog_app" {
        components.push("PostCard".to_string());
        components.push("PostEditor".to_string());
    }

    if project_type == "ecommerce" {
        components.push("ProductCard".to_string());
        components.push("CartItem".to_string());
        components.push("CheckoutButton".to_string());
    }

    if project_type == "portfolio" {
        components.push("ProjectCard".to_string());
        components.push("ContactForm".to_string());
    }

    components
}

fn generate_features(prompt: &str, project_type: &str) -> Vec<String> {
    let mut features = Vec::new();

    features.push("responsive_ui".to_string());

    if has_auth(prompt) {
        features.push("authentication".to_string());
    }

    if has_database(prompt) {
        features.push("database_storage".to_string());
    }

    if prompt.contains("analytics") || prompt.contains("chart") {
        features.push("analytics".to_string());
    }

    if project_type == "todo_app" {
        features.push("task_management".to_string());
    }

    if project_type == "blog_app" {
        features.push("blog_post_management".to_string());
    }

    if project_type == "ecommerce" {
        features.push("product_listing".to_string());
        features.push("cart_system".to_string());
    }

    if project_type == "portfolio" {
        features.push("project_showcase".to_string());
    }

    features
}

fn generate_api_routes(prompt: &str) -> Vec<ApiRouteBlueprint> {
    let mut routes = Vec::new();

    if has_auth(prompt) {
        routes.push(ApiRouteBlueprint {
            method: "POST".to_string(),
            path: "/auth/signup".to_string(),
            purpose: "Create a new user account".to_string(),
        });

        routes.push(ApiRouteBlueprint {
            method: "POST".to_string(),
            path: "/auth/login".to_string(),
            purpose: "Login user account".to_string(),
        });
    }

    if prompt.contains("todo") {
        routes.push(ApiRouteBlueprint {
            method: "GET".to_string(),
            path: "/tasks".to_string(),
            purpose: "Fetch all tasks".to_string(),
        });

        routes.push(ApiRouteBlueprint {
            method: "POST".to_string(),
            path: "/tasks".to_string(),
            purpose: "Create a new task".to_string(),
        });
    }

    if prompt.contains("blog") {
        routes.push(ApiRouteBlueprint {
            method: "GET".to_string(),
            path: "/posts".to_string(),
            purpose: "Fetch all blog posts".to_string(),
        });

        routes.push(ApiRouteBlueprint {
            method: "POST".to_string(),
            path: "/posts".to_string(),
            purpose: "Create a new blog post".to_string(),
        });
    }

    if prompt.contains("dashboard") || prompt.contains("analytics") {
        routes.push(ApiRouteBlueprint {
            method: "GET".to_string(),
            path: "/analytics".to_string(),
            purpose: "Fetch analytics data".to_string(),
        });
    }

    routes
}

fn generate_database_tables(prompt: &str, project_type: &str) -> Vec<String> {
    let mut tables = Vec::new();

    if has_auth(prompt) {
        tables.push("users".to_string());
    }

    if project_type == "todo_app" {
        tables.push("tasks".to_string());
    }

    if project_type == "blog_app" {
        tables.push("posts".to_string());
    }

    if project_type == "ecommerce" {
        tables.push("products".to_string());
        tables.push("orders".to_string());
        tables.push("cart_items".to_string());
    }

    if prompt.contains("dashboard") || prompt.contains("analytics") {
        tables.push("analytics_events".to_string());
    }

    tables
}
