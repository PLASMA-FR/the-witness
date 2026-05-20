# Web UI Design Report

The Web UI is the operator console for The Witness. It is intentionally not a chatbot screen. It is a mission-control view for watching AI endpoints, judging outputs, repairing prompts, and keeping an audit trail.

## Before this pass

The dashboard already had a strong dark visual direction, but the final review found a few release-blocking polish issues:

- dense health rows could crowd long status text;
- chart panels looked too empty when live requests were not present;
- empty/demo states needed to feel intentional rather than unfinished;
- API aliases expected by the Settings/System pages were missing;
- generated `web/dist` files were tracked even though the repo should rebuild them locally.

## What changed

- Added `/api/settings` as an alias for the redacted config API.
- Added `/api/system/status` for dashboard/service status summaries.
- Improved the System Health layout so long Tailscale and URL messages wrap instead of colliding.
- Added explicit demo-data messaging to the request chart.
- Added a readable center label to the verdict donut.
- Kept `web/dist/` out of Git while still verifying the production build.

## Design principles used

1. Information Architecture — navigation maps to the operator workflow: dashboard, endpoints, requests, detail, repair, review, models, logs, doctor, settings.
2. Human-Centered UX — empty states explain what to do next instead of blaming the user for missing setup.
3. Visual Hierarchy — hero, readiness, metrics, activity, and actions are ordered from urgent to supporting detail.
4. Responsive Design — cards stack, mobile navigation appears, and desktop density stays useful.
5. Dashboard Design — key metrics are glanceable: endpoints, requests, approvals, disapprovals, human review, latency, retries.
6. Data-Dense UI — operational detail stays visible without hiding the safety story.
7. Terminal-Inspired Product Design — dark surface, monospaced commands, and local-first copy match the developer workflow.
8. Interaction Design — primary actions are obvious; secondary actions are grouped as quick cards and copy buttons.
9. Component System Design — panels, status pills, metric cards, code blocks, and forms share tokens and spacing.
10. Accessibility — focus outlines, readable button labels, contrast-aware status colors, and reduced-motion support are present.
11. Mobile-First UX — mobile gets bottom navigation and stacked cards instead of a squeezed desktop sidebar.
12. Onboarding UX — demo mode fills the dashboard and explains the live endpoint path.
13. Microcopy — copy is direct, calm, and operational: “Proxy ready,” “Before traffic is trusted,” “Run doctor.”
14. Motion and Microinteractions — small pulses and hover states communicate liveness without turning the app into a distraction.
15. Brand System — teal safety signal, dark local-control surface, and “Witness” language reinforce the reliability firewall identity.

## Screenshots

Screenshots generated during verification are stored in `web_screenshots/` when browser automation is available. The desktop dashboard screenshot used in review was captured through the browser automation session.

## Accessibility notes

- Buttons use text labels or accessible labels.
- Focus-visible outlines are defined globally.
- Motion is disabled for users who prefer reduced motion.
- Status is not communicated only by color; labels such as PASS, WARN, FAIL, APPROVED, BLOCKED, and HUMAN are visible.

## Remaining design notes

- The MVP uses demo data when no live endpoints or logs exist. This is intentional and labeled.
- Production deployments should run `the-witness doctor` before sending real traffic through the proxy.
- Streaming response visualization is future work; MVP focuses on non-streaming OpenAI-compatible chat completions.
