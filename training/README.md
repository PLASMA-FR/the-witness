# The Witness Training

This folder contains dataset, scripts, notebooks, and model cards for fine-tuning Gemma 4 E2B/E4B judges.

Primary runtime: Google Colab T4 GPU with Unsloth 4-bit LoRA/QLoRA.

Memory behavior: the notebooks use GPU VRAM for the quantized model, LoRA adapters, activations, and training tensors; they use Colab system RAM/disk for dataset rows, tokenizer files, dataloader buffers, checkpoints, metrics, archives, and upload/download plumbing. The notebooks fail fast if CUDA is unavailable so they do not silently train on CPU/system RAM only.

Kaggle is still supported as an optional artifact upload/download path after training, but the notebooks are Colab T4 GPU-first.

Recommended flow:

1. Open `training/notebooks/finetune_gemma4_e2b_unsloth.ipynb` in Google Colab.
2. Select `Runtime -> Change runtime type -> T4 GPU`.
3. Run the install/check cell and confirm it prints GPU VRAM and system RAM.
4. Let the setup cell clone the repo into `/content/the-witness`, or upload the dataset manually.
5. For a smoke test, set `WITNESS_TRAIN_LIMIT`, `WITNESS_VAL_LIMIT`, `WITNESS_MAX_STEPS`, and small T4-safe sequence/batch values.
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
