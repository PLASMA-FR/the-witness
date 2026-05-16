# Fine-tuning notebooks

These are real Jupyter notebooks for fine-tuning The Witness Gemma 4 judge.

Primary runtime: Google Colab TPU.

Fallback runtime: Google Colab GPU with Unsloth.

Important: Unsloth/bitsandbytes are CUDA-oriented and do not run on TPU. The notebooks use Transformers + PEFT LoRA + PyTorch/XLA for TPU and Unsloth LoRA/QLoRA for GPU.

Status: notebook and pipeline ready. Training/upload pending until the user runs a notebook and publishes or copies the trained artifact.

## Files

- `finetune_gemma4_e2b_unsloth.ipynb` — Colab TPU/GPU notebook that fine-tunes Gemma 4 E2B as `witness-gemma4-e2b-judge`.
- `finetune_gemma4_e4b_unsloth.ipynb` — Colab TPU/GPU notebook that fine-tunes Gemma 4 E4B as `witness-gemma4-e4b-judge`.

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

## Recommended Colab TPU path

Use E2B first:

1. Open `finetune_gemma4_e2b_unsloth.ipynb` in Google Colab.
2. Select `Runtime -> Change runtime type -> TPU`.
3. Set this if auto-detection is wrong:

   ```python
   import os
   os.environ["WITNESS_ACCELERATOR"] = "tpu"
   ```

4. Run the setup cell. It clones this repo into `/content/the-witness` if the dataset is not already present.
5. Optional: set `WITNESS_MOUNT_DRIVE=1` before setup to save outputs to Google Drive.
6. Verify the editable base model ID:
   - `BASE_MODEL = os.environ.get("GEMMA4_E2B_BASE", "google/gemma-4-e2b")`
7. For a smoke test, set:

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

8. For the real run, leave the limits at `0` or unset and increase training steps as TPU memory allows, for example:

   ```python
   import os
   os.environ["WITNESS_ACCELERATOR"] = "tpu"
   os.environ["WITNESS_MAX_STEPS"] = "300"
   ```

9. Run all cells.
10. Confirm outputs exist:
   - adapter/model files
   - tokenizer files
   - `metrics.json`
   - `validation_predictions.jsonl`
   - README/model card
   - zip archive
11. Download the zip, copy from Google Drive, or upload to Hugging Face Hub with the optional upload cell.
12. On The Witness machine, test the copied model path:

```bash
cd /home/admin/Gemma/witness
hf download your-name/witness-gemma4-e2b-judge --local-dir models/witness-gemma4-e2b-judge
./target/debug/the-witness model test --backend unsloth --model ./models/witness-gemma4-e2b-judge
```

## GPU fallback

If TPU setup fails, switch Colab to a GPU runtime and set:

```python
import os
os.environ["WITNESS_ACCELERATOR"] = "gpu"
```

That path uses Unsloth + bitsandbytes QLoRA.

Kaggle remains optional after Colab training:

```bash
./training/scripts/kaggle_upload_model.sh /path/to/witness-gemma4-e2b-judge witness-gemma4-e2b-judge
./target/debug/the-witness model download --source kaggle --model witness-gemma4-e2b-judge
```

See `docs/google_colab_finetuning.md` and `docs/user_completion_guide.md` for the full completion flow.
