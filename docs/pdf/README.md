# The Witness PDF documentation

This directory contains the generated PDF documentation booklet for The Witness.

## Final PDF

`docs/pdf/The_Witness_Documentation.pdf`

## Source files

- `The_Witness_Documentation.md` — portable Markdown source text.
- `source.html` — print-ready HTML layout used for the final PDF.
- `style.css` — custom visual design and print CSS.
- `generate_documentation.py` — helper script that regenerates the Markdown, HTML, CSS, and README source files.

## Generation tool

The PDF was generated with headless Chromium using CSS print layout:

```bash
chromium --headless --no-sandbox --disable-gpu \
  --print-to-pdf=docs/pdf/The_Witness_Documentation.pdf \
  file:///home/admin/Gemma/witness/docs/pdf/source.html
```

Chromium was used because it supports modern HTML/CSS layouts, local images, page breaks, and high-quality PDF printing.

## Regenerate

From the project root:

```bash
cd /home/admin/Gemma/witness
python3 docs/pdf/generate_documentation.py
chromium --headless --no-sandbox --disable-gpu \
  --print-to-pdf=docs/pdf/The_Witness_Documentation.pdf \
  file:///home/admin/Gemma/witness/docs/pdf/source.html
```

## Verify

```bash
ls -lh docs/pdf/The_Witness_Documentation.pdf
pdfinfo docs/pdf/The_Witness_Documentation.pdf | grep Pages || true
python3 - <<'PY'
from pathlib import Path
for p in Path('docs/pdf').rglob('*'):
    if p.is_file() and p.suffix not in ('.png', '.jpg', '.mp4'):
        text = p.read_text(errors='ignore')
        assert 'REAL_SECRET_PLACEHOLDER_DO_NOT_USE' not in text
print('docs/pdf secret placeholder scan ok')
PY
cargo fmt
cargo test
cargo build
```

The documentation intentionally uses environment variables such as `$BLACKBOX_API_KEY` and never includes real credentials.
