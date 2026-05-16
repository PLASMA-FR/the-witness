# Google Colab T4 GPU one-cell fine-tuning guide

The Witness fine-tuning notebooks are now optimized as one-cell Google Colab notebooks for a constrained free/standard Colab-style runtime:

- one NVIDIA T4 GPU,
- about 15 GiB GPU VRAM,
- about 12 GiB system RAM.

The notebooks favor reliability over speed. They use smaller sequence length, batch size 1, gradient accumulation, low LoRA rank, 4-bit loading, checkpoint limits, and adapter-only saving by default.

## Notebooks

- `training/notebooks/finetune_gemma4_e2b_unsloth.ipynb` — recommended first run.
- `training/notebooks/finetune_gemma4_e4b_unsloth.ipynb` — larger optional run; may need even lower settings.

Each notebook contains exactly one code cell. Run that one cell after setting the Colab runtime and Hugging Face settings.

## Required setup

1. Open the notebook in Google Colab.
2. Select `Runtime -> Change runtime type -> T4 GPU`.
3. Add a Hugging Face write token to Colab Secrets as `HF_TOKEN`.
4. Set `HF_REPO_ID` near the top of the single cell, for example:

```python
os.environ.setdefault("HF_REPO_ID", "your-name/witness-gemma4-e2b-judge")
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

The one cell uploads to `HF_REPO_ID` using `HF_TOKEN`:

```text
https://huggingface.co/<HF_REPO_ID>
```

Default artifact type is LoRA adapter + tokenizer + metrics. Merged 16-bit saving is disabled by default because it can exceed Colab RAM/VRAM limits. Enable only if you know the runtime can handle it:

```python
os.environ["WITNESS_SAVE_MERGED"] = "1"
```

## Use the uploaded model in The Witness

```bash
cd /home/admin/Gemma/witness
hf download your-name/witness-gemma4-e2b-judge --local-dir models/witness-gemma4-e2b-judge
./target/debug/the-witness model test --backend unsloth --model ./models/witness-gemma4-e2b-judge
```

## Honest status

The repo contains one-cell Colab T4 notebooks and dataset. The notebooks were locally validated for JSON structure and Python syntax, and project tests pass. They have not been executed on a live Google Colab T4 from this machine. The trained Hugging Face model exists only after you run the notebook and the upload succeeds.
