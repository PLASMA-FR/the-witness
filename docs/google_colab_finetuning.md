# Google Colab TPU fine-tuning guide

The Witness fine-tuning notebooks are Colab-first and now include a Google TPU path. Kaggle remains an optional artifact export/download path, but the recommended training runtime is Google Colab with TPU. A CUDA GPU fallback remains available for Unsloth.

Important honesty note: Unsloth and bitsandbytes are CUDA/GPU-oriented and do not run on TPU. The TPU path uses Transformers + PEFT LoRA + PyTorch/XLA. The GPU fallback path still uses Unsloth LoRA/QLoRA.

## Notebooks

- `training/notebooks/finetune_gemma4_e2b_unsloth.ipynb` — recommended first run; TPU path supported.
- `training/notebooks/finetune_gemma4_e4b_unsloth.ipynb` — stronger/larger optional run; may need more TPU memory.

## Colab TPU quick start

1. Open Google Colab.
2. Upload one of the notebooks, or open it from the GitHub repo.
3. Select `Runtime -> Change runtime type -> TPU`.
4. Optional but recommended: set the accelerator explicitly before the install cell:

```python
import os
os.environ["WITNESS_ACCELERATOR"] = "tpu"
```

5. Run the package install cell. It installs common packages and verifies/installs `torch_xla` for TPU.
6. Let the setup cell clone this repo into `/content/the-witness`, or upload the repo/dataset manually.
7. Verify the config cell:

```python
BASE_MODEL = os.environ.get("GEMMA4_E2B_BASE", "google/gemma-4-e2b")
OUTPUT_DIR = /content/witness_outputs/witness-gemma4-e2b-judge
TRAIN_FILE = /content/the-witness/training/dataset/witness_judge_train.jsonl
VAL_FILE = /content/the-witness/training/dataset/witness_judge_val.jsonl
ACCELERATOR = "tpu"
```

The base model ID is configurable because exact public Gemma 4 IDs may vary.

## GPU fallback

If Colab TPU package compatibility fails, switch to GPU and set:

```python
import os
os.environ["WITNESS_ACCELERATOR"] = "gpu"
```

The GPU path uses Unsloth + bitsandbytes QLoRA. The TPU path does not.

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

For a fast TPU sanity run:

```python
import os
os.environ["WITNESS_ACCELERATOR"] = "tpu"
os.environ["WITNESS_TRAIN_LIMIT"] = "100"
os.environ["WITNESS_VAL_LIMIT"] = "25"
os.environ["WITNESS_MAX_STEPS"] = "10"
os.environ["WITNESS_MAX_SEQ_LENGTH"] = "512"
os.environ["WITNESS_BATCH_SIZE"] = "1"
os.environ["WITNESS_LORA_RANK"] = "8"
```

For the real run, leave dataset limits unset or `0`, and increase steps as TPU memory allows:

```python
import os
os.environ["WITNESS_ACCELERATOR"] = "tpu"
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

The repo contains Colab TPU/GPU-ready notebooks and dataset. The fine-tuned model is not trained until you run a notebook and publish or copy the output artifact. I validated notebook JSON and project tests locally, but I cannot execute a real Google TPU runtime from this machine.
