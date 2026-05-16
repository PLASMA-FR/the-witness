# Fine-tuning notebooks

These are real Jupyter notebooks for fine-tuning The Witness Gemma 4 judge with Unsloth.

Status: notebook and pipeline ready. Training/upload pending until the user runs a notebook and uploads the trained artifact.

## Files

- `finetune_gemma4_e2b_unsloth.ipynb` — fine-tunes Gemma 4 E2B as `witness-gemma4-e2b-judge`.
- `finetune_gemma4_e4b_unsloth.ipynb` — fine-tunes Gemma 4 E4B as `witness-gemma4-e4b-judge`.

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

## Recommended path

Use E2B first:

1. Open `finetune_gemma4_e2b_unsloth.ipynb` on Kaggle with a GPU runtime.
2. Upload the dataset files:
   - `training/dataset/witness_judge_train.jsonl`
   - `training/dataset/witness_judge_val.jsonl`
3. Verify the editable base model ID:
   - `BASE_MODEL = os.environ.get("GEMMA4_E2B_BASE", "google/gemma-4-e2b")`
4. For a smoke test, set:

   ```bash
   WITNESS_TRAIN_LIMIT=200
   WITNESS_VAL_LIMIT=50
   WITNESS_MAX_STEPS=20
   ```

5. For the real run, leave the limits at `0` or unset and increase training steps, for example:

   ```bash
   WITNESS_MAX_STEPS=300
   ```

6. Run all cells.
7. Confirm outputs exist:
   - adapter/model files
   - tokenizer files
   - `metrics.json`
   - `validation_predictions.jsonl`
   - README/model card
8. Upload to Kaggle target:
   - `plasmafr/witness-gemma4-e2b-judge`
9. On The Witness machine, download and test:

```bash
cd /home/admin/Gemma/witness
./target/debug/the-witness model download --source kaggle --model witness-gemma4-e2b-judge
./target/debug/the-witness model test --backend unsloth --model witness-gemma4-e2b-judge
```

See `docs/user_completion_guide.md` for the full completion flow.
