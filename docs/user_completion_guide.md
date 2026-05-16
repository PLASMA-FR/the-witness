# Completing The Witness: Fine-tune, Upload, Run, and Test

This guide is the user-side completion checklist for The Witness after the Rust code has been built and tested.

Important honesty note: the Unsloth model is not fine-tuned or uploaded until you run the notebook and complete the Kaggle upload. Current status is: notebook and pipeline ready, training/upload pending.

## 1. Current project status

What is ready:

- Rust code builds and tests.
- The TUI exists.
- The first-run setup/model pipeline exists.
- The proxy, judge schema, prompt repair, logs, model registry, and Blackbox endpoint shortcut exist.
- Fine-tuning notebooks exist:
  - `training/notebooks/finetune_gemma4_e2b_unsloth.ipynb`
  - `training/notebooks/finetune_gemma4_e4b_unsloth.ipynb`
- The fine-tuning dataset is now larger than 10 MB:
  - train: 12,000 rows, about 10.9 MB
  - validation: 1,500 rows, about 1.37 MB
  - total: about 12.28 MB / 11.71 MiB
- The fine-tuned model registry entry points at `plasmafr/witness-gemma4-e2b-judge`.

What is not done until you do it:

- The fine-tuned model is not trained.
- The fine-tuned model is not uploaded.
- The fine-tuned model is not downloadable/testable until it exists on Kaggle and credentials are configured.
- Ollama models must be pulled locally.
- Blackbox testing requires `BLACKBOX_API_KEY` in your shell environment.

## 2. Step 1 — Install local requirements

From the project root:

```bash
cd /home/admin/Gemma/witness
cargo build
```

Optional but recommended for local judging:

```bash
ollama pull gemma4:e2b
ollama pull gemma4:e4b
```

Use `gemma4:e2b` as the default fast judge. Use `gemma4:e4b` for stronger coding/high-risk profiles if your hardware can run it.

## 3. Step 2 — Run The Witness doctor

```bash
./target/debug/the-witness doctor
```

Failures that are normal before setup:

- Ollama missing.
- Ollama not running.
- `gemma4:e2b` missing.
- `gemma4:e4b` missing/optional.
- Kaggle credentials missing.
- Fine-tuned model not downloaded.
- `BLACKBOX_API_KEY` not set.
- Judge schema/model/proxy setup flags not passed.

Doctor is supposed to fail readiness until the selected judge backend, model test, proxy test, and credentials are configured or demo mode is selected.

## 4. Step 3 — Run demo mode without external APIs

If the end-to-end demo script is available:

```bash
bash scripts/e2e_demo.sh
```

If the script is unavailable or you only want to test the deterministic demo judge:

```bash
./target/debug/the-witness model test --backend demo --model demo-judge
```

Expected result:

- The intentionally bad answer is rejected.
- The clearly correct answer is approved.
- The JSON schema is validated.
- Logs are created under the project log path.

## 5. Step 4 — Run the TUI

```bash
./target/debug/the-witness setup
./target/debug/the-witness start
```

Expected setup flow:

1. The first-run setup wizard opens if setup is incomplete.
2. Choose `Ollama` for local default judging.
3. Choose `gemma4:e2b` for the default fast model.
4. Use `gemma4:e4b` for stronger/high-risk mode if available.
5. Use `demo` judge only for demos/testing.
6. Keep fallback mode as `human_review` for safety.
7. Run judge capability and proxy tests before using real endpoints.

## 6. Step 5 — Add a watched endpoint

Blackbox shortcut:

```bash
export BLACKBOX_API_KEY="YOUR_KEY_HERE"
./target/debug/the-witness endpoint add-blackbox
```

Manual TUI steps:

1. Open The Witness TUI.
2. Go to Endpoint Watchlist.
3. Add endpoint.
4. Use:
   - endpoint name: `Blackbox Grok Code`
   - upstream URL: `https://api.blackbox.ai/v1`
   - local proxy URL: `http://localhost:8787/v1`
   - model: `blackboxai/x-ai/grok-code-fast-1:free`
   - profile: `coding`
   - strictness: `high`
   - retry limit: `4`
   - fallback: `human_review`
   - auth: `bearer_env BLACKBOX_API_KEY`

Never put the key itself in config files, docs, logs, or screenshots.

## 7. Step 6 — Test Blackbox directly

Set the environment variable only in your shell:

```bash
export BLACKBOX_API_KEY="YOUR_KEY_HERE"
```

Direct upstream sanity test:

```bash
curl https://api.blackbox.ai/v1/chat/completions \
  -H "Authorization: Bearer $BLACKBOX_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "blackboxai/x-ai/grok-code-fast-1:free",
    "messages": [
      {
        "role": "user",
        "content": "Write a Python script that prints Hello World"
      }
    ]
  }'
```

If this fails, fix Blackbox credentials/connectivity before testing the proxy.

## 8. Step 7 — Test through The Witness

Start The Witness/proxy first, then send:

```bash
curl http://localhost:8787/v1/chat/completions \
  -H "Authorization: Bearer $BLACKBOX_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "blackboxai/x-ai/grok-code-fast-1:free",
    "messages": [
      {
        "role": "user",
        "content": "Write a Python script that prints Hello World"
      }
    ]
  }'
```

Expected:

- The request appears in the TUI.
- The Blackbox response is captured.
- Gemma judge checks the candidate response.
- Approved response returns to curl.
- Disapproved response triggers prompt repair and retry.
- Logs are saved.

## 9. Step 8 — Fine-tune the model on Kaggle

Open this notebook on Kaggle:

```text
training/notebooks/finetune_gemma4_e2b_unsloth.ipynb
```

Optional stronger model notebook:

```text
training/notebooks/finetune_gemma4_e4b_unsloth.ipynb
```

Upload these dataset files as a Kaggle input dataset, preferably named `witness-judge-dataset`:

```text
training/dataset/witness_judge_train.jsonl
training/dataset/witness_judge_val.jsonl
```

Dataset size:

- `witness_judge_train.jsonl`: 12,000 rows, about 10.9 MB
- `witness_judge_val.jsonl`: 1,500 rows, about 1.37 MB
- total: about 12.28 MB / 11.71 MiB

If you want to regenerate or revalidate before uploading:

```bash
cd /home/admin/Gemma/witness
python3 training/scripts/prepare_dataset.py
python3 training/scripts/validate_dataset.py
wc -c training/dataset/witness_judge_train.jsonl training/dataset/witness_judge_val.jsonl
```

For a quick Kaggle smoke test, set notebook environment variables:

```bash
WITNESS_TRAIN_LIMIT=200
WITNESS_VAL_LIMIT=50
WITNESS_MAX_STEPS=20
```

For the real run, leave `WITNESS_TRAIN_LIMIT` and `WITNESS_VAL_LIMIT` unset or `0`, and increase steps, for example:

```bash
WITNESS_MAX_STEPS=300
```

In the E2B notebook, verify or edit:

```python
BASE_MODEL = os.environ.get("GEMMA4_E2B_BASE", "google/gemma-4-e2b")
OUTPUT_MODEL_NAME = "witness-gemma4-e2b-judge"
OUTPUT_DIR = "/kaggle/working/witness-gemma4-e2b-judge"
```

If `google/gemma-4-e2b` is not the correct available public ID, replace it with the currently available Gemma 4 E2B model ID.

Run all cells.

Expected outputs:

- LoRA adapter/model files.
- tokenizer files.
- optional merged model.
- `metrics.json`.
- `validation_predictions.jsonl`.
- model card.
- README.
- sample inference script.
- upload package metadata.

Do not claim the model is trained unless these outputs are produced by your notebook run.

## 10. Step 9 — Upload trained model to Kaggle

Target slug:

```text
plasmafr/witness-gemma4-e2b-judge
```

Preferred method:

- Use the Kaggle upload cells in the notebook.
- Confirm whether Kaggle model upload or dataset fallback succeeded.
- Copy the final Kaggle resource URL.

Script method from the project root, after the notebook has produced a local output directory:

```bash
cd /home/admin/Gemma/witness
./training/scripts/kaggle_upload_model.sh training/outputs/witness-gemma4-e2b-judge witness-gemma4-e2b-judge
```

If running inside Kaggle and the output directory is `/kaggle/working/witness-gemma4-e2b-judge`, use the notebook upload cells or adapt the script path accordingly.

Do not upload secrets. Do not commit model weights unless intentionally publishing them through Kaggle.

## 11. Step 10 — Download fine-tuned model into The Witness

On The Witness machine:

```bash
cd /home/admin/Gemma/witness
./target/debug/the-witness model download --source kaggle --model witness-gemma4-e2b-judge
```

Then test:

```bash
./target/debug/the-witness model test --backend unsloth --model witness-gemma4-e2b-judge
```

If this fails, common causes are:

- Kaggle credentials missing.
- Model slug not uploaded yet.
- Authenticated Kaggle account cannot access the uploaded model/dataset.
- Model files are not compatible with the local inference path.
- Local Unsloth inference server/path is not started or configured.
- The test is still pointed at the default Ollama URL instead of a configured Unsloth backend.

## 12. Step 11 — Select fine-tuned model in TUI

1. Open The Witness TUI:

```bash
./target/debug/the-witness start
```

2. Go to Model Manager or Settings.
3. Select `Fine-tuned Witness Gemma 4 E2B Judge`.
4. Download from Kaggle if missing.
5. Test model.
6. Set as default judge or assign per endpoint.
7. Keep `human_review` fallback enabled.

## 13. Step 12 — Final verification checklist

Run:

```bash
cd /home/admin/Gemma/witness
cargo fmt --check
cargo test
cargo build
python3 training/scripts/validate_dataset.py
./target/debug/the-witness model list
./target/debug/the-witness doctor
```

If available:

```bash
bash scripts/e2e_demo.sh
```

Then run the Blackbox direct curl and The Witness proxy curl test from this guide.

## 14. Troubleshooting

### Ollama not installed

Install Ollama, then rerun doctor.

### Ollama not running

Start Ollama:

```bash
ollama serve
```

Then rerun:

```bash
./target/debug/the-witness doctor
```

### `gemma4:e2b` missing

```bash
ollama pull gemma4:e2b
```

### `gemma4:e4b` missing

Optional but recommended for coding/high-risk profiles:

```bash
ollama pull gemma4:e4b
```

### Kaggle credentials missing

Local method:

```bash
mkdir -p ~/.kaggle
cp kaggle.json ~/.kaggle/kaggle.json
chmod 600 ~/.kaggle/kaggle.json
```

Or set Kaggle environment variables if your environment supports them.

### Kaggle model not found

Confirm:

- You uploaded to `plasmafr/witness-gemma4-e2b-judge`.
- The authenticated account can access it.
- The registry still points to the same slug.

### `BLACKBOX_API_KEY` not set

```bash
export BLACKBOX_API_KEY="YOUR_KEY_HERE"
```

Do not put the key in files.

### Port 8787 already in use

Stop the process using the port or configure another local proxy URL/port for the endpoint.

### Judge returned invalid JSON

Try:

- Lower temperature.
- Use the demo judge to confirm the pipeline.
- Pull/test `gemma4:e2b` again.
- Use the fine-tuned judge after upload/download.
- Increase strict prompt/schema instructions.

### TUI not opening

Check whether you are in an interactive terminal. For non-interactive checks use:

```bash
./target/debug/the-witness doctor
./target/debug/the-witness model list
```

### Model test connection refused

The configured judge backend is not reachable. Start Ollama or the selected local server, check URL/port, then rerun the model test.
