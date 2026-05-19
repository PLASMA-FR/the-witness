# Web UI Humanization Report

Who this is for: reviewers, maintainers, and anyone checking the Web UI polish pass.

What you will do: see what changed, which screens were improved, how mobile/accessibility were handled, and what remains environment-dependent.

## What was wrong before

The Web UI worked, but several parts read like an internal dashboard instead of a product surface. Labels such as “Doctor,” “Models,” and “Disapproved” were accurate but not self-explanatory. Empty states and error messages needed clearer next steps. Mobile needed special care so navigation would not consume the page.

## What changed

- Dashboard language now presents the page as “Mission Control.”
- Endpoint copy explains what is protected and how secrets stay outside config.
- Verdict wording uses human outcomes: “Approved and returned,” “Blocked before reaching the app,” and “Needs a human decision.”
- System checks now explain readiness in user language.
- Empty states explain what will appear and how to trigger it.
- Mobile navigation uses a compact top bar and bottom nav instead of a permanent sidebar.
- Cards, badges, copy buttons, command blocks, doctor checks, endpoint cards, model cards, request rows, and audit timeline copy were tightened.

## Screens redesigned or polished

| Screen | Improvements |
|---|---|
| Mission Control | Stronger hero, clear system state, endpoint action, model and proxy context. |
| Watched Endpoints | Safer auth wording, Blackbox quick-add guidance, endpoint card actions. |
| Live Requests | Clear stream purpose, filters, mobile-friendly request cards, human verdict wording. |
| Request Detail | Proof-trail framing for prompt, candidate, verdict, repair, and final response. |
| Prompt Repair | Explains how rejection reasons become retry instructions. |
| Needs a Human Decision | Honest pause state for high-risk or uncertain responses. |
| Choose How The Witness Thinks | Positions Gemma models and fine-tuned judge without unsupported claims. |
| Audit Logs | Describes verdicts, repairs, retries, and manual decisions as reviewable evidence. |
| System Check | Groups readiness checks and copyable fix commands. |
| Settings | Groups models, privacy, proxy, services, and defaults with safer wording. |

## Components created or improved

AppShell, ResponsiveSidebar, MobileDrawer behavior, TopStatusBar, PageHeader, MetricCard, StatusPill, VerdictBadge, EndpointCard, ModelCard, RequestCard, RequestTable/Card behavior, RequestTimeline, PromptDiff, PromptRepairCard, HumanReviewCard, AuditTimeline, DoctorCheckCard, FixCommandBox/CommandBlock, CopyButton, Primary/Secondary buttons, EmptyState, HealthIndicator, PrivacyBadge wording, Profile/Retry badges, resource LinkCards, and secret environment variable inputs.

## Mobile fixes

- Sidebar becomes a drawer below 820px.
- Bottom navigation exposes the highest-value pages on mobile.
- Tables become stacked cards.
- Buttons remain at least 44px high.
- Long commands wrap or scroll inside code blocks.
- Page content starts near the top; no large empty mobile sidebar remains.

## Accessibility improvements

- Skip link to main content.
- Visible focus states for buttons, links, inputs, selects, and textareas.
- Text labels accompany status colors.
- Icon-only buttons have accessible labels where they perform unique actions.
- Font sizes and touch targets were kept readable on 360–430px widths.
- Motion respects `prefers-reduced-motion`.

## Human copy improvements

Examples:

| Before | After |
|---|---|
| Dashboard | Mission Control |
| Models | Choose How The Witness Thinks |
| Doctor | System Check |
| Logs | Audit Logs |
| Disapproved | Blocked before reaching the app |
| Approved | Approved and returned |
| Human Review | Needs a human decision |
| No data | No requests yet. Send a request through a watched endpoint and the approval loop will appear here. |

## 20 design skills used

1. Information Architecture — navigation follows Mission Control → Endpoints → Requests → Repair → Review → Models → Logs → System Check → Settings.
2. Human-Centered UX — each page states what is happening, why it matters, and what to do next.
3. Visual Hierarchy — page headers, cards, metrics, badges, and command blocks separate priorities.
4. Responsive Design — desktop grids collapse into tablet/mobile layouts.
5. Mobile-First UX — drawer/sidebar behavior and bottom nav prevent the old empty-sidebar issue.
6. Dashboard Design — Mission Control shows active endpoints, verdicts, retries, latency, health, and live activity.
7. Data-Dense UI Design — technical request data appears in compact badges and cards.
8. Terminal-Inspired Product Design — dark local-control-room identity with readable monospace command blocks.
9. Interaction Design — copy actions, filters, endpoint tests, and navigation states are explicit.
10. Component System Design — shared cards, pills, buttons, code blocks, and panels carry consistent behavior.
11. Accessibility Design — focus, labels, contrast, and status text are included.
12. Onboarding UX — empty states push toward adding the first endpoint and running system check.
13. UX Microcopy — labels and statuses explain outcomes in plain language.
14. Error Message Design — Blackbox and doctor failures include cause and fix.
15. Trust & Safety Communication — the UI says what is blocked, paused, audited, and human-reviewed.
16. Motion & Microinteractions — subtle live pulses and card transitions support comprehension.
17. Brand System — calm navy/teal control-room visual system with approved/red/amber states.
18. Conversion-Oriented Product UX — primary path is add endpoint → send request → inspect verdict.
19. Technical Storytelling — request detail, audit timeline, and repair timeline explain the approval loop.
20. Release-Quality Polish — no intentional dead empty states; missing backend data falls back gracefully to labeled demo data.

## Screenshots

Expected screenshot directory:

```text
/home/admin/Gemma/witness/web_screenshots
```

Target files:

- `dashboard_desktop.png`
- `dashboard_mobile.png`
- `endpoints_desktop.png`
- `endpoints_mobile.png`
- `requests_desktop.png`
- `request_detail_desktop.png`
- `models_desktop.png`
- `models_mobile.png`
- `doctor_desktop.png`
- `settings_desktop.png`

## Known limitations

- Some Web UI actions depend on the local dashboard API being available.
- Missing live API data falls back to clearly marked demo data so the interface remains understandable.
- Vite may warn about a large JavaScript chunk because charts are bundled with the main dashboard.
- Optional backends still require their target runtimes and models.

## Test results

This report should be read with the latest command output from the final handoff. The required checks are `cargo fmt`, `cargo test`, `cargo build --release`, `npm run build`, dashboard API curls, browser/screenshot capture when available, and secret scans.
