# Google Colab T4 GPU one-cell fine-tuning guide

The Witness fine-tuning notebooks are now optimized as one-cell Google Colab notebooks for a constrained free/standard Colab-style runtime:

- one NVIDIA T4 GPU,
- about 15 GiB GPU VRAM,
- about 12 GiB system RAM.

The notebooks favor reliability over speed. They use smaller sequence length, batch size 1, gradient accumulation, low LoRA rank, 4-bit loading, checkpoint limits, and adapter-only saving by default.

## Notebooks

- `training/notebooks/finetune_gemma4_e2b_unsloth.ipynb` — recommended E2B adapter workflow.
- Colab notebook link used for the current custom adapter: https://colab.research.google.com/drive/17-CgEQLNg8bpnhhWzJwpapRxQyHIqybq?usp=sharing
- `training/notebooks/finetune_gemma4_e4b_unsloth.ipynb` — experimental template only. E4B was too large for the available runtime and is not the published custom model.

Each notebook contains exactly one code cell. Run that one cell after setting the Colab runtime and Hugging Face settings.

## Required setup

1. Open the notebook in Google Colab.
2. Select `Runtime -> Change runtime type -> T4 GPU`.
3. Add a Hugging Face write token to Colab Secrets as `HF_TOKEN`.
4. Set `HF_REPO_ID` near the top of the single cell, for example:

```python
os.environ.setdefault("HF_REPO_ID", "ahmadalfakeh/witness-gemma4-e2b-judge")
```

5. Run the one cell.

Do not paste a real Hugging Face token into the notebook. Use Colab Secrets or an environment variable.

## What the one cell does

The single cell performs the full workflow:

1. installs dependencies,
2. obtains `HF_TOKEN`,
3. verifies CUDA/T4-like VRAM and system RAM,
4. clones `/content/the-witness` if needed,
5. loads the Witness dataset,
6. loads Gemma in 4-bit with Unsloth/bitsandbytes on CUDA,
7. applies LoRA adapters,
8. trains with slow memory-safe settings,
9. runs lightweight JSON verdict validation,
10. saves adapter/model artifacts,
11. creates a zip archive,
12. creates/updates the Hugging Face model repo,
13. uploads the output folder to Hugging Face Hub.

## Memory behavior

The notebooks use both GPU VRAM and system RAM deliberately:

- GPU VRAM: quantized base model, LoRA adapters, activations, gradients/training tensors.
- System RAM/disk: dataset rows, tokenizer files, Python process, dataloader buffers, checkpoints, metrics, zipping, and Hugging Face upload plumbing.

The cell fails fast if CUDA is unavailable:

```python
if not torch.cuda.is_available():
    raise RuntimeError(...)
```

It also checks model placement:

```python
param_device = next(model.parameters()).device
if param_device.type != "cuda":
    raise RuntimeError(...)
```

This prevents accidental CPU/system-RAM-only training.

## Memory-safe defaults

The one-cell notebooks default to:

```text
WITNESS_MAX_SEQ_LENGTH=1024
WITNESS_BATCH_SIZE=1
WITNESS_GRAD_ACCUM=8
WITNESS_LORA_RANK=8
WITNESS_LORA_ALPHA=16
WITNESS_LEARNING_RATE=1.5e-4
WITNESS_MAX_STEPS=300
WITNESS_VAL_LIMIT=300
WITNESS_SAVE_MERGED=0
```

These defaults are slower but safer for one T4 with about 12 GiB system RAM and 15 GiB VRAM.

If memory is still tight, reduce:

```python
os.environ["WITNESS_MAX_SEQ_LENGTH"] = "768"
os.environ["WITNESS_GRAD_ACCUM"] = "4"
os.environ["WITNESS_MAX_STEPS"] = "100"
os.environ["WITNESS_VAL_LIMIT"] = "100"
```

## Hugging Face output

The current custom E2B adapter has been published to:

```text
https://huggingface.co/ahmadalfakeh/witness-gemma4-e2b-judge
```

The one cell uploads to `HF_REPO_ID` using `HF_TOKEN`:

```text
https://huggingface.co/<HF_REPO_ID>
```

Default artifact type is LoRA adapter + tokenizer + metrics. This is intentionally not the full multi-GB Gemma base model. At runtime, load the original Gemma 4 E2B base model (`google/gemma-4-e2b`, or the configured equivalent) and attach the adapter. Merged 16-bit saving is disabled by default because it can exceed Colab RAM/VRAM limits. Enable only if you know the runtime can handle it:

```python
os.environ["WITNESS_SAVE_MERGED"] = "1"
```

## Use the uploaded model in The Witness

```bash
cd /home/admin/Gemma/witness
hf download ahmadalfakeh/witness-gemma4-e2b-judge --local-dir models/witness-gemma4-e2b-judge
./target/debug/the-witness model test --backend unsloth --model ./models/witness-gemma4-e2b-judge
```

## Honest status

The current E2B LoRA adapter is published at https://huggingface.co/ahmadalfakeh/witness-gemma4-e2b-judge. The E4B adapter is not published/trained for this project because it was too large for the available runtime. Local repository validation still only checks notebook JSON/syntax and project tests; it does not prove that this machine ran Colab training.
