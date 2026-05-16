# The Witness Unsloth Training

This folder contains dataset, scripts, notebooks, and model cards for fine-tuning Gemma 4 E2B/E4B judges with Unsloth.

Primary runtime: Google Colab with a GPU.

Kaggle is still supported as an optional artifact upload/download path after training, but the notebooks are now Colab-first.

Recommended flow:

1. Open `training/notebooks/finetune_gemma4_e2b_unsloth.ipynb` in Google Colab.
2. Select a GPU runtime.
3. Let the setup cell clone the repo into `/content/the-witness`, or upload the dataset manually.
4. For a smoke test, set `WITNESS_TRAIN_LIMIT`, `WITNESS_VAL_LIMIT`, and `WITNESS_MAX_STEPS` to small values.
5. Run all cells.
6. Download the zip artifact, copy the output from Google Drive, or upload to Hugging Face Hub.
7. Test the resulting local model path with The Witness.

Validate the dataset before training:

```bash
python3 training/scripts/validate_dataset.py
```

Full guide:

```text
docs/google_colab_finetuning.md
```
