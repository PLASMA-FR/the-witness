# Fine-tuning notebooks

These are real Jupyter notebooks for fine-tuning The Witness Gemma 4 judge with Unsloth.

Primary runtime: Google Colab with a GPU.

Status: notebook and pipeline ready. Training/upload pending until the user runs a notebook and publishes or copies the trained artifact.

## Files

- `finetune_gemma4_e2b_unsloth.ipynb` — Colab-first notebook that fine-tunes Gemma 4 E2B as `witness-gemma4-e2b-judge`.
- `finetune_gemma4_e4b_unsloth.ipynb` — Colab-first notebook that fine-tunes Gemma 4 E4B as `witness-gemma4-e4b-judge`.

## Dataset size

The bundled dataset is now larger than 10 MB:

- train: 12,000 rows, about 10.9 MB
- validation: 1,500 rows, about 1.37 MB
- total: about 12.28 MB / 11.71 MiB

Regenerate/validate it with:

```bash
cd /home/admin/Gemma/witness
python3 training/scripts/prepare_dataset.py
python3 training/scripts/validate_dataset.py
wc -c training/dataset/witness_judge_train.jsonl training/dataset/witness_judge_val.jsonl
```

## Recommended Colab path

Use E2B first:

1. Open `finetune_gemma4_e2b_unsloth.ipynb` in Google Colab.
2. Select `Runtime -> Change runtime type -> GPU`.
3. Run the setup cell. It clones this repo into `/content/the-witness` if the dataset is not already present.
4. Optional: set `WITNESS_MOUNT_DRIVE=1` before setup to save outputs to Google Drive.
5. Verify the editable base model ID:
   - `BASE_MODEL = os.environ.get("GEMMA4_E2B_BASE", "google/gemma-4-e2b")`
6. For a smoke test, set:

   ```python
   import os
   os.environ["WITNESS_TRAIN_LIMIT"] = "200"
   os.environ["WITNESS_VAL_LIMIT"] = "50"
   os.environ["WITNESS_MAX_STEPS"] = "20"
   ```

7. For the real run, leave the limits at `0` or unset and increase training steps, for example:

   ```python
   import os
   os.environ["WITNESS_MAX_STEPS"] = "300"
   ```

8. Run all cells.
9. Confirm outputs exist:
   - adapter/model files
   - tokenizer files
   - `metrics.json`
   - `validation_predictions.jsonl`
   - README/model card
   - zip archive
10. Download the zip, copy from Google Drive, or upload to Hugging Face Hub with the optional upload cell.
11. On The Witness machine, test the copied model path:

```bash
cd /home/admin/Gemma/witness
hf download your-name/witness-gemma4-e2b-judge --local-dir models/witness-gemma4-e2b-judge
./target/debug/the-witness model test --backend unsloth --model ./models/witness-gemma4-e2b-judge
```

Kaggle remains optional after Colab training:

```bash
./training/scripts/kaggle_upload_model.sh /path/to/witness-gemma4-e2b-judge witness-gemma4-e2b-judge
./target/debug/the-witness model download --source kaggle --model witness-gemma4-e2b-judge
```

See `docs/google_colab_finetuning.md` and `docs/user_completion_guide.md` for the full completion flow.
