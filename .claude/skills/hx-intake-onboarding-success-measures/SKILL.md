---
name: hx:intake-onboarding-success-measures
description: Define concrete, measurable success criteria that determine whether a project has achieved its goal. Use this skill when the intake process reaches the "success_measures" section, when the user needs help articulating what "done and successful" looks like, or when success criteria are vague or missing. Also trigger when the user says "how will we know this worked", "what does success look like", "define success criteria", "what metrics matter", or when establishing KPIs, acceptance thresholds, or completion benchmarks for a project.
---

# Intake Success Measures

You help users define what success actually looks like — not in vague terms like "it works well," but in concrete, measurable criteria that everyone can agree on. These measures become the benchmark against which UAT is conducted and the project is ultimately judged.

---

## Why success measures matter

Without explicit success measures, three bad things happen:

1. **Scope creep has no brake.** If you can't say "this is done," the project never is. Success measures draw the finish line.

2. **UAT becomes subjective.** When the project reaches user acceptance testing, the tester needs to know what to check. "Does it feel fast?" is not testable. "Dashboard loads in under 2 seconds" is.

3. **Stakeholders disagree silently.** The user thinks success means "zero bugs." Their manager thinks it means "ships by Friday." Without written measures, nobody discovers the mismatch until it's too late.

---

## Before you start

Pull in context from earlier sections:

```bash
harnessx project active
```

Read the project description and recall the confirmed goal and scope. Success measures should map directly to what the project set out to achieve — if a goal says "replace manual SSH monitoring with a dashboard," the success measures should verify that's actually happened.

---

## How to run the conversation

### Start by proposing, not asking

Most users struggle to articulate success measures from scratch. Instead of asking "what does success look like?", propose measures based on what you already know from the goal and scope, then let the user react:

> "Based on your goal and scope, here are some success measures I'd suggest. Tell me which ones resonate, which need adjusting, and what's missing:"

Then present a tailored set of proposed measures across the relevant categories below.

### Categories of success measures

Not every project needs measures in every category. Pick the ones relevant to the project.

**Functional measures** — Does it do what it's supposed to?
- Feature completion: "All in-scope features are implemented and working"
- Correctness: "Calculations match expected outputs for the test dataset"
- Integration: "Successfully reads from and writes to the external API"

Suggest specific examples tied to the project's features. For a trading dashboard: "Live price updates reflect within 500ms of exchange data." For a CLI tool: "All 12 subcommands produce correct output for the documented examples."

**Performance measures** — Is it fast/efficient enough?
- Response time: "API responds in under 200ms at p95"
- Throughput: "Handles 100 concurrent connections"
- Resource usage: "Memory stays under 512MB during normal operation"

Only propose these when performance is relevant to the project. A personal script doesn't need latency benchmarks.

**Quality measures** — Is it reliable and maintainable?
- Test coverage: "Core business logic has unit test coverage"
- Error handling: "Graceful error messages for all user-facing failure modes"
- Documentation: "API endpoints documented with examples"

**User experience measures** — Can people actually use it?
- Usability: "A new user can complete the primary workflow without documentation"
- Accessibility: "Meets WCAG 2.1 AA for all interactive elements"
- Satisfaction: "Ops team confirms it replaces their current SSH workflow"

**Business measures** — Does it achieve the intended outcome?
- Adoption: "Used by the target team within the first week"
- Efficiency: "Reduces time spent on X from 2 hours to 10 minutes"
- Cost: "Runs within the allocated infrastructure budget"

### Shape them through conversation

After proposing initial measures, iterate with the user:

- **Too vague?** Push for numbers: "When you say 'fast enough,' what's the threshold? Would 1 second feel fast? 500ms?"
- **Too ambitious?** Help them tier it: "That's a great north-star metric. For the MVP, what's the minimum acceptable version?"
- **Missing something?** Probe based on the project type: "You haven't mentioned anything about error handling — is that because it's straightforward, or because we haven't discussed it yet?"
- **Conflicting measures?** Surface the tension: "You want it shipped by Friday and fully tested — if you had to pick one, which matters more?"

### The three qualities of a good success measure

Help the user refine each measure until it has:

1. **Observable** — You can look at the system and see whether it's met. Not "users are happy" but "users complete the checkout flow without errors."

2. **Measurable** — There's a threshold or condition, not just a direction. Not "fast" but "under 2 seconds." Not "tested" but "all critical paths have integration tests."

3. **Agreed** — The user has explicitly confirmed this is what they care about. Don't assume.

---

## Structuring the final measures

Once the conversation has produced a set of measures, organize them into two tiers:

### Must-have (blocking)
These are the measures that must be met for the project to be considered successful. UAT cannot pass without these. Keep this list focused — typically 3-7 items.

### Nice-to-have (non-blocking)
These are measures that would make the project better but aren't required for sign-off. They might be pursued if time allows, or deferred to a follow-up.

Present the tiered list to the user for confirmation. This distinction matters because it directly feeds into what the UAT section will test.

---

## Capturing as action items

Create action items for each confirmed success measure using `harnessx intake-actions create`:

```bash
harnessx intake-actions create \
  --title "Success measure: Dashboard loads in under 2 seconds at p95" \
  --category "quality-gate" \
  --origin "intake:success_measures" \
  --detail "Must-have measure. The main dashboard view must render completely within 2 seconds at p95 under normal load (up to 10 concurrent users). This will be verified during UAT with real data." \
  --tags "performance,uat-blocking" \
  --complexity low \
  --mode plan \
  --note-author "hx-intake-specialist" \
  --note-text "User emphasized this is the single most important metric — their team currently waits 30+ seconds for SSH-based checks."
```

Use tags to distinguish blocking vs non-blocking:
- `uat-blocking` — must-have measures
- `uat-nice-to-have` — non-blocking measures

---

## Knowing when you're done

You're done when:
- The user has a clear set of must-have success measures (at least 2-3)
- Each measure is observable and measurable
- The user has confirmed the must-have vs nice-to-have split
- All measures are captured as action items with appropriate tags

Summarize: "We've defined X must-have and Y nice-to-have success measures. The must-haves will be the gate for UAT sign-off." Then mark the section complete.

---

## Document the Discussion

Before marking this section complete, write a comprehensive markdown file that captures the full substance of the success measures discussion. Get the active project ID:

```bash
harnessx project active
```

Then save the document to `harnessx/<project-id>/intake/success_measures.md`.

The document should include:

- **Date** of the discussion
- **Initial proposals** — the success measures you suggested based on the goal and scope, and the user's reaction
- **Questions asked and responses** — how measures were refined, what thresholds were debated, any conflicts surfaced
- **The confirmed must-have measures** — the full list of blocking success criteria the user approved, each with its observable/measurable definition
- **The confirmed nice-to-have measures** — non-blocking measures that would improve the project
- **Key decisions and reasoning** — why specific measures were classified as must-have vs. nice-to-have, any trade-offs discussed
- **Measures that were considered and rejected** — ideas that came up but were dropped, and why
- **Relationship to UAT** — how these measures will be verified during user acceptance testing
- **Action items created** during this section (titles and brief descriptions)

Write this as a readable narrative document, not a raw chat log. The goal is that any agent or person reading this file later knows exactly what success looks like for this project and why those criteria were chosen, without needing access to the original conversation.
