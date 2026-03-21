---
name: hx-intake-completion-project-risk
description: Multi-agent project risk review that audits all intake documents, exploration notes, and existing action items to identify gaps that would lead to poor quality output — missing concurrency plans, unaddressed error handling, integration assumptions, data integrity blindspots, and other concerns LLMs typically overlook. Creates defensive action items to ensure all bases are covered before planning begins. Use this skill when the intake-completion project_risk_manager section needs to run, when the user says "check for risks", "what are we missing", "audit the actions", "risk review", "gap analysis", or when the pipeline reaches project_risk_manager after ideation completes.
---

# Project Risk Manager

You are the last line of defense before the project enters planning. Your job is to find what everyone else missed — the action items that should exist but don't, the assumptions that haven't been questioned, the technical considerations that LLMs habitually overlook because they're the kind of thing that only surfaces during implementation when it's expensive to fix.

This skill exists because there's a predictable pattern in LLM-assisted projects: the intake captures the user's intent well, exploration maps the technical landscape, and ideation adds creative value — but nobody stops to ask "have we actually thought about concurrency? Error recovery? Data migration? Deployment? What happens when the API is down?" These are the questions that separate a project that ships cleanly from one that hits constant surprises.

## Why This Matters

Milestones, epics, and stories come later in the pipeline. But those planning artifacts build on top of the action items created during intake. If a critical concern isn't captured as an action item now, it won't appear in the plan, won't get a milestone, and will surface as a fire drill during execution. The cost of adding an action item here is nearly zero. The cost of discovering the gap during implementation is enormous.

Your job is not to be paranoid about everything — it's to be systematically thorough about the things that actually matter for this specific project.

---

## Startup Sequence

Run these commands and read all outputs:

```bash
harnessx project status
harnessx intake-onboarding status
harnessx intake-completion status
harnessx intake-actions list
```

Then read ALL project documents:

**Intake documents** in `harnessx/<project-id>/intake/`:
- `goal.md` — what the project is trying to achieve
- `scope.md` — what's in and out of bounds
- `user_knowledge.md` — user's expertise and preferences
- `resources.md` — collected resources
- `success_measures.md` — what "done and successful" looks like
- `uat.md` — user acceptance testing plan
- Interview markdowns (`interview-*.md`)

**Exploration documents** in `harnessx/<project-id>/intake/exploration/`:
- `summary.md` and all subfolder notes

**Ideation documents** in `harnessx/<project-id>/intake/ideation/`:
- `ideation-report.md`

**Existing action items**:
- `harnessx/<project-id>/intake/intake_actions.json` — read every single one

Build a complete mental model of the project: goal, scope, technical landscape, what's been explored, what ideas were generated, and — critically — what action items already exist and what they cover.

---

## The Risk Review Process

### Phase 1: Dispatch Risk Auditors

Launch 3-5 agents concurrently, each examining the project through a different risk lens. Every agent receives the full project context (summarized) and the complete list of existing action items.

The core instruction to each agent: **"Read every existing action item. Then identify what's missing — the actions that should exist but don't."**

**Risk Lenses:**

| Lens | What This Agent Looks For | Best Model |
|------|--------------------------|------------|
| **Technical Foundations** | Concurrency strategy, threading model, memory management, data structure choices, algorithm complexity, build system, CI/CD, deployment | Opus |
| **Integration & External Systems** | API contracts, authentication flows, rate limits, retry logic, timeout handling, webhook reliability, third-party SLA assumptions, version pinning | Opus |
| **Data & State** | Data integrity, migration paths, backup/recovery, schema evolution, state machine completeness, race conditions, consistency guarantees, caching invalidation | Opus |
| **Error & Failure Modes** | Error propagation strategy, graceful degradation, circuit breakers, logging/observability, crash recovery, partial failure handling, user-facing error messages | Sonnet |
| **Quality & Delivery** | Testing strategy gaps, performance benchmarks, documentation plan, security review, accessibility, cross-platform concerns, dependency audit, tech debt tracking | Sonnet |

Choose 3-5 lenses based on the project type. A trading system needs all five. A simple CLI tool might only need Technical Foundations + Error Modes + Quality. Adapt to the project.

**Agent Prompt Template:**

```
You are a project risk auditor reviewing a software project before it enters planning. Your lens is [LENS NAME].

## Project Context

[Summarize: goal, scope, technical landscape from exploration, key decisions from ideation]

## Existing Action Items

Here is every action item that currently exists:

[Paste the full list — id, title, category, detail for each]

## Your Assignment

Your job is to find what's MISSING. Read every existing action item carefully. Then, thinking through the [LENS] perspective, identify gaps — action items that should exist but don't.

For each gap you identify:

1. **Title** — Verb-first, specific (e.g., "Design retry strategy for exchange API disconnections" not "Handle errors")
2. **Why this is a risk** — What goes wrong if this isn't addressed? Be concrete about the failure mode.
3. **What the action item should cover** — Specific enough that an agent can act on it without guessing
4. **Severity** — Critical (blocks delivery) / High (causes rework) / Medium (degrades quality) / Low (nice to have)
5. **Category** — research, verification, implementation, testing, exploration, integration, documentation
6. **Already partially covered?** — Does an existing action touch on this? If so, which one, and what's missing from it?

## What to Look For (specific to [LENS])

[Include the lens-specific checklist from below]

## Rules

- Only flag gaps that are relevant to THIS project's goal and scope
- Don't flag things already covered by existing action items (but DO flag items that partially cover something and need expansion)
- Be specific — "add error handling" is not useful. "Design error propagation from WebSocket message handler through orderbook update pipeline" is useful.
- Prioritize gaps that would be expensive to discover later vs. cheap to address now
- Remember: milestones, epics, and stories come later. You're not planning the work — you're ensuring the important considerations have action items so they don't get lost.

Write your findings to: [output path]
```

### Lens-Specific Checklists

Include these in the agent prompt for the relevant lens.

**Technical Foundations:**
- Is there an action for choosing/validating the concurrency model (async runtime, thread pool, actor model)?
- Is there an action for memory management strategy (allocation patterns, pool reuse, arena allocation)?
- Is there an action for build configuration (release profiles, feature flags, conditional compilation)?
- Is there an action for dependency audit (license compatibility, maintenance status, transitive deps)?
- Is there an action for performance-critical algorithm choices (time complexity verified for expected data sizes)?
- Is there an action for deployment strategy (containerization, environment config, rollback plan)?
- If multi-threaded: is there an action for shared state management and synchronization strategy?
- If real-time: is there an action for latency budget allocation across pipeline stages?

**Integration & External Systems:**
- For each external API: is there an action covering auth flow, rate limits, error codes, and retry strategy?
- Is there an action for handling API version changes or deprecations?
- Is there an action for timeout configuration (connect, read, total) for each external call?
- Is there an action for circuit breaker or backpressure design when external systems are slow/down?
- Is there an action for credential management (rotation, secure storage, environment separation)?
- Is there an action for webhook delivery guarantees (idempotency, ordering, replay)?
- Is there an action for monitoring external system health from our side?

**Data & State:**
- Is there an action for data model design (schemas, relationships, constraints)?
- Is there an action for state machine completeness (all valid transitions, invalid transition handling)?
- Is there an action for data migration path (if upgrading from existing system)?
- Is there an action for backup and disaster recovery (if persistent state)?
- Is there an action for data validation at system boundaries (input sanitization, schema validation)?
- Is there an action for handling stale/cached data (TTLs, invalidation strategy)?
- Is there an action for race condition analysis (concurrent writes, read-your-writes consistency)?
- Is there an action for what happens to in-flight data during restart/crash?

**Error & Failure Modes:**
- Is there an action for error type hierarchy design (structured errors, error codes, context preservation)?
- Is there an action for logging and observability strategy (structured logging, correlation IDs, metrics)?
- Is there an action for graceful degradation (what still works when subsystem X fails)?
- Is there an action for crash recovery (restart behavior, state reconstruction, data integrity checks)?
- Is there an action for user-facing error messages (when applicable — helpful, actionable, not leaking internals)?
- Is there an action for distinguishing transient vs. permanent failures in retry logic?
- Is there an action for dead letter / poison message handling?

**Quality & Delivery:**
- Is there an action for testing strategy (unit test scope, integration test environments, test data management)?
- Is there an action for performance benchmarks (baseline measurements, regression detection)?
- Is there an action for documentation (API docs, architecture decision records, operational runbooks)?
- Is there an action for security review (input validation, auth boundaries, secret management)?
- Is there an action for dependency update strategy (how often, how to test, breaking change handling)?
- Is there an action for configuration management (env-specific config, feature flags, defaults)?
- Is there an action for technical debt tracking (known shortcuts that need revisiting)?

### Phase 2: Collect and Deduplicate

After all agents return, read every finding from `harnessx/<project-id>/intake/risk-review/`.

**Deduplication:** Multiple agents may flag the same gap from different angles. This is a strong signal — merge them into a single, stronger finding. Note which lenses converged.

**Cross-reference with existing actions:** For each gap, verify it truly isn't covered. Search existing action items by title and detail. Sometimes an existing action covers a gap but with different wording. Only create new actions for genuine gaps.

**Filter by relevance:** Not every checklist item applies to every project. A CLI tool doesn't need webhook reliability. A batch processor doesn't need real-time latency budgets. Drop findings that don't apply to this project's actual scope.

### Phase 3: Write the Risk Report

Create `harnessx/<project-id>/intake/risk-review/risk-report.md`:

```markdown
# Project Risk Report

**Date:** YYYY-MM-DD
**Project:** <project-id>
**Auditors:** [which lenses were used]

## Executive Summary

[2-3 sentences: how many gaps found, severity distribution, overall assessment of project readiness for planning]

## Critical Gaps (Must Address Before Planning)

These gaps would block delivery or cause major rework if not addressed.

### 1. [Gap Title]

**Risk:** [What goes wrong if this isn't addressed]
**Severity:** Critical
**Lens:** [Which auditor(s) found this]
**Existing coverage:** [None / Partially covered by action-X — what's missing]
**Recommended action:** [What the new action item should say]

### 2. ...

## High-Priority Gaps (Should Address Before Planning)

These gaps would cause rework or quality issues.

### ...

## Medium-Priority Gaps (Address During Planning)

These can be captured as part of milestone/epic planning but should be tracked now.

### ...

## Low-Priority Gaps (Track for Later)

Nice-to-haves that don't need action items yet — noted here for the planning stage.

| Gap | Why It's Low Priority | When to Revisit |
|-----|----------------------|-----------------|
| ... | ... | ... |

## Existing Actions That Need Expansion

These existing action items partially cover a risk but need additional detail or scope.

| Action ID | Current Title | What's Missing | Severity |
|-----------|--------------|----------------|----------|
| ... | ... | ... | ... |

## Convergence Map

Gaps flagged by multiple auditors independently:

| Gap Theme | Lenses | Signal Strength |
|-----------|--------|----------------|
| ... | Technical + Data | Strong — both arrived independently |

## Assessment

**Ready for planning:** [Yes / Yes with conditions / No — explain]
**Biggest risk:** [The single most important thing to address]
**Confidence level:** [How confident are we that we've covered the important gaps]
```

### Phase 4: Create Action Items

For each gap in Critical and High-Priority sections, create an action item using `hx:intake-actions-writing`.

**Key fields for risk-origin actions:**

- **origin**: `intake:project_risk`
- **category**: depends on the gap — most will be `research`, `verification`, or `implementation`
- **detail**: include the risk (what goes wrong), the specific concern, and what the action should produce. Self-contained for a fresh agent.
- **input_docs**: point to the risk report AND any relevant exploration/ideation notes
- **complexity**: map from severity — Critical → high, High → medium, Medium → low
- **mode**: `plan` (risk mitigations need planning, not immediate execution)
- **note-author**: `hx-intake-completion-project-risk`
- **note-text**: explain which auditor lens(es) flagged this and why it matters

For Medium-Priority gaps: create action items but mark complexity as `low` and add a note that these can be folded into milestone planning.

For existing actions that need expansion: use `harnessx intake-actions update` to add a note explaining what's missing, rather than creating duplicate actions.

Do NOT create action items for Low-Priority gaps — they're documented in the report for the planning stage to pick up.

### Phase 5: Tag Everything

Follow `hx:tag-context-writing` protocol.

**Tag the risk report:**
- Add `#action-N` to each gap paragraph that generated a new action item

**Cross-reference with source documents:**
- If a gap relates to something in scope.md (e.g., "concurrency is in scope but no action for thread model"), tag that scope paragraph
- If a gap relates to exploration findings, tag the relevant exploration note
- If a gap relates to an ideation idea, tag the relevant ideation paragraph

**Tag existing action expansions:**
- When updating an existing action with a note, tag the risk report paragraph that prompted the update

**Verify every tag:**
```bash
harnessx context search-context --query "#action-N"
```

---

## Choosing Agent Count and Lenses

Scale to the project:

**Simple project** (CLI tool, single-purpose utility):
- 3 agents: Technical Foundations + Error Modes + Quality
- Skip Integration unless it talks to external services
- Skip Data unless it persists state

**Standard project** (web service, data pipeline, multi-module app):
- 4 agents: Technical Foundations + Integration + Data + Error Modes
- Quality concerns can be folded into Technical Foundations

**Complex project** (trading system, distributed system, real-time pipeline):
- 5 agents: all lenses
- Consider splitting Technical Foundations into "Architecture" and "Performance" if the project is performance-critical

**Non-coding project** (if harnessx is used for non-software projects):
- Adapt the lenses to the domain — replace Technical/Data/Integration with domain-appropriate risk categories
- The principle is the same: what's missing from the action items that would cause problems later?

---

## The LLM Blindspot Problem

This skill exists specifically because LLMs have predictable blindspots when planning projects. Here are the most common ones — make sure your auditor agents are primed to catch them:

1. **"Happy path only"** — LLMs plan for when everything works. They rarely create actions for what happens when things fail, time out, return unexpected data, or run concurrently.

2. **"It'll just work"** — LLMs assume integrations are straightforward. They skip actions for auth flows, API quirks, rate limit handling, and version compatibility.

3. **"One machine, one thread"** — LLMs default to sequential, single-process thinking. They miss actions for concurrency, distributed state, message ordering, and deployment topology.

4. **"The data is clean"** — LLMs assume inputs are well-formed. They miss actions for validation, sanitization, schema evolution, and handling corrupt/partial data.

5. **"Deploy and done"** — LLMs forget about operational concerns: monitoring, logging, alerting, rollback procedures, configuration management, and secret rotation.

6. **"Tests will catch it"** — LLMs create actions for writing tests but not for test infrastructure: test data management, integration test environments, CI pipeline setup, flaky test handling.

7. **"The spec is complete"** — LLMs take requirements at face value. They miss actions for clarifying ambiguous requirements, resolving contradictions between documents, and validating assumptions with the user.

Each auditor agent should be specifically warned about these blindspots relevant to their lens.

---

## Completion

After the risk report is written, action items are created, expansions are noted, and tags are placed:

```bash
harnessx intake-completion complete project_risk_manager
```

This marks the project_risk_manager section as completed in `intake_completion.json`. When all three intake-completion sections (exploration, ideation, project_risk_manager) are done, the CLI automatically marks the `intake_completion` pipeline stage as complete and the project advances to planning.

---

## Edge Cases

**Exploration or ideation not done yet:**
- Risk review depends on both. If either is incomplete, tell the user and wait.
- You need the full picture to audit effectively.

**Very few existing action items:**
- This is itself a risk signal — either intake was thin or actions weren't captured properly.
- Flag it in the report and create more foundational actions.

**Hundreds of existing action items:**
- Focus auditor agents on gaps, not reviewing every action in detail.
- Look for category-level coverage: are there ANY actions for error handling? For testing? For deployment?
- Missing categories are more important than missing details within covered categories.

**Project is very simple (almost no risks):**
- Even simple projects benefit from: error handling strategy, testing approach, deployment plan.
- Keep the review proportional — don't invent risks. But do the check.

**Risk review reveals scope is insufficient:**
- If multiple auditors flag that the scope doesn't address a critical concern, note it prominently.
- Create a high-priority action to revisit scope with the user.
- Do not change scope yourself.
