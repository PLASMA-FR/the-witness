# The Witness Training

This folder contains dataset, scripts, notebooks, and model cards for fine-tuning the Gemma 4 E2B judge LoRA adapter. E4B assets are experimental templates only; the E4B adapter was not published/trained because it was too large for the available runtime.

Primary runtime: Google Colab T4 GPU, in a single notebook cell, optimized for about 12 GiB system RAM and 15 GiB GPU VRAM.

The one-cell notebooks use Unsloth + bitsandbytes 4-bit LoRA/QLoRA. They deliberately use GPU VRAM for the quantized model, LoRA adapters, activations, and training tensors, while system RAM/disk handle dataset rows, tokenizer files, dataloader buffers, checkpoints, metrics, archives, and Hugging Face upload plumbing.

Required Colab setup:

1. Open `training/notebooks/finetune_gemma4_e2b_unsloth.ipynb`.
2. Select `Runtime -> Change runtime type -> T4 GPU`.
3. Add a Hugging Face write token to Colab Secrets as `HF_TOKEN`.
4. Set `HF_REPO_ID` at the top of the one cell, for example `ahmadalfakeh/witness-gemma4-e2b-judge`.
5. Run the one cell.

The one cell installs packages, checks CUDA/GPU/system RAM, trains, validates, saves artifacts, zips them, creates the Hugging Face repo if needed, and uploads the model folder using the token.

Memory-safe defaults are intentionally slower:

```text
WITNESS_MAX_SEQ_LENGTH=1024
WITNESS_BATCH_SIZE=1
WITNESS_GRAD_ACCUM=8
WITNESS_LORA_RANK=8
WITNESS_MAX_STEPS=300
WITNESS_VAL_LIMIT=300
WITNESS_SAVE_MERGED=0
```

`WITNESS_SAVE_MERGED=0` avoids a merged 16-bit export by default because it can exceed a constrained T4/12 GiB RAM runtime. The default artifact is the LoRA adapter plus tokenizer, metrics, model card, and validation predictions.

Validate the dataset before training:

```bash
python3 training/scripts/validate_dataset.py
```

Full guide:

```text
docs/google_colab_finetuning.md
```
