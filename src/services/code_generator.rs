use indoc::indoc;

use crate::models::generated_code::{CodePreviewResponse, GeneratedFile};
use crate::services::{
    ai_client::generate_code_bundle_with_ai, planner::build_code_preview_response,
    prompt_builder::build_code_generation_prompt,
};

pub async fn generate_code_from_prompt(prompt: &str) -> CodePreviewResponse {
    let ai_prompt = build_code_generation_prompt(prompt);

    match generate_code_bundle_with_ai(&ai_prompt).await {
        Ok(Some(ai_response)) => {
            println!("AI generated dynamic UI files successfully.");
            merge_base_files_with_ai_files(ai_response)
        }

        Ok(None) => {
            println!("AI not available. Using fallback generator.");
            build_code_preview_response(prompt)
        }

        Err(error) => {
            println!("AI generation failed: {}", error);
            println!("Using fallback generator.");
            build_code_preview_response(prompt)
        }
    }
}

fn merge_base_files_with_ai_files(ai_response: CodePreviewResponse) -> CodePreviewResponse {
    let mut files = generate_base_react_files();

    for file in ai_response.files {
        if is_allowed_ai_file(&file.path) {
            files.push(file);
        }
    }

    CodePreviewResponse {
        files,
        summary: ai_response.summary,
    }
}

fn is_allowed_ai_file(path: &str) -> bool {
    matches!(path, "src/App.jsx" | "src/styles/app.css")
}

fn generate_base_react_files() -> Vec<GeneratedFile> {
    vec![
        GeneratedFile {
            path: "package.json".to_string(),
            language: "json".to_string(),
            content: indoc! { r#"
                {
                  "name": "buildx-generated-app",
                  "version": "0.1.0",
                  "private": true,
                  "type": "module",
                  "scripts": {
                    "dev": "vite",
                    "build": "vite build",
                    "preview": "vite preview"
                  },
                  "dependencies": {
                    "@vitejs/plugin-react": "latest",
                    "vite": "latest",
                    "react": "latest",
                    "react-dom": "latest"
                  },
                  "devDependencies": {}
                }
            "# }
            .to_string(),
        },
        GeneratedFile {
            path: "index.html".to_string(),
            language: "html".to_string(),
            content: indoc! { r#"
                <!doctype html>
                <html lang="en">
                  <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    <title>BuildX Generated App</title>
                  </head>
                  <body>
                    <div id="root"></div>
                    <script type="module" src="/src/main.jsx"></script>
                  </body>
                </html>
            "# }
            .to_string(),
        },
        GeneratedFile {
            path: "vite.config.js".to_string(),
            language: "javascript".to_string(),
            content: indoc! { r#"
                import { defineConfig } from "vite";
                import react from "@vitejs/plugin-react";

                export default defineConfig({
                  plugins: [react()],
                });
            "# }
            .to_string(),
        },
        GeneratedFile {
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
        },
        GeneratedFile {
            path: "src/styles/global.css".to_string(),
            language: "css".to_string(),
            content: indoc! { r#"
                * {
                  box-sizing: border-box;
                }

                body {
                  margin: 0;
                  font-family: Inter, system-ui, sans-serif;
                  background: #f8fafc;
                  color: #0f172a;
                }

                button,
                input,
                textarea {
                  font: inherit;
                }

                button {
                  cursor: pointer;
                }
            "# }
            .to_string(),
        },
    ]
}
