# Kaggle CLI for The Witness

The confirmed fine-tuned Witness model slug is:

```text
plasmafr/witness-gemma4-e2b-judge
```

Kaggle CLI must be authenticated locally. Do not ask The Witness to store or print tokens, and never commit `kaggle.json`.

Install CLI if needed:

```bash
python -m pip install kaggle
```

If credentials are not already configured, create an API token in Kaggle Account settings and place it outside the repo:

```bash
mkdir -p ~/.kaggle
cp kaggle.json ~/.kaggle/kaggle.json
chmod 600 ~/.kaggle/kaggle.json
```

Run notebook on Kaggle:

- training/notebooks/finetune_gemma4_e2b_unsloth.ipynb

Upload the trained model:

```bash
./training/scripts/kaggle_upload_model.sh training/outputs/witness-gemma4-e2b-judge witness-gemma4-e2b-judge
```

Download from The Witness:

```bash
the-witness model download --source kaggle --model witness-gemma4-e2b-judge
```

The download uses `plasmafr/witness-gemma4-e2b-judge`, places files in `./models/witness-gemma4-e2b-judge`, verifies recognizable model files, then marks the registry entry installed.
