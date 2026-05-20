use the_witness::models::registry::ModelRegistry;

#[test]
fn custom_ollama_models_are_registered_without_replacing_gemma_defaults() {
    let mut registry = ModelRegistry::default_models();

    let entry =
        registry.add_or_update_custom_ollama_model("llama3.2:3b", Some("Local scratch judge"));

    assert_eq!(entry.backend, "ollama");
    assert_eq!(entry.model, "llama3.2:3b");
    assert_eq!(entry.source, "ollama-custom");
    assert!(registry.find("gemma4:e2b").is_some());
    assert!(registry.find("gemma4:e4b").is_some());
    assert!(registry.find("llama3.2:3b").is_some());
}

#[test]
fn custom_ollama_model_names_are_stable_and_editable() {
    let mut registry = ModelRegistry::default_models();

    registry.add_or_update_custom_ollama_model("my-org/custom-gemma:latest", None);
    let updated = registry
        .add_or_update_custom_ollama_model("my-org/custom-gemma:latest", Some("Updated display"));

    assert_eq!(updated.id, "ollama-my-org-custom-gemma-latest");
    assert_eq!(updated.display_name, "Updated display");
    assert_eq!(
        registry
            .models
            .iter()
            .filter(|m| m.model == "my-org/custom-gemma:latest")
            .count(),
        1
    );
}
