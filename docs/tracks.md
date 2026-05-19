# Hackathon Track Fit

Who this is for: reviewers who want to understand where The Witness fits.

What you will do: see the primary impact track, the four Gemma technology paths, and the boundaries of what is claimed.

## Primary impact track: Safety & Trust

The Witness is submitted primarily for Safety & Trust because it makes AI outputs more transparent, explainable, auditable, and controllable before they reach users or agents.

It does this by watching AI endpoints, judging candidate responses with Gemma 4, blocking unsafe or incorrect answers, repairing prompts, retrying, pausing risky cases for human review, and saving an audit trail.

## Gemma technology paths

| Technology path | How The Witness uses it | Setup note |
|---|---|---|
| Ollama | Default local Gemma judge path. | Pull `gemma4:e2b`; optionally pull `gemma4:e4b` for stricter/high-risk profiles. |
| llama.cpp | Local inference path for resource-constrained machines. | Configure a llama.cpp server URL or compatible model path, then run model tests. |
| LiteRT | Edge prefilter path for fast approval classification before escalating to the full judge. | Requires a LiteRT-compatible model/runtime in the target environment. |
| Unsloth | Fine-tuned Witness judge workflow. | Public Colab notebook and Hugging Face adapter are provided. |

Fine-tuned model:

```text
https://huggingface.co/ahmadalfakeh/witness-gemma4-e2b-judge
```

Fine-tuning notebook:

```text
https://colab.research.google.com/drive/17-CgEQLNg8bpnhhWzJwpapRxQyHIqybq?usp=sharing
```

## What is not claimed

Cactus is not claimed in this submission. The project focuses on Ollama, llama.cpp, LiteRT, and Unsloth.

The Witness is designed as a risk-reduction and verification layer. It does not guarantee correctness, and high-risk domains still require qualified human judgment.
