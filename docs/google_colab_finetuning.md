# Google Colab T4 GPU fine-tuning guide

The Witness fine-tuning notebooks are Colab-first and now target the Google Colab T4 GPU runtime. Kaggle remains an optional artifact export/download path, but the recommended training runtime is Google Colab with T4 GPU.

Important memory note: the notebooks are designed to use both GPU VRAM and system RAM. Unsloth + bitsandbytes 4-bit QLoRA put the model, LoRA adapters, activations, and training tensors on CUDA GPU VRAM. The dataset, tokenizer files, Python process, dataloader buffers, checkpoints, metrics, archives, and Drive/Hugging Face upload plumbing use system RAM and disk. The notebook fails fast if CUDA is unavailable so it does not accidentally train using only CPU/system RAM.

## Notebooks

- `training/notebooks/finetune_gemma4_e2b_unsloth.ipynb` — recommended first run on Colab T4 GPU.
- `training/notebooks/finetune_gemma4_e4b_unsloth.ipynb` — stronger/larger optional run; may need lower sequence length/batch on T4.

## Colab T4 GPU quick start

1. Open Google Colab.
2. Upload one of the notebooks, or open it from the GitHub repo.
3. Select `Runtime -> Change runtime type -> T4 GPU`.
4. Run the package install/check cell.
5. Confirm the cell prints:
   - CUDA GPU name
   - GPU VRAM free/total
   - system RAM available/total
6. Let the setup cell clone this repo into `/content/the-witness`, or upload the repo/dataset manually.
7. Verify the config cell:

```python
BASE_MODEL = os.environ.get("GEMMA4_E2B_BASE", "google/gemma-4-e2b")
OUTPUT_DIR = /content/witness_outputs/witness-gemma4-e2b-judge
TRAIN_FILE = /content/the-witness/training/dataset/witness_judge_train.jsonl
VAL_FILE = /content/the-witness/training/dataset/witness_judge_val.jsonl
ACCELERATOR = "t4-gpu"
```

The base model ID is configurable because exact public Gemma 4 IDs may vary.

## Memory behavior

The training cell verifies that model parameters are on CUDA:

```python
param_device = next(model.parameters()).device
if param_device.type != "cuda":
    raise RuntimeError(...)
```

This catches the bad case where the notebook silently falls back to CPU/system RAM. It also prints CUDA VRAM after model load and system RAM after dataset/model setup.

If T4 VRAM is tight, lower:

```python
os.environ["WITNESS_MAX_SEQ_LENGTH"] = "1024"
os.environ["WITNESS_BATCH_SIZE"] = "1"
os.environ["WITNESS_LORA_RANK"] = "8"
```

## Optional Google Drive persistence

To save outputs directly to Drive, set this before the setup cell:

```python
import os
os.environ["WITNESS_MOUNT_DRIVE"] = "1"
```

Then run the setup cell. If Drive is mounted, outputs default to:

```text
/content/drive/MyDrive/the-witness/outputs/<model-name>
```

Otherwise outputs default to:

```text
/content/witness_outputs/<model-name>
```

## Smoke test settings

For a fast T4 sanity run:

```python
import os
os.environ["WITNESS_TRAIN_LIMIT"] = "200"
os.environ["WITNESS_VAL_LIMIT"] = "50"
os.environ["WITNESS_MAX_STEPS"] = "20"
os.environ["WITNESS_MAX_SEQ_LENGTH"] = "1024"
os.environ["WITNESS_BATCH_SIZE"] = "1"
os.environ["WITNESS_LORA_RANK"] = "8"
```

For the real run, leave dataset limits unset or `0`, and increase steps as memory allows:

```python
import os
os.environ["WITNESS_TRAIN_LIMIT"] = "0"
os.environ["WITNESS_VAL_LIMIT"] = "0"
os.environ["WITNESS_MAX_STEPS"] = "300"
```

## Optional Hugging Face Hub upload

The notebooks include an optional Hugging Face upload cell.

1. Create a write token in Hugging Face.
2. Add it to Colab Secrets as `HF_TOKEN`.
3. Set a target repo:

```python
import os
os.environ["HF_REPO_ID"] = "your-name/witness-gemma4-e2b-judge"
```

4. Run the upload cell.

If `HF_TOKEN` or `HF_REPO_ID` is missing, the cell creates a zip archive instead. Download it from Colab Files or copy it from Drive.

## Use the trained output in The Witness

### From a downloaded zip or Drive folder

```bash
cd /home/admin/Gemma/witness
mkdir -p models/witness-gemma4-e2b-judge
# Copy Colab output files into models/witness-gemma4-e2b-judge
./target/debug/the-witness model test --backend unsloth --model ./models/witness-gemma4-e2b-judge
```

### From Hugging Face Hub

```bash
hf download your-name/witness-gemma4-e2b-judge --local-dir models/witness-gemma4-e2b-judge
./target/debug/the-witness model test --backend unsloth --model ./models/witness-gemma4-e2b-judge
```

### Optional Kaggle export

Kaggle is still supported for artifact distribution after Colab training:

```bash
./training/scripts/kaggle_upload_model.sh /path/to/witness-gemma4-e2b-judge witness-gemma4-e2b-judge
./target/debug/the-witness model download --source kaggle --model witness-gemma4-e2b-judge
```

## Security rules

- Do not paste real tokens into notebook cells.
- Use Colab Secrets for `HF_TOKEN`.
- Do not commit generated model files, `.env`, `kaggle.json`, `.safetensors`, `.gguf`, or zip archives.
- Keep trained artifacts in Drive, Hugging Face Hub, Kaggle, or another model registry.

## Honest status

The repo contains Colab T4 GPU-ready notebooks and dataset. The fine-tuned model is not trained until you run a notebook and publish or copy the output artifact. I validated notebook JSON and project tests locally, but I cannot execute a real Google Colab T4 runtime from this machine.
