# Fine-tuning notebooks

These notebooks are intentionally one-cell Google Colab notebooks for fine-tuning The Witness Gemma 4 judge.

Primary runtime: one Google Colab T4 GPU with about 15 GiB GPU VRAM and about 12 GiB system RAM.

Files:

- `finetune_gemma4_e2b_unsloth.ipynb` — recommended first run; one code cell fine-tunes `witness-gemma4-e2b-judge`.
- `finetune_gemma4_e4b_unsloth.ipynb` — experimental template only. E4B was too large for the available runtime and is not the published custom model.

## Why one cell?

The user can open the notebook, select T4 GPU, set Hugging Face variables/secrets, and run exactly one cell. That one cell:

1. installs dependencies,
2. reads the Hugging Face token from Colab Secrets or `HF_TOKEN`,
3. checks CUDA, GPU VRAM, and system RAM,
4. clones/uses the repo dataset,
5. loads Gemma through Unsloth 4-bit QLoRA on GPU VRAM,
6. trains with slow memory-safe settings,
7. validates JSON verdict output,
8. saves adapter/model artifacts,
9. creates a zip archive,
10. uploads the output folder to Hugging Face Hub.

## Required Colab setup

1. Open the notebook in Google Colab.
2. Select `Runtime -> Change runtime type -> T4 GPU`.
3. Add `HF_TOKEN` to Colab Secrets. Use a Hugging Face write token. Do not paste tokens into the notebook.
4. In the top of the one cell, set:

```python
os.environ.setdefault("HF_REPO_ID", "ahmadalfakeh/witness-gemma4-e2b-judge")
```

or set `HF_REPO_ID` as an environment variable before running.

5. Run the single cell.

## Memory behavior

The notebooks are optimized for a constrained Colab T4 runtime even if that makes training slower:

- GPU VRAM is used for the quantized model, LoRA adapters, activations, and training tensors.
- System RAM/disk are used for dataset rows, tokenizer files, dataloader buffers, checkpoints, metrics, archives, and upload/download plumbing.
- CUDA is required. The cell raises an error if CUDA is missing.
- The cell checks model parameter placement and raises an error if the model is not on CUDA.

Memory-safe defaults:

```python
WITNESS_MAX_SEQ_LENGTH=1024
WITNESS_BATCH_SIZE=1
WITNESS_GRAD_ACCUM=8
WITNESS_LORA_RANK=8
WITNESS_LORA_ALPHA=16
WITNESS_MAX_STEPS=300
WITNESS_VAL_LIMIT=300
WITNESS_SAVE_MERGED=0
```

`WITNESS_SAVE_MERGED=0` is intentional. A merged 16-bit model can exceed Colab T4/system RAM limits. The default upload is the LoRA adapter plus tokenizer, metrics, model card, and validation outputs. If you really need a merged model and have enough memory, set `WITNESS_SAVE_MERGED=1` before running.

## Hugging Face upload

The one cell requires:

- `HF_TOKEN` from Colab Secrets or environment.
- `HF_REPO_ID`, for example `ahmadalfakeh/witness-gemma4-e2b-judge`.

The cell calls `create_repo(..., exist_ok=True)` and `HfApi.upload_folder(...)` using the token. When complete, it prints:

```text
DONE: uploaded to https://huggingface.co/<HF_REPO_ID>
```

## Use the uploaded model locally

```bash
cd /home/admin/Gemma/witness
hf download ahmadalfakeh/witness-gemma4-e2b-judge --local-dir models/witness-gemma4-e2b-judge
./target/debug/the-witness model test --backend unsloth --model ./models/witness-gemma4-e2b-judge
```

## Honest status

The notebooks are validated locally for JSON structure and Python syntax, and the project tests pass. They have not been executed on a live Colab T4 from this machine. The actual fine-tuned model exists only after you run the one-cell notebook in Colab and the Hugging Face upload succeeds.
