#!/usr/bin/env python3
import os, shutil
from pathlib import Path
src=Path(os.environ.get("ADAPTER_DIR","training/outputs/witness-gemma4-e2b-judge/adapter"))
dst=Path(os.environ.get("EXPORT_DIR","training/outputs/exported-model"))
dst.mkdir(parents=True,exist_ok=True)
if src.exists(): shutil.copytree(src,dst,dirs_exist_ok=True)
(dst/"README.md").write_text("# Exported Witness judge model\n")
print("exported", dst)
