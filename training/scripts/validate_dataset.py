#!/usr/bin/env python3
import json, sys
from collections import Counter
from pathlib import Path
REQ={"endpoint_name","profile","strictness","system_prompt","user_prompt","candidate_response","verdict","confidence","safety_score","usefulness_score","prompt_alignment_score","correctness_risk","rejection_reason","suggested_fix","improved_prompt_instruction"}
VERDICTS={"APPROVED","DISAPPROVED","NEEDS_HUMAN_REVIEW"}; RISKS={"low","medium","high"}
def validate(path):
    counts=Counter(); errors=[]
    for i,line in enumerate(Path(path).read_text().splitlines(),1):
        try: row=json.loads(line)
        except Exception as e: errors.append(f"{path}:{i}: invalid JSON {e}"); continue
        miss=REQ-set(row); 
        if miss: errors.append(f"{path}:{i}: missing {sorted(miss)}")
        if row.get('verdict') not in VERDICTS: errors.append(f"{path}:{i}: bad verdict")
        if row.get('correctness_risk') not in RISKS: errors.append(f"{path}:{i}: bad risk")
        for k in ('confidence','safety_score','usefulness_score','prompt_alignment_score'):
            v=row.get(k)
            if not isinstance(v,(int,float)): errors.append(f"{path}:{i}: {k} not numeric")
        if row.get('verdict')=='DISAPPROVED' and not row.get('rejection_reason'): errors.append(f"{path}:{i}: disapproved needs reason")
        target={k:row.get(k) for k in ['verdict','confidence','safety_score','usefulness_score','prompt_alignment_score','correctness_risk','rejection_reason','suggested_fix','improved_prompt_instruction']}
        target['requires_human_review']=bool(row.get('requires_human_review', row.get('verdict')=='NEEDS_HUMAN_REVIEW'))
        json.dumps(target)
        counts[row.get('verdict')]+=1
    return counts, errors
all_counts=Counter(); all_errors=[]
for p in sys.argv[1:] or ['training/dataset/witness_judge_train.jsonl','training/dataset/witness_judge_val.jsonl']:
    c,e=validate(p); all_counts.update(c); all_errors+=e
if all_counts['NEEDS_HUMAN_REVIEW']==0: all_errors.append('no human review cases')
if all_counts['APPROVED']==0 or all_counts['DISAPPROVED']==0: all_errors.append('approved/disapproved examples missing')
if all_errors:
    print('\n'.join(all_errors)); sys.exit(1)
print('Dataset valid:', dict(all_counts))
