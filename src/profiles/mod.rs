#[derive(Debug, Clone)]
pub struct ValidationProfile {
    pub name: &'static str,
    pub approval: &'static str,
    pub rejection: &'static str,
    pub human_review: &'static str,
    pub repair_style: &'static str,
}
pub fn builtins() -> Vec<ValidationProfile> {
    vec![
        ValidationProfile {
            name: "General Safety",
            approval: "safe, useful, aligned",
            rejection: "unsafe, false, off-prompt",
            human_review: "uncertain high impact",
            repair_style: "clear corrective instruction",
        },
        ValidationProfile {
            name: "Coding",
            approval: "correct, secure, tested code guidance",
            rejection: "vulnerable, hallucinated APIs, destructive commands",
            human_review: "secrets, production migrations",
            repair_style: "ask for precise, tested, secure fix",
        },
        ValidationProfile {
            name: "Education",
            approval: "age-appropriate and explanatory",
            rejection: "misleading or harmful tutoring",
            human_review: "student safety concerns",
            repair_style: "pedagogical correction",
        },
        ValidationProfile {
            name: "Medical",
            approval: "cautious general info",
            rejection: "diagnosis or dangerous dosing",
            human_review: "specific medical decisions",
            repair_style: "add consult-professional caution",
        },
        ValidationProfile {
            name: "Finance",
            approval: "general educational info",
            rejection: "guaranteed returns or personal advice",
            human_review: "investment decisions",
            repair_style: "risk-aware correction",
        },
        ValidationProfile {
            name: "Legal",
            approval: "general legal information",
            rejection: "jurisdiction-specific certainty",
            human_review: "legal action advice",
            repair_style: "add jurisdiction and lawyer caveat",
        },
        ValidationProfile {
            name: "Scientific Research",
            approval: "cited uncertainty and methodology",
            rejection: "fabricated claims",
            human_review: "high-stakes research",
            repair_style: "demand evidence and caveats",
        },
        ValidationProfile {
            name: "Disaster Response",
            approval: "actionable and safe",
            rejection: "unsafe emergency guidance",
            human_review: "life-critical",
            repair_style: "prioritize official guidance",
        },
        ValidationProfile {
            name: "Arabic-English Multilingual",
            approval: "faithful bilingual support",
            rejection: "mistranslation or cultural harm",
            human_review: "ambiguous safety meaning",
            repair_style: "preserve language and meaning",
        },
        ValidationProfile {
            name: "Custom",
            approval: "user-defined",
            rejection: "user-defined",
            human_review: "user-defined",
            repair_style: "user-defined",
        },
    ]
}
