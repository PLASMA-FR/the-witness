# Legacy Kaggle notes for The Witness

The current custom Witness judge is not stored in Kaggle.

Use the Hugging Face E2B LoRA adapter instead:

```text
https://huggingface.co/ahmadalfakeh/witness-gemma4-e2b-judge
```

Download it with:

```bash
the-witness model download --source huggingface --model witness-gemma4-e2b-judge
```

This artifact is adapter-only LoRA output. It must be loaded with the original Gemma 4 E2B base model (`google/gemma-4-e2b`, or the configured equivalent). It is not a full multi-GB model checkout.

The old Kaggle scripts remain in `training/scripts/` only as legacy optional utilities for users who intentionally publish their own copies to Kaggle. They are not used by the default registry entry and they do not point to the current custom model.

If you use those legacy scripts, keep Kaggle credentials outside the repo and never commit `kaggle.json`.
