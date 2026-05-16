#[derive(Debug, Clone)]
pub struct ModelChoice {
    pub label: &'static str,
    pub description: &'static str,
    pub default_name: &'static str,
}
pub fn choices() -> Vec<ModelChoice> {
    vec![
        ModelChoice {
            label: "Default / Fast",
            description: "confirmed Ollama default; good for general verification and low-end laptops",
            default_name: "gemma4:e2b",
        },
        ModelChoice {
            label: "Strong / High-risk",
            description: "preferred for coding, medical, finance, legal, and critical strictness profiles when installed",
            default_name: "gemma4:e4b",
        },
        ModelChoice {
            label: "Fine-tuned Witness Judge",
            description: "Fine-tuned in one-cell Google Colab T4 GPU with Unsloth; uses GPU VRAM plus system RAM and uploads to Hugging Face Hub",
            default_name: "witness-gemma4-e2b-judge",
        },
        ModelChoice {
            label: "Custom",
            description: "enter any Ollama model name, local path, or OpenAI-compatible model manually",
            default_name: "gemma4:e2b",
        },
    ]
}
