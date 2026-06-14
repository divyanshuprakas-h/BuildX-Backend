use indoc::indoc;

use crate::models::backend_plan::{BackendPlanItem, BackendPlanResponse};
use crate::models::blueprint::{ApiRouteBlueprint, BlueprintResponse, PageBlueprint};
use crate::models::file_plan::{FilePlanItem, FrontendPlanResponse};
use crate::models::generated_code::{CodePreviewResponse, GeneratedFile};
use crate::models::project_plan::ProjectPlanResponse;

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

pub fn build_frontend_plan_response(prompt_input: &str) -> FrontendPlanResponse {
    let prompt = prompt_input.to_lowercase();

    let project_type = detect_project_type(&prompt);
    let files = generate_frontend_files(&prompt, project_type);

    FrontendPlanResponse {
        framework: "React + Vite".to_string(),
        styling: "CSS Modules or Tailwind CSS".to_string(),
        files,
        summary: "Frontend file plan generated successfully.".to_string(),
    }
}

pub fn build_backend_plan_response(prompt_input: &str) -> BackendPlanResponse {
    let prompt = prompt_input.to_lowercase();

    let project_type = detect_project_type(&prompt);
    let database = detect_database_choice(&prompt);
    let files = generate_backend_files(&prompt, project_type);
    let dependencies = generate_backend_dependencies(&prompt);

    BackendPlanResponse {
        framework: "Rust + Axum".to_string(),
        database,
        files,
        dependencies,
        summary: "Backend file plan generated successfully.".to_string(),
    }
}

pub fn build_project_plan_response(prompt_input: &str) -> ProjectPlanResponse {
    let intent = build_intent_response(prompt_input);
    let blueprint = build_blueprint_response(prompt_input);
    let frontend_plan = build_frontend_plan_response(prompt_input);
    let backend_plan = build_backend_plan_response(prompt_input);

    ProjectPlanResponse {
        intent,
        blueprint,
        frontend_plan,
        backend_plan,
        summary: "Full project plan generated successfully.".to_string(),
    }
}

pub fn build_code_preview_response(prompt_input: &str) -> CodePreviewResponse {
    let prompt = prompt_input.to_lowercase();
    let project_type = detect_project_type(&prompt);

    let files = generate_code_preview_files(&prompt, project_type);

    CodePreviewResponse {
        files,
        summary: "Code preview generated successfully.".to_string(),
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

fn generate_frontend_files(prompt: &str, project_type: &str) -> Vec<FilePlanItem> {
    let mut files = Vec::new();

    files.push(FilePlanItem {
        path: "src/main.jsx".to_string(),
        file_type: "entry".to_string(),
        purpose: "React application entry point".to_string(),
    });

    files.push(FilePlanItem {
        path: "src/App.jsx".to_string(),
        file_type: "app".to_string(),
        purpose: "Main app component and route setup".to_string(),
    });

    files.push(FilePlanItem {
        path: "src/styles/global.css".to_string(),
        file_type: "style".to_string(),
        purpose: "Global styles for the application".to_string(),
    });

    files.push(FilePlanItem {
        path: "src/pages/Home.jsx".to_string(),
        file_type: "page".to_string(),
        purpose: "Home page of the application".to_string(),
    });

    files.push(FilePlanItem {
        path: "src/components/Navbar.jsx".to_string(),
        file_type: "component".to_string(),
        purpose: "Top navigation component".to_string(),
    });

    files.push(FilePlanItem {
        path: "src/components/Footer.jsx".to_string(),
        file_type: "component".to_string(),
        purpose: "Footer component".to_string(),
    });

    if has_auth(prompt) {
        files.push(FilePlanItem {
            path: "src/pages/Login.jsx".to_string(),
            file_type: "page".to_string(),
            purpose: "Login page".to_string(),
        });

        files.push(FilePlanItem {
            path: "src/pages/Signup.jsx".to_string(),
            file_type: "page".to_string(),
            purpose: "Signup page".to_string(),
        });

        files.push(FilePlanItem {
            path: "src/components/LoginForm.jsx".to_string(),
            file_type: "component".to_string(),
            purpose: "Reusable login form component".to_string(),
        });

        files.push(FilePlanItem {
            path: "src/components/SignupForm.jsx".to_string(),
            file_type: "component".to_string(),
            purpose: "Reusable signup form component".to_string(),
        });
    }

    if project_type == "todo_app" {
        files.push(FilePlanItem {
            path: "src/pages/Tasks.jsx".to_string(),
            file_type: "page".to_string(),
            purpose: "Task management page".to_string(),
        });

        files.push(FilePlanItem {
            path: "src/components/TaskInput.jsx".to_string(),
            file_type: "component".to_string(),
            purpose: "Input component for creating tasks".to_string(),
        });

        files.push(FilePlanItem {
            path: "src/components/TaskList.jsx".to_string(),
            file_type: "component".to_string(),
            purpose: "List component for showing tasks".to_string(),
        });

        files.push(FilePlanItem {
            path: "src/components/TaskItem.jsx".to_string(),
            file_type: "component".to_string(),
            purpose: "Single task item component".to_string(),
        });
    }

    if project_type == "dashboard" {
        files.push(FilePlanItem {
            path: "src/pages/Dashboard.jsx".to_string(),
            file_type: "page".to_string(),
            purpose: "Dashboard page".to_string(),
        });

        files.push(FilePlanItem {
            path: "src/components/Sidebar.jsx".to_string(),
            file_type: "component".to_string(),
            purpose: "Dashboard sidebar navigation".to_string(),
        });

        files.push(FilePlanItem {
            path: "src/components/DashboardCards.jsx".to_string(),
            file_type: "component".to_string(),
            purpose: "Cards for showing dashboard stats".to_string(),
        });

        files.push(FilePlanItem {
            path: "src/components/AnalyticsChart.jsx".to_string(),
            file_type: "component".to_string(),
            purpose: "Chart component for analytics".to_string(),
        });
    }

    if project_type == "blog_app" {
        files.push(FilePlanItem {
            path: "src/pages/Blog.jsx".to_string(),
            file_type: "page".to_string(),
            purpose: "Blog listing page".to_string(),
        });

        files.push(FilePlanItem {
            path: "src/pages/CreatePost.jsx".to_string(),
            file_type: "page".to_string(),
            purpose: "Create blog post page".to_string(),
        });

        files.push(FilePlanItem {
            path: "src/components/PostCard.jsx".to_string(),
            file_type: "component".to_string(),
            purpose: "Blog post preview card".to_string(),
        });

        files.push(FilePlanItem {
            path: "src/components/PostEditor.jsx".to_string(),
            file_type: "component".to_string(),
            purpose: "Blog post editor component".to_string(),
        });
    }

    if project_type == "ecommerce" {
        files.push(FilePlanItem {
            path: "src/pages/Products.jsx".to_string(),
            file_type: "page".to_string(),
            purpose: "Product listing page".to_string(),
        });

        files.push(FilePlanItem {
            path: "src/pages/Cart.jsx".to_string(),
            file_type: "page".to_string(),
            purpose: "Shopping cart page".to_string(),
        });

        files.push(FilePlanItem {
            path: "src/components/ProductCard.jsx".to_string(),
            file_type: "component".to_string(),
            purpose: "Product card component".to_string(),
        });

        files.push(FilePlanItem {
            path: "src/components/CartItem.jsx".to_string(),
            file_type: "component".to_string(),
            purpose: "Cart item component".to_string(),
        });
    }

    if project_type == "portfolio" {
        files.push(FilePlanItem {
            path: "src/pages/Projects.jsx".to_string(),
            file_type: "page".to_string(),
            purpose: "Portfolio projects page".to_string(),
        });

        files.push(FilePlanItem {
            path: "src/pages/Contact.jsx".to_string(),
            file_type: "page".to_string(),
            purpose: "Contact page".to_string(),
        });

        files.push(FilePlanItem {
            path: "src/components/ProjectCard.jsx".to_string(),
            file_type: "component".to_string(),
            purpose: "Project showcase card".to_string(),
        });

        files.push(FilePlanItem {
            path: "src/components/ContactForm.jsx".to_string(),
            file_type: "component".to_string(),
            purpose: "Contact form component".to_string(),
        });
    }

    files
}

fn detect_database_choice(prompt: &str) -> String {
    if has_database(prompt) {
        "PostgreSQL".to_string()
    } else {
        "None".to_string()
    }
}

fn generate_backend_dependencies(prompt: &str) -> Vec<String> {
    let mut dependencies = Vec::new();

    dependencies.push("axum".to_string());
    dependencies.push("tokio".to_string());
    dependencies.push("serde".to_string());
    dependencies.push("serde_json".to_string());

    if has_auth(prompt) {
        dependencies.push("jsonwebtoken".to_string());
        dependencies.push("bcrypt".to_string());
    }

    if has_database(prompt) {
        dependencies.push("sqlx".to_string());
        dependencies.push("dotenvy".to_string());
    }

    dependencies
}

fn generate_backend_files(prompt: &str, project_type: &str) -> Vec<BackendPlanItem> {
    let mut files = Vec::new();

    files.push(BackendPlanItem {
        path: "src/main.rs".to_string(),
        file_type: "entry".to_string(),
        purpose: "Backend server entry point".to_string(),
    });

    files.push(BackendPlanItem {
        path: "src/app.rs".to_string(),
        file_type: "app".to_string(),
        purpose: "Creates Axum router and connects routes".to_string(),
    });

    files.push(BackendPlanItem {
        path: "src/routes/mod.rs".to_string(),
        file_type: "module".to_string(),
        purpose: "Exports all route modules".to_string(),
    });

    files.push(BackendPlanItem {
        path: "src/routes/health.rs".to_string(),
        file_type: "route".to_string(),
        purpose: "Health check API route".to_string(),
    });

    files.push(BackendPlanItem {
        path: "src/models/mod.rs".to_string(),
        file_type: "module".to_string(),
        purpose: "Exports all model modules".to_string(),
    });

    files.push(BackendPlanItem {
        path: "src/services/mod.rs".to_string(),
        file_type: "module".to_string(),
        purpose: "Exports all service modules".to_string(),
    });

    if has_auth(prompt) {
        files.push(BackendPlanItem {
            path: "src/routes/auth.rs".to_string(),
            file_type: "route".to_string(),
            purpose: "Signup, login, and authentication routes".to_string(),
        });

        files.push(BackendPlanItem {
            path: "src/models/user.rs".to_string(),
            file_type: "model".to_string(),
            purpose: "User request and response structs".to_string(),
        });

        files.push(BackendPlanItem {
            path: "src/services/auth_service.rs".to_string(),
            file_type: "service".to_string(),
            purpose: "Authentication business logic".to_string(),
        });
    }

    if has_database(prompt) {
        files.push(BackendPlanItem {
            path: "src/db/mod.rs".to_string(),
            file_type: "database".to_string(),
            purpose: "Database connection module".to_string(),
        });
    }

    if project_type == "todo_app" {
        files.push(BackendPlanItem {
            path: "src/routes/tasks.rs".to_string(),
            file_type: "route".to_string(),
            purpose: "Task CRUD API routes".to_string(),
        });

        files.push(BackendPlanItem {
            path: "src/models/task.rs".to_string(),
            file_type: "model".to_string(),
            purpose: "Task data structs".to_string(),
        });

        files.push(BackendPlanItem {
            path: "src/services/task_service.rs".to_string(),
            file_type: "service".to_string(),
            purpose: "Task management business logic".to_string(),
        });
    }

    if project_type == "blog_app" {
        files.push(BackendPlanItem {
            path: "src/routes/posts.rs".to_string(),
            file_type: "route".to_string(),
            purpose: "Blog post CRUD API routes".to_string(),
        });

        files.push(BackendPlanItem {
            path: "src/models/post.rs".to_string(),
            file_type: "model".to_string(),
            purpose: "Blog post data structs".to_string(),
        });

        files.push(BackendPlanItem {
            path: "src/services/post_service.rs".to_string(),
            file_type: "service".to_string(),
            purpose: "Blog post business logic".to_string(),
        });
    }

    if project_type == "dashboard" || prompt.contains("analytics") {
        files.push(BackendPlanItem {
            path: "src/routes/analytics.rs".to_string(),
            file_type: "route".to_string(),
            purpose: "Analytics API routes".to_string(),
        });

        files.push(BackendPlanItem {
            path: "src/services/analytics_service.rs".to_string(),
            file_type: "service".to_string(),
            purpose: "Analytics calculation logic".to_string(),
        });
    }

    if project_type == "ecommerce" {
        files.push(BackendPlanItem {
            path: "src/routes/products.rs".to_string(),
            file_type: "route".to_string(),
            purpose: "Product API routes".to_string(),
        });

        files.push(BackendPlanItem {
            path: "src/routes/orders.rs".to_string(),
            file_type: "route".to_string(),
            purpose: "Order API routes".to_string(),
        });

        files.push(BackendPlanItem {
            path: "src/models/product.rs".to_string(),
            file_type: "model".to_string(),
            purpose: "Product data structs".to_string(),
        });

        files.push(BackendPlanItem {
            path: "src/services/product_service.rs".to_string(),
            file_type: "service".to_string(),
            purpose: "Product business logic".to_string(),
        });
    }

    files
}

fn generate_code_preview_files(prompt: &str, project_type: &str) -> Vec<GeneratedFile> {
    let mut files = Vec::new();

    files.push(GeneratedFile {
        path: "src/main.jsx".to_string(),
        language: "jsx".to_string(),
        content: indoc! { r#"
                        import React from "react";
                        import ReactDOM from "react-dom/client";
                        import App from "./App";
                        import "./styles/global.css";

                        ReactDOM.createRoot(document.getElementById("root")).render(
                        <React.StrictMode>
                            <App />
                        </React.StrictMode>
                        );
                    "# }
        .to_string(),
    });

    files.push(GeneratedFile {
        path: "src/styles/global.css".to_string(),
        language: "css".to_string(),
        content: indoc! {r#"* {
            box-sizing: border-box;
            }

            body {
            margin: 0;
            font-family: system-ui, sans-serif;
            background: #f8fafc;
            color: #0f172a;
            }

            button {
            cursor: pointer;
            }
        "#}
        .to_string(),
    });

    if project_type == "todo_app" {
        files.push(GeneratedFile {
            path: "src/App.jsx".to_string(),
            language: "jsx".to_string(),
            content: indoc! {r#"
                            import { useState } from "react";
                            import "./styles/todo.css";

                            function App() {
                            const [tasks, setTasks] = useState([]);
                            const [taskText, setTaskText] = useState("");

                            const addTask = () => {
                                if (!taskText.trim()) return;

                                setTasks([
                                ...tasks,
                                {
                                    id: Date.now(),
                                    title: taskText,
                                    completed: false,
                                },
                                ]);

                                setTaskText("");
                            };

                            const toggleTask = (id) => {
                                setTasks(
                                tasks.map((task) =>
                                    task.id === id
                                    ? { ...task, completed: !task.completed }
                                    : task
                                )
                                );
                            };

                            return (
                                <main className="app">
                                <section className="card">
                                    <h1>BuildX Todo App</h1>
                                    <p>Create and manage your daily tasks.</p>

                                    <div className="task-form">
                                    <input
                                        value={taskText}
                                        onChange={(event) => setTaskText(event.target.value)}
                                        placeholder="Enter a task"
                                    />
                                    <button onClick={addTask}>Add Task</button>
                                    </div>

                                    <ul className="task-list">
                                    {tasks.map((task) => (
                                        <li key={task.id}>
                                        <label>
                                            <input
                                            type="checkbox"
                                            checked={task.completed}
                                            onChange={() => toggleTask(task.id)}
                                            />
                                            <span className={task.completed ? "done" : ""}>
                                            {task.title}
                                            </span>
                                        </label>
                                        </li>
                                    ))}
                                    </ul>
                                </section>
                                </main>
                            );
                            }

                            export default App;
                            "#}
            .to_string(),
        });

        files.push(GeneratedFile {
            path: "src/styles/todo.css".to_string(),
            language: "css".to_string(),
            content: indoc! {r#".app {
                        min-height: 100vh;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        padding: 24px;
                        }

                        .card {
                        width: 100%;
                        max-width: 520px;
                        background: white;
                        border-radius: 18px;
                        padding: 28px;
                        box-shadow: 0 20px 60px rgba(15, 23, 42, 0.12);
                        }

                        .task-form {
                        display: flex;
                        gap: 12px;
                        margin-top: 20px;
                        }

                        .task-form input {
                        flex: 1;
                        padding: 12px 14px;
                        border: 1px solid #cbd5e1;
                        border-radius: 10px;
                        }

                        .task-form button {
                        padding: 12px 16px;
                        border: none;
                        border-radius: 10px;
                        background: #0f172a;
                        color: white;
                        }

                        .task-list {
                        list-style: none;
                        padding: 0;
                        margin-top: 24px;
                        }

                        .task-list li {
                        padding: 12px 0;
                        border-bottom: 1px solid #e2e8f0;
                        }

                        .done {
                        text-decoration: line-through;
                        color: #64748b;
                        }
                        "#}
            .to_string(),
        });
    } else {
        files.push(GeneratedFile {
            path: "src/App.jsx".to_string(),
            language: "jsx".to_string(),
            content: indoc! {r#"
            function App() {
            return (
                <main className="app">
                <section className="card">
                    <h1>BuildX Generated App</h1>
                    <p>Your app starter code is ready.</p>
                </section>
                </main>
            );
            }

            export default App;
            "#}
            .to_string(),
        });
    }

    if has_auth(prompt) {
        files.push(GeneratedFile {
            path: "src/components/LoginForm.jsx".to_string(),
            language: "jsx".to_string(),
            content: indoc! {r#"
            function LoginForm() {
            return (
                <form>
                <h2>Login</h2>

                <input type="email" placeholder="Email" />
                <input type="password" placeholder="Password" />

                <button type="submit">Login</button>
                </form>
            );
            }

            export default LoginForm;
            "#}
            .to_string(),
        });
    }

    files
}
