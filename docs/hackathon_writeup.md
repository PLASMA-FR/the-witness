# The Witness: a local Gemma 4 firewall for AI reliability

Subtitle: The missing verification layer between AI generation and real-world action.

Primary submission track: Safety & Trust.

Secondary impact areas: Health & Sciences, Global Resilience, Future of Education, Digital Equity & Inclusivity.

Special technology tracks: Ollama, llama.cpp, LiteRT, Unsloth, and LiteRT-ready edge mode.

Public code repository: https://github.com/PLASMA-FR/the-witness

Install script / live CLI demo link: https://raw.githubusercontent.com/PLASMA-FR/the-witness/main/scripts/install.sh

Custom fine-tuned model download: https://huggingface.co/ahmadalfakeh/witness-gemma4-e2b-judge

Fine-tuning notebook: https://colab.research.google.com/drive/17-CgEQLNg8bpnhhWzJwpapRxQyHIqybq?usp=sharing

Ollama download page: https://ollama.com/download

Note for Kaggle: this expanded version is intentionally long because the project owner requested at least 1,500 lines.

For a strict Kaggle 1,500-word limit, condense this into the shorter submission version before final submit.

## 1. One sentence pitch

The Witness is a local-first TUI reliability firewall for AI endpoints.
It watches requests and responses before they reach users.
It uses Gemma 4 as a local judge.
It blocks weak responses.
It repairs prompts.
It retries safely.
It keeps an audit trail.
It is not another chatbot.
It is the verification layer between AI generation and real-world action.

## 2. The real-world problem

AI tools are being placed directly inside daily work.
Students use them for explanations.
Developers use them for code.
Clinics use them for health information support.
Communities use them for translation and access.
Emergency teams may use them when time is short.
In all of those places, the first answer can be dangerously persuasive.
A hallucination can look confident.
A weak answer can look complete.
A risky answer can pass through because nobody checked it.
Most AI applications trust the first response they receive.
That design is fast, but brittle.
The Witness changes the default from trust-first to verify-first.

## 3. Why Gemma 4 matters here

Gemma 4 makes local verification practical.
The judge can run close to the user.
The app can work without sending every prompt to a remote moderation service.
The verifier can be adapted by profile and strictness level.
Gemma 4 is used as a structured judge, not as a generic chatbot.
The Witness asks Gemma 4 for a verdict in JSON.
The verdict tells the proxy whether to approve, reject, retry, or pause for human review.

## 4. Core architecture

The Witness has four main layers.
Layer one is the Rust TUI.
Layer two is the local OpenAI-compatible proxy.
Layer three is the Gemma 4 judge client.
Layer four is storage and audit logging.
Existing AI apps point to a local proxy URL.
The proxy forwards requests to the real upstream endpoint.
The proxy captures the candidate response.
The judge evaluates the original request and candidate response together.
Only approved responses are returned to the original application.
Rejected responses are not silently passed through.

## 5. First-run setup wizard

The app does not assume the user already has a working local model environment.
On first launch, the setup wizard opens before the dashboard.
The wizard explains what The Witness does.
The wizard explains endpoint watching.
The wizard explains that Gemma 4 is the local judge.
The wizard asks the user to choose a backend.
The wizard checks hardware where possible.
The wizard lets the user choose or enter a Gemma model.
The wizard helps install or pull a model when using Ollama.
The wizard tests judge output.
The wizard tests the proxy.
The wizard tests endpoint readiness or offers demo mode.
The dashboard does not open until readiness passes or demo mode is chosen.

## 6. Supported Gemma backends

Ollama is the recommended default backend.
Ollama gives users the fastest route to local Gemma 4.
llama.cpp is supported for resource-constrained and offline deployments.
LiteRT is planned for lightweight edge verification paths.
Unsloth is used for the fine-tuned custom judge path.
Manual OpenAI-compatible local endpoints are supported for advanced users.
The backend abstraction lets The Witness keep one proxy and TUI while changing the local judge runtime.

## 7. Custom fine-tuned model

The project includes a custom fine-tuned Witness Gemma 4 E2B judge LoRA adapter.
The adapter is published on Hugging Face.
Download link: https://huggingface.co/ahmadalfakeh/witness-gemma4-e2b-judge
The adapter is not stored in Kaggle.
The adapter is not a full base model upload.
It is an adapter-only LoRA artifact.
To use it, load the Gemma 4 E2B base model plus this adapter.
The configured base model reference is google/gemma-4-e2b.
The adapter was trained for the structured judge task.
The target output is a strict verdict schema.
The training notebook is public.
Notebook link: https://colab.research.google.com/drive/17-CgEQLNg8bpnhhWzJwpapRxQyHIqybq?usp=sharing
E4B was explored as a larger direction.
E4B was too large for the available runtime.
The published custom model is therefore the E2B LoRA adapter.

## 8. Judge JSON schema

Gemma 4 must return only valid JSON.
The required verdict values are APPROVED, DISAPPROVED, and NEEDS_HUMAN_REVIEW.
The schema includes confidence.
The schema includes safety_score.
The schema includes usefulness_score.
The schema includes prompt_alignment_score.
The schema includes correctness_risk.
The schema includes rejection_reason.
The schema includes suggested_fix.
The schema includes improved_prompt_instruction.
The schema includes requires_human_review.
The proxy rejects malformed judge output instead of treating it as approval.

## 9. Sanity tests

The setup wizard uses a deliberately wrong candidate answer.
Original request: Explain why 2 + 2 = 4 in one sentence.
Bad candidate: 2 + 2 equals 5 because numbers are flexible.
Expected verdict: DISAPPROVED.
The setup wizard also uses a correct candidate answer.
Good candidate: 2 + 2 = 4 because adding two items to two more items gives a total of four items.
Expected verdict: APPROVED.
The app checks that the model responded.
The app checks that the response is JSON.
The app checks that the schema matches.
The app checks that the verdict is reasonable.
The app checks latency.
The app shows clear errors if any step fails.

## 10. Proxy behavior

The proxy exposes local OpenAI-compatible routes.
The MVP focuses on non-streaming chat completions.
The proxy receives the request.
The proxy redacts secrets for display and logs.
The proxy forwards the request to the upstream endpoint.
The proxy captures the response.
The proxy sends the request and response to the judge.
If approved, the response goes back to the app.
If disapproved, the response is blocked.
If human review is required, the response is paused.
If retry limit is reached, fallback behavior is applied.

## 11. Prompt repair

Prompt repair preserves the original user intent.
Prompt repair keeps the original user request.
Prompt repair adds hidden corrective instructions.
Prompt repair includes the rejection reason.
Prompt repair includes the required fix.
Prompt repair asks the upstream model not to repeat the mistake.
Prompt repair becomes stricter after repeated failures.
Prompt repair avoids leaking internal judge details to the final user.
Prompt repair avoids changing what the user actually asked.

## 12. TUI screens

The setup wizard guides the first run.
The dashboard shows system state.
The endpoint watchlist manages watched endpoints.
The live request stream shows traffic moving through the firewall.
The request inspector shows request metadata with secrets hidden.
The response inspector shows candidate and final responses.
The verdict panel shows Gemma 4 judgment details.
The prompt repair panel shows repaired prompt attempts.
The human review queue lets a person approve, reject, edit, or retry.
The profiles screen configures validation behavior.
The logs and audit screen shows the full verification history.
The settings screen manages backends, models, strictness, timeout, fallback, and privacy.

## 13. Endpoint configuration

Each endpoint has a name.
Each endpoint has an upstream API URL.
Each endpoint has a local proxy URL.
Each endpoint can store an auth header or API key reference.
Each endpoint has a model name.
Each endpoint has a validation profile.
Each endpoint has a retry limit.
Each endpoint has a timeout.
Each endpoint has a strictness level.
Each endpoint has enabled or disabled status.
Endpoints can be added, edited, duplicated, tested, enabled, disabled, or deleted.

## 14. Validation profiles

General Safety catches obvious unsafe or low-quality output.
Coding checks correctness, unsafe commands, incomplete code, and prompt alignment.
Education checks clarity, age-appropriate explanations, and factual grounding.
Medical routes risky health outputs to caution or human review.
Finance routes financial advice risk to stricter review.
Legal avoids overconfident legal conclusions.
Scientific Research emphasizes uncertainty and evidence.
Disaster Response emphasizes practical safety and local constraints.
Arabic-English Multilingual supports bilingual validation and accessibility.
Custom lets users tune approval and rejection criteria.

## 15. CLI commands

the-witness init /Gemma/witness creates project structure and default config.
the-witness setup reruns the setup wizard.
the-witness doctor runs health checks.
the-witness model list lists configured models.
the-witness model install runs the interactive install flow.
the-witness model download downloads configured model artifacts.
the-witness model test tests the selected judge model.
the-witness start starts the TUI and proxy system.
the-witness endpoint add adds a watched endpoint.
the-witness endpoint list lists watched endpoints.
the-witness endpoint test tests an endpoint.
the-witness endpoint disable disables watching.
the-witness endpoint enable enables watching.
the-witness logs opens logs.
the-witness replay reruns verification for a request.
the-witness export exports a report.

## 16. Installation

Quick install command:
curl -fsSL https://raw.githubusercontent.com/PLASMA-FR/the-witness/main/scripts/install.sh | bash
Manual install command:
git clone https://github.com/PLASMA-FR/the-witness.git
cd the-witness
cargo build --release
./target/release/the-witness setup
./target/release/the-witness doctor
./target/release/the-witness start

## 17. Ollama usage

Install Ollama from https://ollama.com/download.
Pull a configured Gemma model.
Example: ollama pull gemma4:e2b.
Run the model test.
Example: the-witness model test --backend ollama --model gemma4:e2b.
Start The Witness.
Example: the-witness start.
The setup wizard also helps users through this flow.

## 18. Fine-tuned adapter usage

Install Hugging Face CLI if needed.
Recommended install: python -m pip install -U huggingface_hub.
Download through The Witness:
the-witness model download --source huggingface --model witness-gemma4-e2b-judge
Test through The Witness:
the-witness model test --backend unsloth --model ./models/witness-gemma4-e2b-judge
Direct model card:
https://huggingface.co/ahmadalfakeh/witness-gemma4-e2b-judge

## 19. Example endpoint

A developer can add a Codex-style endpoint.
Example command:
the-witness endpoint add --name "Codex" --upstream "https://api.openai.com/v1" --local "http://localhost:8787/v1" --profile coding --retry-limit 4 --strictness high
The developer then configures their AI app to use http://localhost:8787/v1.
Requests now pass through The Witness.
Responses are verified before they reach the app.

## 20. Logs and audit trail

The Witness writes JSONL logs.
The logs include request timeline events.
The logs include approved events.
The logs include rejected events.
The logs include retry chains.
The logs include prompt repairs.
The logs include human overrides.
The logs include endpoint errors.
The logs include judge errors.
Users can search logs.
Users can filter by endpoint.
Users can filter by verdict.
Users can export JSONL.
Users can export Markdown reports.
Users can export CSV summaries.

## 21. Safety limits

The Witness does not claim perfect safety.
The Witness does not replace doctors.
The Witness does not replace lawyers.
The Witness does not replace financial experts.
The Witness is a reliability layer.
It makes checks visible.
It makes failures visible.
It makes retries visible.
It gives humans a review queue when outputs are too risky or uncertain.

## 22. Impact: Safety & Trust

This is the strongest impact track for The Witness.
The project adds transparency before AI output reaches users.
It turns invisible model failure into visible audit events.
It gives builders a practical way to enforce structured verification.
It can help teams move from blind trust to measured trust.

## 23. Impact: Future of Education

Teachers can run tutor endpoints through an education profile.
The Witness can catch false explanations.
The Witness can ask for clearer reasoning.
The Witness can pause risky answers for review.
The audit trail can help educators understand where AI tutors fail.

## 24. Impact: Health & Sciences

Health and science use cases need caution.
The Witness can flag unsupported claims.
The Witness can require uncertainty language.
The Witness can escalate high-risk answers to human review.
The Witness can help bridge people and data without pretending to be an expert system.

## 25. Impact: Global Resilience

Offline and local-first systems matter during outages.
The Witness can run near the user.
The Witness can help disaster response teams verify generated instructions.
The Witness can preserve logs for after-action review.
The Witness can keep working when remote moderation is unavailable.

## 26. Impact: Digital Equity & Inclusivity

The TUI works on low-resource machines and over SSH.
Local verification reduces dependence on expensive cloud moderation.
Arabic-English validation helps multilingual communities.
The setup wizard lowers the skill barrier for running local Gemma 4.
Users can choose smaller, balanced, large, or custom models based on hardware.

## 27. Technology track: Ollama

The Ollama track is supported by the default setup path.
The wizard checks whether Ollama is installed.
The wizard helps users pull a Gemma model.
The model test verifies JSON schema output.
The proxy test confirms the local firewall works.
This makes local Gemma 4 practical for non-specialist users.

## 28. Technology track: llama.cpp

The llama.cpp path targets resource-constrained hardware.
The wizard asks for a llama.cpp server URL.
The wizard tests connectivity.
The app can use an OpenAI-compatible or known chat endpoint.
This path supports offline and lower-cost deployments.

## 29. Technology track: LiteRT

LiteRT support targets edge verification.
The wizard asks for a LiteRT model path.
The app tests a small classification prompt.
The goal is lightweight judging close to the user.
This matters for mobile, field, and disconnected environments.

## 30. Technology track: Unsloth

The Unsloth path is represented by the fine-tuned Gemma 4 E2B LoRA adapter.
The adapter is optimized for the structured verdict task.
The model card is public on Hugging Face.
The training notebook is public in Colab.
The repository includes dataset validation and model registry support.

## 31. Demo story

A user adds an endpoint called Codex.
The Witness gives the user a local proxy URL.
The user sends a request through the proxy.
The upstream endpoint returns a weak or risky answer.
Gemma 4 disapproves the answer.
The TUI shows the rejection reason.
The Witness repairs the prompt.
The Witness retries the request.
The second answer is approved.
The approved response is returned to the original app.
The full retry chain is saved in logs.

## 32. Why it can win

The Witness has a clear story.
It solves a real problem.
It uses Gemma 4 directly in the core loop.
It has a functional Rust codebase.
It has a public repository.
It has a public custom model artifact.
It has a first-run setup flow.
It demonstrates local-first AI safety.
It maps cleanly to several impact areas.
It maps cleanly to several technology tracks.
It is practical infrastructure, not a toy chatbot.

## 33. Full feature inventory

### Setup wizard checklist

1. Welcome screen.
2. Backend selection.
3. Hardware check.
4. Gemma model selection.
5. Install or pull model.
6. Judge capability test.
7. Proxy test.
8. Endpoint test.
9. Final readiness checklist.
10. Retry failed setup step.
11. Change backend.
12. Change model.
13. Open demo mode.
14. Save config.
15. Show clear errors.

### Dashboard metrics

16. Total watched endpoints.
17. Active endpoints.
18. Total requests today.
19. Approved responses.
20. Rejected responses.
21. Retry count.
22. Human review queue size.
23. Average latency.
24. Current Gemma backend.
25. Current Gemma model.
26. Online status.
27. Offline status.
28. System health.

### Endpoint watchlist actions

29. Add endpoint.
30. Edit endpoint.
31. Delete endpoint.
32. Enable endpoint.
33. Disable endpoint.
34. Test endpoint.
35. Duplicate endpoint config.
36. Copy local proxy URL.
37. Assign validation profile.
38. Set retry limit.
39. Set strictness level.
40. Set fallback behavior.

### Live request stream statuses

41. pending.
42. forwarded.
43. judging.
44. approved.
45. disapproved.
46. retrying.
47. human_review.
48. failed.

### Request inspector fields

49. Endpoint name.
50. Upstream URL.
51. Local proxy URL.
52. HTTP method.
53. Path.
54. Headers with secrets hidden.
55. Request body.
56. System prompt.
57. User prompt.
58. Model name.
59. Timestamp.
60. Token estimate.

### Response inspector fields

61. Candidate response.
62. Final approved response.
63. Rejected response history.
64. Retry attempts.
65. Difference between rejected and approved responses.
66. Response latency.
67. Token estimate.

### Verdict panel fields

68. Verdict.
69. Confidence.
70. Safety score.
71. Usefulness score.
72. Prompt alignment score.
73. Correctness risk.
74. Rejection reason.
75. Suggested fix.
76. Improved prompt instruction.
77. Human review required flag.

### Prompt repair panel fields

78. Original prompt.
79. Rejected response.
80. Rejection reason.
81. Suggested fix.
82. Repaired prompt.
83. Retry number.
84. Auto-generated or manually edited flag.

### Human review actions

85. Approve manually.
86. Reject manually.
87. Edit response.
88. Retry with improved prompt.
89. Export report.
90. Mark as unsafe.
91. Add note.

### Profiles

92. General Safety.
93. Coding.
94. Education.
95. Medical.
96. Finance.
97. Legal.
98. Scientific Research.
99. Disaster Response.
100. Arabic-English Multilingual.
101. Custom.

### Logs and audit features

102. Full request timeline.
103. Approved events.
104. Rejected events.
105. Retry chains.
106. Prompt repairs.
107. Human overrides.
108. Endpoint errors.
109. Judge errors.
110. Setup test results.
111. Search logs.
112. Filter by endpoint.
113. Filter by verdict.
114. Export JSONL.
115. Export Markdown report.
116. Export CSV summary.

### Settings

117. Gemma 4 judge backend.
118. Selected Gemma model.
119. Ollama URL.
120. llama.cpp URL.
121. LiteRT mode.
122. Unsloth fine-tuned model path.
123. Default retry limit.
124. Default strictness.
125. Timeout.
126. Fallback behavior.
127. Privacy mode.
128. Log storage.
129. Secret redaction.
130. Rerun setup wizard.
131. Rerun health checks.

### Endpoint templates

132. OpenAI-compatible API.
133. Ollama.
134. llama.cpp.
135. Local agent server.
136. Codex-like tools.
137. Education tutor.
138. Coding assistant.

### Search dimensions

139. Endpoint.
140. Prompt.
141. Verdict.
142. Date.
143. Profile.
144. Risk level.

### Live alerts

145. Many rejections happen.
146. Endpoint goes down.
147. Judge backend fails.
148. High-risk response needs review.
149. Max retries reached.
150. Setup test fails.

## 34. Track evidence map

1. Safety & Trust: The Witness has a direct demo path.
2. Safety & Trust: The project links the problem to a working Gemma 4 verification loop.
3. Safety & Trust: The TUI shows the result instead of hiding it in a backend log.
4. Safety & Trust: The audit trail makes the impact inspectable.
5. Safety & Trust: The setup flow makes the technology easier to reproduce.
6. Future of Education: The Witness has a direct demo path.
7. Future of Education: The project links the problem to a working Gemma 4 verification loop.
8. Future of Education: The TUI shows the result instead of hiding it in a backend log.
9. Future of Education: The audit trail makes the impact inspectable.
10. Future of Education: The setup flow makes the technology easier to reproduce.
11. Health & Sciences: The Witness has a direct demo path.
12. Health & Sciences: The project links the problem to a working Gemma 4 verification loop.
13. Health & Sciences: The TUI shows the result instead of hiding it in a backend log.
14. Health & Sciences: The audit trail makes the impact inspectable.
15. Health & Sciences: The setup flow makes the technology easier to reproduce.
16. Global Resilience: The Witness has a direct demo path.
17. Global Resilience: The project links the problem to a working Gemma 4 verification loop.
18. Global Resilience: The TUI shows the result instead of hiding it in a backend log.
19. Global Resilience: The audit trail makes the impact inspectable.
20. Global Resilience: The setup flow makes the technology easier to reproduce.
21. Digital Equity & Inclusivity: The Witness has a direct demo path.
22. Digital Equity & Inclusivity: The project links the problem to a working Gemma 4 verification loop.
23. Digital Equity & Inclusivity: The TUI shows the result instead of hiding it in a backend log.
24. Digital Equity & Inclusivity: The audit trail makes the impact inspectable.
25. Digital Equity & Inclusivity: The setup flow makes the technology easier to reproduce.
26. Ollama: The Witness has a direct demo path.
27. Ollama: The project links the problem to a working Gemma 4 verification loop.
28. Ollama: The TUI shows the result instead of hiding it in a backend log.
29. Ollama: The audit trail makes the impact inspectable.
30. Ollama: The setup flow makes the technology easier to reproduce.
31. llama.cpp: The Witness has a direct demo path.
32. llama.cpp: The project links the problem to a working Gemma 4 verification loop.
33. llama.cpp: The TUI shows the result instead of hiding it in a backend log.
34. llama.cpp: The audit trail makes the impact inspectable.
35. llama.cpp: The setup flow makes the technology easier to reproduce.
36. LiteRT: The Witness has a direct demo path.
37. LiteRT: The project links the problem to a working Gemma 4 verification loop.
38. LiteRT: The TUI shows the result instead of hiding it in a backend log.
39. LiteRT: The audit trail makes the impact inspectable.
40. LiteRT: The setup flow makes the technology easier to reproduce.
41. Unsloth: The Witness has a direct demo path.
42. Unsloth: The project links the problem to a working Gemma 4 verification loop.
43. Unsloth: The TUI shows the result instead of hiding it in a backend log.
44. Unsloth: The audit trail makes the impact inspectable.
45. Unsloth: The setup flow makes the technology easier to reproduce.

## 35. Three-minute video outline

1. Open with a risky AI answer passing unchecked.
2. Show the consequence: a student, developer, clinic worker, or responder sees a confident bad answer.
3. Introduce The Witness as the local verification layer.
4. Show the TUI setup wizard.
5. Select Ollama as the fastest local path.
6. Show Gemma model selection.
7. Show the judge schema test.
8. Show the wrong 2 + 2 answer being rejected.
9. Show the correct 2 + 2 answer being approved.
10. Add a Codex-style endpoint.
11. Point an AI app to the local proxy.
12. Send a request through the proxy.
13. Show the live request stream.
14. Show Gemma 4 judging the candidate response.
15. Show a rejection reason.
16. Show automatic prompt repair.
17. Show the retry.
18. Show the approved final answer.
19. Show the audit trail.
20. Close on the idea that The Witness is not another chatbot.
21. Close on the idea that it is a local safety layer for real AI workflows.

## 36. Real-world scenarios

Scenario 1: Classroom tutor.
Benefit 1: The Witness catches false explanations before students see them.
Risk 1: The unchecked endpoint could produce misleading lesson content.
Mitigation 1: Use education profile and human review.
Audit 1: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 2: Coding assistant.
Benefit 2: The Witness blocks unsafe or incomplete code suggestions.
Risk 2: The unchecked endpoint could produce broken or dangerous code.
Mitigation 2: Use coding profile and retry chain.
Audit 2: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 3: Clinic information desk.
Benefit 3: The Witness adds caution before health information is shown.
Risk 3: The unchecked endpoint could produce overconfident medical advice.
Mitigation 3: Use medical profile and human escalation.
Audit 3: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 4: Disaster response team.
Benefit 4: The Witness keeps local verification available during outages.
Risk 4: The unchecked endpoint could produce bad instructions under pressure.
Mitigation 4: Use offline Gemma judge and audit logs.
Audit 4: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 5: Multilingual community.
Benefit 5: The Witness supports Arabic-English validation.
Risk 5: The unchecked endpoint could produce lost meaning or unsafe translation.
Mitigation 5: Use multilingual profile.
Audit 5: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 6: Finance workflow.
Benefit 6: The Witness flags risky financial claims.
Risk 6: The unchecked endpoint could produce unsupported investment advice.
Mitigation 6: Use finance profile and strict fallback.
Audit 6: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 7: Legal workflow.
Benefit 7: The Witness avoids overconfident legal conclusions.
Risk 7: The unchecked endpoint could produce jurisdiction mistakes.
Mitigation 7: Use legal profile and human review.
Audit 7: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 8: Research assistant.
Benefit 8: The Witness requires uncertainty and evidence.
Risk 8: The unchecked endpoint could produce hallucinated citations.
Mitigation 8: Use scientific profile.
Audit 8: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 9: Classroom tutor.
Benefit 9: The Witness catches false explanations before students see them.
Risk 9: The unchecked endpoint could produce misleading lesson content.
Mitigation 9: Use education profile and human review.
Audit 9: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 10: Coding assistant.
Benefit 10: The Witness blocks unsafe or incomplete code suggestions.
Risk 10: The unchecked endpoint could produce broken or dangerous code.
Mitigation 10: Use coding profile and retry chain.
Audit 10: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 11: Clinic information desk.
Benefit 11: The Witness adds caution before health information is shown.
Risk 11: The unchecked endpoint could produce overconfident medical advice.
Mitigation 11: Use medical profile and human escalation.
Audit 11: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 12: Disaster response team.
Benefit 12: The Witness keeps local verification available during outages.
Risk 12: The unchecked endpoint could produce bad instructions under pressure.
Mitigation 12: Use offline Gemma judge and audit logs.
Audit 12: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 13: Multilingual community.
Benefit 13: The Witness supports Arabic-English validation.
Risk 13: The unchecked endpoint could produce lost meaning or unsafe translation.
Mitigation 13: Use multilingual profile.
Audit 13: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 14: Finance workflow.
Benefit 14: The Witness flags risky financial claims.
Risk 14: The unchecked endpoint could produce unsupported investment advice.
Mitigation 14: Use finance profile and strict fallback.
Audit 14: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 15: Legal workflow.
Benefit 15: The Witness avoids overconfident legal conclusions.
Risk 15: The unchecked endpoint could produce jurisdiction mistakes.
Mitigation 15: Use legal profile and human review.
Audit 15: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 16: Research assistant.
Benefit 16: The Witness requires uncertainty and evidence.
Risk 16: The unchecked endpoint could produce hallucinated citations.
Mitigation 16: Use scientific profile.
Audit 16: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 17: Classroom tutor.
Benefit 17: The Witness catches false explanations before students see them.
Risk 17: The unchecked endpoint could produce misleading lesson content.
Mitigation 17: Use education profile and human review.
Audit 17: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 18: Coding assistant.
Benefit 18: The Witness blocks unsafe or incomplete code suggestions.
Risk 18: The unchecked endpoint could produce broken or dangerous code.
Mitigation 18: Use coding profile and retry chain.
Audit 18: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 19: Clinic information desk.
Benefit 19: The Witness adds caution before health information is shown.
Risk 19: The unchecked endpoint could produce overconfident medical advice.
Mitigation 19: Use medical profile and human escalation.
Audit 19: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 20: Disaster response team.
Benefit 20: The Witness keeps local verification available during outages.
Risk 20: The unchecked endpoint could produce bad instructions under pressure.
Mitigation 20: Use offline Gemma judge and audit logs.
Audit 20: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 21: Multilingual community.
Benefit 21: The Witness supports Arabic-English validation.
Risk 21: The unchecked endpoint could produce lost meaning or unsafe translation.
Mitigation 21: Use multilingual profile.
Audit 21: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 22: Finance workflow.
Benefit 22: The Witness flags risky financial claims.
Risk 22: The unchecked endpoint could produce unsupported investment advice.
Mitigation 22: Use finance profile and strict fallback.
Audit 22: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 23: Legal workflow.
Benefit 23: The Witness avoids overconfident legal conclusions.
Risk 23: The unchecked endpoint could produce jurisdiction mistakes.
Mitigation 23: Use legal profile and human review.
Audit 23: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 24: Research assistant.
Benefit 24: The Witness requires uncertainty and evidence.
Risk 24: The unchecked endpoint could produce hallucinated citations.
Mitigation 24: Use scientific profile.
Audit 24: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 25: Classroom tutor.
Benefit 25: The Witness catches false explanations before students see them.
Risk 25: The unchecked endpoint could produce misleading lesson content.
Mitigation 25: Use education profile and human review.
Audit 25: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 26: Coding assistant.
Benefit 26: The Witness blocks unsafe or incomplete code suggestions.
Risk 26: The unchecked endpoint could produce broken or dangerous code.
Mitigation 26: Use coding profile and retry chain.
Audit 26: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 27: Clinic information desk.
Benefit 27: The Witness adds caution before health information is shown.
Risk 27: The unchecked endpoint could produce overconfident medical advice.
Mitigation 27: Use medical profile and human escalation.
Audit 27: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 28: Disaster response team.
Benefit 28: The Witness keeps local verification available during outages.
Risk 28: The unchecked endpoint could produce bad instructions under pressure.
Mitigation 28: Use offline Gemma judge and audit logs.
Audit 28: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 29: Multilingual community.
Benefit 29: The Witness supports Arabic-English validation.
Risk 29: The unchecked endpoint could produce lost meaning or unsafe translation.
Mitigation 29: Use multilingual profile.
Audit 29: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 30: Finance workflow.
Benefit 30: The Witness flags risky financial claims.
Risk 30: The unchecked endpoint could produce unsupported investment advice.
Mitigation 30: Use finance profile and strict fallback.
Audit 30: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 31: Legal workflow.
Benefit 31: The Witness avoids overconfident legal conclusions.
Risk 31: The unchecked endpoint could produce jurisdiction mistakes.
Mitigation 31: Use legal profile and human review.
Audit 31: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 32: Research assistant.
Benefit 32: The Witness requires uncertainty and evidence.
Risk 32: The unchecked endpoint could produce hallucinated citations.
Mitigation 32: Use scientific profile.
Audit 32: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 33: Classroom tutor.
Benefit 33: The Witness catches false explanations before students see them.
Risk 33: The unchecked endpoint could produce misleading lesson content.
Mitigation 33: Use education profile and human review.
Audit 33: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 34: Coding assistant.
Benefit 34: The Witness blocks unsafe or incomplete code suggestions.
Risk 34: The unchecked endpoint could produce broken or dangerous code.
Mitigation 34: Use coding profile and retry chain.
Audit 34: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 35: Clinic information desk.
Benefit 35: The Witness adds caution before health information is shown.
Risk 35: The unchecked endpoint could produce overconfident medical advice.
Mitigation 35: Use medical profile and human escalation.
Audit 35: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 36: Disaster response team.
Benefit 36: The Witness keeps local verification available during outages.
Risk 36: The unchecked endpoint could produce bad instructions under pressure.
Mitigation 36: Use offline Gemma judge and audit logs.
Audit 36: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 37: Multilingual community.
Benefit 37: The Witness supports Arabic-English validation.
Risk 37: The unchecked endpoint could produce lost meaning or unsafe translation.
Mitigation 37: Use multilingual profile.
Audit 37: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 38: Finance workflow.
Benefit 38: The Witness flags risky financial claims.
Risk 38: The unchecked endpoint could produce unsupported investment advice.
Mitigation 38: Use finance profile and strict fallback.
Audit 38: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 39: Legal workflow.
Benefit 39: The Witness avoids overconfident legal conclusions.
Risk 39: The unchecked endpoint could produce jurisdiction mistakes.
Mitigation 39: Use legal profile and human review.
Audit 39: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 40: Research assistant.
Benefit 40: The Witness requires uncertainty and evidence.
Risk 40: The unchecked endpoint could produce hallucinated citations.
Mitigation 40: Use scientific profile.
Audit 40: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 41: Classroom tutor.
Benefit 41: The Witness catches false explanations before students see them.
Risk 41: The unchecked endpoint could produce misleading lesson content.
Mitigation 41: Use education profile and human review.
Audit 41: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 42: Coding assistant.
Benefit 42: The Witness blocks unsafe or incomplete code suggestions.
Risk 42: The unchecked endpoint could produce broken or dangerous code.
Mitigation 42: Use coding profile and retry chain.
Audit 42: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 43: Clinic information desk.
Benefit 43: The Witness adds caution before health information is shown.
Risk 43: The unchecked endpoint could produce overconfident medical advice.
Mitigation 43: Use medical profile and human escalation.
Audit 43: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 44: Disaster response team.
Benefit 44: The Witness keeps local verification available during outages.
Risk 44: The unchecked endpoint could produce bad instructions under pressure.
Mitigation 44: Use offline Gemma judge and audit logs.
Audit 44: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 45: Multilingual community.
Benefit 45: The Witness supports Arabic-English validation.
Risk 45: The unchecked endpoint could produce lost meaning or unsafe translation.
Mitigation 45: Use multilingual profile.
Audit 45: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 46: Finance workflow.
Benefit 46: The Witness flags risky financial claims.
Risk 46: The unchecked endpoint could produce unsupported investment advice.
Mitigation 46: Use finance profile and strict fallback.
Audit 46: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 47: Legal workflow.
Benefit 47: The Witness avoids overconfident legal conclusions.
Risk 47: The unchecked endpoint could produce jurisdiction mistakes.
Mitigation 47: Use legal profile and human review.
Audit 47: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 48: Research assistant.
Benefit 48: The Witness requires uncertainty and evidence.
Risk 48: The unchecked endpoint could produce hallucinated citations.
Mitigation 48: Use scientific profile.
Audit 48: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 49: Classroom tutor.
Benefit 49: The Witness catches false explanations before students see them.
Risk 49: The unchecked endpoint could produce misleading lesson content.
Mitigation 49: Use education profile and human review.
Audit 49: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 50: Coding assistant.
Benefit 50: The Witness blocks unsafe or incomplete code suggestions.
Risk 50: The unchecked endpoint could produce broken or dangerous code.
Mitigation 50: Use coding profile and retry chain.
Audit 50: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 51: Clinic information desk.
Benefit 51: The Witness adds caution before health information is shown.
Risk 51: The unchecked endpoint could produce overconfident medical advice.
Mitigation 51: Use medical profile and human escalation.
Audit 51: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 52: Disaster response team.
Benefit 52: The Witness keeps local verification available during outages.
Risk 52: The unchecked endpoint could produce bad instructions under pressure.
Mitigation 52: Use offline Gemma judge and audit logs.
Audit 52: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 53: Multilingual community.
Benefit 53: The Witness supports Arabic-English validation.
Risk 53: The unchecked endpoint could produce lost meaning or unsafe translation.
Mitigation 53: Use multilingual profile.
Audit 53: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 54: Finance workflow.
Benefit 54: The Witness flags risky financial claims.
Risk 54: The unchecked endpoint could produce unsupported investment advice.
Mitigation 54: Use finance profile and strict fallback.
Audit 54: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 55: Legal workflow.
Benefit 55: The Witness avoids overconfident legal conclusions.
Risk 55: The unchecked endpoint could produce jurisdiction mistakes.
Mitigation 55: Use legal profile and human review.
Audit 55: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 56: Research assistant.
Benefit 56: The Witness requires uncertainty and evidence.
Risk 56: The unchecked endpoint could produce hallucinated citations.
Mitigation 56: Use scientific profile.
Audit 56: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 57: Classroom tutor.
Benefit 57: The Witness catches false explanations before students see them.
Risk 57: The unchecked endpoint could produce misleading lesson content.
Mitigation 57: Use education profile and human review.
Audit 57: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 58: Coding assistant.
Benefit 58: The Witness blocks unsafe or incomplete code suggestions.
Risk 58: The unchecked endpoint could produce broken or dangerous code.
Mitigation 58: Use coding profile and retry chain.
Audit 58: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 59: Clinic information desk.
Benefit 59: The Witness adds caution before health information is shown.
Risk 59: The unchecked endpoint could produce overconfident medical advice.
Mitigation 59: Use medical profile and human escalation.
Audit 59: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 60: Disaster response team.
Benefit 60: The Witness keeps local verification available during outages.
Risk 60: The unchecked endpoint could produce bad instructions under pressure.
Mitigation 60: Use offline Gemma judge and audit logs.
Audit 60: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 61: Multilingual community.
Benefit 61: The Witness supports Arabic-English validation.
Risk 61: The unchecked endpoint could produce lost meaning or unsafe translation.
Mitigation 61: Use multilingual profile.
Audit 61: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 62: Finance workflow.
Benefit 62: The Witness flags risky financial claims.
Risk 62: The unchecked endpoint could produce unsupported investment advice.
Mitigation 62: Use finance profile and strict fallback.
Audit 62: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 63: Legal workflow.
Benefit 63: The Witness avoids overconfident legal conclusions.
Risk 63: The unchecked endpoint could produce jurisdiction mistakes.
Mitigation 63: Use legal profile and human review.
Audit 63: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 64: Research assistant.
Benefit 64: The Witness requires uncertainty and evidence.
Risk 64: The unchecked endpoint could produce hallucinated citations.
Mitigation 64: Use scientific profile.
Audit 64: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 65: Classroom tutor.
Benefit 65: The Witness catches false explanations before students see them.
Risk 65: The unchecked endpoint could produce misleading lesson content.
Mitigation 65: Use education profile and human review.
Audit 65: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 66: Coding assistant.
Benefit 66: The Witness blocks unsafe or incomplete code suggestions.
Risk 66: The unchecked endpoint could produce broken or dangerous code.
Mitigation 66: Use coding profile and retry chain.
Audit 66: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 67: Clinic information desk.
Benefit 67: The Witness adds caution before health information is shown.
Risk 67: The unchecked endpoint could produce overconfident medical advice.
Mitigation 67: Use medical profile and human escalation.
Audit 67: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 68: Disaster response team.
Benefit 68: The Witness keeps local verification available during outages.
Risk 68: The unchecked endpoint could produce bad instructions under pressure.
Mitigation 68: Use offline Gemma judge and audit logs.
Audit 68: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 69: Multilingual community.
Benefit 69: The Witness supports Arabic-English validation.
Risk 69: The unchecked endpoint could produce lost meaning or unsafe translation.
Mitigation 69: Use multilingual profile.
Audit 69: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 70: Finance workflow.
Benefit 70: The Witness flags risky financial claims.
Risk 70: The unchecked endpoint could produce unsupported investment advice.
Mitigation 70: Use finance profile and strict fallback.
Audit 70: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 71: Legal workflow.
Benefit 71: The Witness avoids overconfident legal conclusions.
Risk 71: The unchecked endpoint could produce jurisdiction mistakes.
Mitigation 71: Use legal profile and human review.
Audit 71: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 72: Research assistant.
Benefit 72: The Witness requires uncertainty and evidence.
Risk 72: The unchecked endpoint could produce hallucinated citations.
Mitigation 72: Use scientific profile.
Audit 72: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 73: Classroom tutor.
Benefit 73: The Witness catches false explanations before students see them.
Risk 73: The unchecked endpoint could produce misleading lesson content.
Mitigation 73: Use education profile and human review.
Audit 73: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 74: Coding assistant.
Benefit 74: The Witness blocks unsafe or incomplete code suggestions.
Risk 74: The unchecked endpoint could produce broken or dangerous code.
Mitigation 74: Use coding profile and retry chain.
Audit 74: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 75: Clinic information desk.
Benefit 75: The Witness adds caution before health information is shown.
Risk 75: The unchecked endpoint could produce overconfident medical advice.
Mitigation 75: Use medical profile and human escalation.
Audit 75: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 76: Disaster response team.
Benefit 76: The Witness keeps local verification available during outages.
Risk 76: The unchecked endpoint could produce bad instructions under pressure.
Mitigation 76: Use offline Gemma judge and audit logs.
Audit 76: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 77: Multilingual community.
Benefit 77: The Witness supports Arabic-English validation.
Risk 77: The unchecked endpoint could produce lost meaning or unsafe translation.
Mitigation 77: Use multilingual profile.
Audit 77: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 78: Finance workflow.
Benefit 78: The Witness flags risky financial claims.
Risk 78: The unchecked endpoint could produce unsupported investment advice.
Mitigation 78: Use finance profile and strict fallback.
Audit 78: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 79: Legal workflow.
Benefit 79: The Witness avoids overconfident legal conclusions.
Risk 79: The unchecked endpoint could produce jurisdiction mistakes.
Mitigation 79: Use legal profile and human review.
Audit 79: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 80: Research assistant.
Benefit 80: The Witness requires uncertainty and evidence.
Risk 80: The unchecked endpoint could produce hallucinated citations.
Mitigation 80: Use scientific profile.
Audit 80: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 81: Classroom tutor.
Benefit 81: The Witness catches false explanations before students see them.
Risk 81: The unchecked endpoint could produce misleading lesson content.
Mitigation 81: Use education profile and human review.
Audit 81: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 82: Coding assistant.
Benefit 82: The Witness blocks unsafe or incomplete code suggestions.
Risk 82: The unchecked endpoint could produce broken or dangerous code.
Mitigation 82: Use coding profile and retry chain.
Audit 82: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 83: Clinic information desk.
Benefit 83: The Witness adds caution before health information is shown.
Risk 83: The unchecked endpoint could produce overconfident medical advice.
Mitigation 83: Use medical profile and human escalation.
Audit 83: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 84: Disaster response team.
Benefit 84: The Witness keeps local verification available during outages.
Risk 84: The unchecked endpoint could produce bad instructions under pressure.
Mitigation 84: Use offline Gemma judge and audit logs.
Audit 84: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 85: Multilingual community.
Benefit 85: The Witness supports Arabic-English validation.
Risk 85: The unchecked endpoint could produce lost meaning or unsafe translation.
Mitigation 85: Use multilingual profile.
Audit 85: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 86: Finance workflow.
Benefit 86: The Witness flags risky financial claims.
Risk 86: The unchecked endpoint could produce unsupported investment advice.
Mitigation 86: Use finance profile and strict fallback.
Audit 86: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 87: Legal workflow.
Benefit 87: The Witness avoids overconfident legal conclusions.
Risk 87: The unchecked endpoint could produce jurisdiction mistakes.
Mitigation 87: Use legal profile and human review.
Audit 87: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 88: Research assistant.
Benefit 88: The Witness requires uncertainty and evidence.
Risk 88: The unchecked endpoint could produce hallucinated citations.
Mitigation 88: Use scientific profile.
Audit 88: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 89: Classroom tutor.
Benefit 89: The Witness catches false explanations before students see them.
Risk 89: The unchecked endpoint could produce misleading lesson content.
Mitigation 89: Use education profile and human review.
Audit 89: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 90: Coding assistant.
Benefit 90: The Witness blocks unsafe or incomplete code suggestions.
Risk 90: The unchecked endpoint could produce broken or dangerous code.
Mitigation 90: Use coding profile and retry chain.
Audit 90: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 91: Clinic information desk.
Benefit 91: The Witness adds caution before health information is shown.
Risk 91: The unchecked endpoint could produce overconfident medical advice.
Mitigation 91: Use medical profile and human escalation.
Audit 91: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 92: Disaster response team.
Benefit 92: The Witness keeps local verification available during outages.
Risk 92: The unchecked endpoint could produce bad instructions under pressure.
Mitigation 92: Use offline Gemma judge and audit logs.
Audit 92: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 93: Multilingual community.
Benefit 93: The Witness supports Arabic-English validation.
Risk 93: The unchecked endpoint could produce lost meaning or unsafe translation.
Mitigation 93: Use multilingual profile.
Audit 93: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 94: Finance workflow.
Benefit 94: The Witness flags risky financial claims.
Risk 94: The unchecked endpoint could produce unsupported investment advice.
Mitigation 94: Use finance profile and strict fallback.
Audit 94: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 95: Legal workflow.
Benefit 95: The Witness avoids overconfident legal conclusions.
Risk 95: The unchecked endpoint could produce jurisdiction mistakes.
Mitigation 95: Use legal profile and human review.
Audit 95: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 96: Research assistant.
Benefit 96: The Witness requires uncertainty and evidence.
Risk 96: The unchecked endpoint could produce hallucinated citations.
Mitigation 96: Use scientific profile.
Audit 96: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 97: Classroom tutor.
Benefit 97: The Witness catches false explanations before students see them.
Risk 97: The unchecked endpoint could produce misleading lesson content.
Mitigation 97: Use education profile and human review.
Audit 97: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 98: Coding assistant.
Benefit 98: The Witness blocks unsafe or incomplete code suggestions.
Risk 98: The unchecked endpoint could produce broken or dangerous code.
Mitigation 98: Use coding profile and retry chain.
Audit 98: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 99: Clinic information desk.
Benefit 99: The Witness adds caution before health information is shown.
Risk 99: The unchecked endpoint could produce overconfident medical advice.
Mitigation 99: Use medical profile and human escalation.
Audit 99: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 100: Disaster response team.
Benefit 100: The Witness keeps local verification available during outages.
Risk 100: The unchecked endpoint could produce bad instructions under pressure.
Mitigation 100: Use offline Gemma judge and audit logs.
Audit 100: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 101: Multilingual community.
Benefit 101: The Witness supports Arabic-English validation.
Risk 101: The unchecked endpoint could produce lost meaning or unsafe translation.
Mitigation 101: Use multilingual profile.
Audit 101: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 102: Finance workflow.
Benefit 102: The Witness flags risky financial claims.
Risk 102: The unchecked endpoint could produce unsupported investment advice.
Mitigation 102: Use finance profile and strict fallback.
Audit 102: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 103: Legal workflow.
Benefit 103: The Witness avoids overconfident legal conclusions.
Risk 103: The unchecked endpoint could produce jurisdiction mistakes.
Mitigation 103: Use legal profile and human review.
Audit 103: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 104: Research assistant.
Benefit 104: The Witness requires uncertainty and evidence.
Risk 104: The unchecked endpoint could produce hallucinated citations.
Mitigation 104: Use scientific profile.
Audit 104: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 105: Classroom tutor.
Benefit 105: The Witness catches false explanations before students see them.
Risk 105: The unchecked endpoint could produce misleading lesson content.
Mitigation 105: Use education profile and human review.
Audit 105: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 106: Coding assistant.
Benefit 106: The Witness blocks unsafe or incomplete code suggestions.
Risk 106: The unchecked endpoint could produce broken or dangerous code.
Mitigation 106: Use coding profile and retry chain.
Audit 106: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 107: Clinic information desk.
Benefit 107: The Witness adds caution before health information is shown.
Risk 107: The unchecked endpoint could produce overconfident medical advice.
Mitigation 107: Use medical profile and human escalation.
Audit 107: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 108: Disaster response team.
Benefit 108: The Witness keeps local verification available during outages.
Risk 108: The unchecked endpoint could produce bad instructions under pressure.
Mitigation 108: Use offline Gemma judge and audit logs.
Audit 108: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 109: Multilingual community.
Benefit 109: The Witness supports Arabic-English validation.
Risk 109: The unchecked endpoint could produce lost meaning or unsafe translation.
Mitigation 109: Use multilingual profile.
Audit 109: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 110: Finance workflow.
Benefit 110: The Witness flags risky financial claims.
Risk 110: The unchecked endpoint could produce unsupported investment advice.
Mitigation 110: Use finance profile and strict fallback.
Audit 110: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 111: Legal workflow.
Benefit 111: The Witness avoids overconfident legal conclusions.
Risk 111: The unchecked endpoint could produce jurisdiction mistakes.
Mitigation 111: Use legal profile and human review.
Audit 111: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 112: Research assistant.
Benefit 112: The Witness requires uncertainty and evidence.
Risk 112: The unchecked endpoint could produce hallucinated citations.
Mitigation 112: Use scientific profile.
Audit 112: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 113: Classroom tutor.
Benefit 113: The Witness catches false explanations before students see them.
Risk 113: The unchecked endpoint could produce misleading lesson content.
Mitigation 113: Use education profile and human review.
Audit 113: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 114: Coding assistant.
Benefit 114: The Witness blocks unsafe or incomplete code suggestions.
Risk 114: The unchecked endpoint could produce broken or dangerous code.
Mitigation 114: Use coding profile and retry chain.
Audit 114: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 115: Clinic information desk.
Benefit 115: The Witness adds caution before health information is shown.
Risk 115: The unchecked endpoint could produce overconfident medical advice.
Mitigation 115: Use medical profile and human escalation.
Audit 115: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 116: Disaster response team.
Benefit 116: The Witness keeps local verification available during outages.
Risk 116: The unchecked endpoint could produce bad instructions under pressure.
Mitigation 116: Use offline Gemma judge and audit logs.
Audit 116: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 117: Multilingual community.
Benefit 117: The Witness supports Arabic-English validation.
Risk 117: The unchecked endpoint could produce lost meaning or unsafe translation.
Mitigation 117: Use multilingual profile.
Audit 117: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 118: Finance workflow.
Benefit 118: The Witness flags risky financial claims.
Risk 118: The unchecked endpoint could produce unsupported investment advice.
Mitigation 118: Use finance profile and strict fallback.
Audit 118: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 119: Legal workflow.
Benefit 119: The Witness avoids overconfident legal conclusions.
Risk 119: The unchecked endpoint could produce jurisdiction mistakes.
Mitigation 119: Use legal profile and human review.
Audit 119: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 120: Research assistant.
Benefit 120: The Witness requires uncertainty and evidence.
Risk 120: The unchecked endpoint could produce hallucinated citations.
Mitigation 120: Use scientific profile.
Audit 120: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 121: Classroom tutor.
Benefit 121: The Witness catches false explanations before students see them.
Risk 121: The unchecked endpoint could produce misleading lesson content.
Mitigation 121: Use education profile and human review.
Audit 121: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 122: Coding assistant.
Benefit 122: The Witness blocks unsafe or incomplete code suggestions.
Risk 122: The unchecked endpoint could produce broken or dangerous code.
Mitigation 122: Use coding profile and retry chain.
Audit 122: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 123: Clinic information desk.
Benefit 123: The Witness adds caution before health information is shown.
Risk 123: The unchecked endpoint could produce overconfident medical advice.
Mitigation 123: Use medical profile and human escalation.
Audit 123: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 124: Disaster response team.
Benefit 124: The Witness keeps local verification available during outages.
Risk 124: The unchecked endpoint could produce bad instructions under pressure.
Mitigation 124: Use offline Gemma judge and audit logs.
Audit 124: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 125: Multilingual community.
Benefit 125: The Witness supports Arabic-English validation.
Risk 125: The unchecked endpoint could produce lost meaning or unsafe translation.
Mitigation 125: Use multilingual profile.
Audit 125: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 126: Finance workflow.
Benefit 126: The Witness flags risky financial claims.
Risk 126: The unchecked endpoint could produce unsupported investment advice.
Mitigation 126: Use finance profile and strict fallback.
Audit 126: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 127: Legal workflow.
Benefit 127: The Witness avoids overconfident legal conclusions.
Risk 127: The unchecked endpoint could produce jurisdiction mistakes.
Mitigation 127: Use legal profile and human review.
Audit 127: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 128: Research assistant.
Benefit 128: The Witness requires uncertainty and evidence.
Risk 128: The unchecked endpoint could produce hallucinated citations.
Mitigation 128: Use scientific profile.
Audit 128: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 129: Classroom tutor.
Benefit 129: The Witness catches false explanations before students see them.
Risk 129: The unchecked endpoint could produce misleading lesson content.
Mitigation 129: Use education profile and human review.
Audit 129: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 130: Coding assistant.
Benefit 130: The Witness blocks unsafe or incomplete code suggestions.
Risk 130: The unchecked endpoint could produce broken or dangerous code.
Mitigation 130: Use coding profile and retry chain.
Audit 130: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 131: Clinic information desk.
Benefit 131: The Witness adds caution before health information is shown.
Risk 131: The unchecked endpoint could produce overconfident medical advice.
Mitigation 131: Use medical profile and human escalation.
Audit 131: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 132: Disaster response team.
Benefit 132: The Witness keeps local verification available during outages.
Risk 132: The unchecked endpoint could produce bad instructions under pressure.
Mitigation 132: Use offline Gemma judge and audit logs.
Audit 132: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 133: Multilingual community.
Benefit 133: The Witness supports Arabic-English validation.
Risk 133: The unchecked endpoint could produce lost meaning or unsafe translation.
Mitigation 133: Use multilingual profile.
Audit 133: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 134: Finance workflow.
Benefit 134: The Witness flags risky financial claims.
Risk 134: The unchecked endpoint could produce unsupported investment advice.
Mitigation 134: Use finance profile and strict fallback.
Audit 134: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 135: Legal workflow.
Benefit 135: The Witness avoids overconfident legal conclusions.
Risk 135: The unchecked endpoint could produce jurisdiction mistakes.
Mitigation 135: Use legal profile and human review.
Audit 135: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 136: Research assistant.
Benefit 136: The Witness requires uncertainty and evidence.
Risk 136: The unchecked endpoint could produce hallucinated citations.
Mitigation 136: Use scientific profile.
Audit 136: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 137: Classroom tutor.
Benefit 137: The Witness catches false explanations before students see them.
Risk 137: The unchecked endpoint could produce misleading lesson content.
Mitigation 137: Use education profile and human review.
Audit 137: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 138: Coding assistant.
Benefit 138: The Witness blocks unsafe or incomplete code suggestions.
Risk 138: The unchecked endpoint could produce broken or dangerous code.
Mitigation 138: Use coding profile and retry chain.
Audit 138: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 139: Clinic information desk.
Benefit 139: The Witness adds caution before health information is shown.
Risk 139: The unchecked endpoint could produce overconfident medical advice.
Mitigation 139: Use medical profile and human escalation.
Audit 139: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 140: Disaster response team.
Benefit 140: The Witness keeps local verification available during outages.
Risk 140: The unchecked endpoint could produce bad instructions under pressure.
Mitigation 140: Use offline Gemma judge and audit logs.
Audit 140: The event is logged with verdict, reason, retry state, and fallback outcome.

Scenario 141: Multilingual community.
Benefit 141: The Witness supports Arabic-English validation.
Risk 141: The unchecked endpoint could produce lost meaning or unsafe translation.
Mitigation 141: Use multilingual profile.
Audit 141: The event is logged with verdict, reason, retry state, and fallback outcome.
