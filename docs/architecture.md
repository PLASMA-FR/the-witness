# Architecture

Application / AI Tool -> The Witness Local Proxy -> Upstream Endpoint -> Candidate Response -> Optional LiteRT Prefilter -> Gemma Judge Backend (Ollama, llama.cpp, Unsloth, Manual) -> Verdict.

APPROVED responses are returned. DISAPPROVED responses are blocked, repaired, retried, and logged. NEEDS_HUMAN_REVIEW responses are paused for review/fallback.
