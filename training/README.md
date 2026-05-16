# The Witness Training

This folder contains dataset, scripts, notebooks, and model cards for fine-tuning Gemma 4 E2B/E4B judges.

Primary runtime: Google Colab TPU.

Fallback runtime: Google Colab GPU with Unsloth.

Important: Unsloth/bitsandbytes are CUDA-oriented and do not run on TPU. The notebooks use a TPU-compatible Transformers + PEFT LoRA + PyTorch/XLA path when `WITNESS_ACCELERATOR=tpu`, and use Unsloth only when `WITNESS_ACCELERATOR=gpu`.

Kaggle is still supported as an optional artifact upload/download path after training, but the notebooks are now Colab TPU-first.

Recommended flow:

1. Open `training/notebooks/finetune_gemma4_e2b_unsloth.ipynb` in Google Colab.
2. Select a TPU runtime.
3. Set `WITNESS_ACCELERATOR=tpu` if auto-detection is wrong.
4. Let the setup cell clone the repo into `/content/the-witness`, or upload the dataset manually.
5. For a smoke test, set `WITNESS_TRAIN_LIMIT`, `WITNESS_VAL_LIMIT`, `WITNESS_MAX_STEPS`, and small TPU-safe sequence/batch values.
6. Run all cells.
7. Download the zip artifact, copy the output from Google Drive, or upload to Hugging Face Hub.
8. Test the resulting local model path with The Witness.

Validate the dataset before training:

```bash
python3 training/scripts/validate_dataset.py
```

Full guide:

```text
docs/google_colab_finetuning.md
```
