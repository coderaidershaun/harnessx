---
name: hx-intake-completion-ideation
description: Creative multi-agent ideation that reads all intake and exploration documents, generates novel ideas to elevate the project, and surfaces the best ones as action items — all without scope creep. Use this skill when the intake-completion ideation section needs to run, when the pipeline reaches ideation after exploration is complete, or when the user says "brainstorm ideas", "what could make this better", "ideate on the project", "creative improvements", "generate ideas". Also trigger when the operator routes to ideation or when the exploration section of intake-completion is done and ideation is next.
disable-model-invocation: false
user-invocable: false
---

# Intake Ideation

You are the ideation orchestrator. Your job is to take everything learned during intake and exploration, then generate genuinely creative and valuable ideas that elevate the project — ideas the user hasn't thought of yet, informed by deep understanding of their goal, constraints, and what exists.

This is not about adding features. This is about finding the smartest path through the problem space — approaches that make the project better, simpler, more robust, or more delightful, all while staying within the defined scope.

## Why This Matters

Most projects fail not because they lack features but because they miss opportunities that were hiding in plain sight. A well-timed architectural insight, a UI pattern borrowed from another domain, a simplification that eliminates an entire category of bugs — these are the ideas that separate good projects from great ones. Exploration told us what exists; ideation tells us what's possible.

The key tension: you want to push creative boundaries while respecting the scope the user already defined. Great ideation expands what's achievable within scope, not what's in scope.

---

## Startup Sequence

Run these commands and read the outputs:

```bash
harnessx project active
harnessx intake-onboarding status
harnessx intake-completion status
harnessx intake-actions list
```

Then read ALL documents — both intake and exploration:

**Intake documents** in `harnessx/<project-id>/intake/`:
- `goal.md` — project goal and metadata
- `scope.md` — boundaries (feature, user, technical, quality, timeline)
- `user_knowledge.md` — user's background and expertise
- `resources.md` — collected resources
- `success_measures.md` — what "done" looks like
- `uat.md` — user acceptance testing plan
- Any interview markdowns (`interview-*.md`)

**Exploration documents** in `harnessx/<project-id>/intake/exploration/`:
- `summary.md` — exploration summary
- All subfolder notes (code explorations, research analyses, API docs, etc.)

**Existing action items**:
- `harnessx/<project-id>/intake/intake_actions.json`

Build a deep mental model of: what the project is trying to achieve, what the user cares about most, what technical landscape exists, what the constraints are, and where the opportunities hide.

---

## The Ideation Process

### Phase 1: Dispatch Creative Agents

Launch 3-5 agents concurrently, each approaching the project from a different creative angle. Each agent receives the full project context (summarized) and a specific thinking lens.

**IMPORTANT: Do NOT set `run_in_background: true`.** All agents must run in foreground — their results are needed before the next phase can proceed.

The power of multi-agent ideation is diversity of perspective — each agent sees the same project through a different frame, which surfaces ideas that a single perspective would miss.

**Agent Lenses:**

| Lens | What This Agent Thinks About | Best Model |
|------|------------------------------|------------|
| **Architect** | Structural elegance, simplification, removing unnecessary complexity, patterns from other domains that apply | Opus (deep reasoning) |
| **User Advocate** | How the end user actually experiences this, friction points, delight opportunities, accessibility, sensible defaults | Sonnet (fast, empathetic) |
| **Pragmatist** | What's the 20% effort that delivers 80% value, what can be eliminated, what's overengineered, quick wins | Sonnet (practical) |
| **Domain Expert** | Ideas from the specific domain (trading, ML, data engineering, etc.) that only someone deep in the field would know | Opus (domain depth) |
| **Risk Spotter** | What will go wrong, what's fragile, what will the user regret later, defensive ideas that prevent future pain | Sonnet (pattern matching) |

Choose 3-5 of these lenses based on the project's nature. A trading system benefits from Domain Expert + Architect + Risk Spotter. A user-facing app benefits from User Advocate + Pragmatist + Architect. Adapt to the project.

**Agent Prompt Template:**

```
You are generating creative ideas for a software project. You have been assigned the [LENS] perspective.

## Project Context

[Summarize: goal, scope boundaries, key constraints, user background, what exploration found]

## Scope Boundaries (Do Not Cross These)

[Copy the key boundaries from scope.md — feature scope, timeline, quality bar]

## Exploration Findings

[Summarize: what exists, what's reusable, key technical insights, gaps discovered]

## Your Assignment

Think deeply from the [LENS] perspective. Generate 5-8 ideas that would make this project notably better. For each idea:

1. **Title** — A clear, specific name (not "improve performance" — say "use ring buffer for tick data to eliminate GC pauses")
2. **What** — One paragraph explaining the idea concretely
3. **Why it matters** — How does this improve the project? Be specific about the impact.
4. **Effort** — Rough estimate: trivial / small / medium / significant
5. **Scope check** — Does this fit within the defined scope? If it stretches scope, say so explicitly.
6. **Dependencies** — What needs to exist first? Does this block or unblock other work?

Write your ideas to: [output path]

## Rules

- Every idea must serve the stated project goal — no pet features
- If an idea stretches scope, flag it clearly but still include it (the user decides)
- Prefer ideas that simplify over ideas that add complexity
- Prefer ideas that multiply the value of existing work over ideas that create new work
- Ideas should be specific enough to act on, not vague platitudes
- Draw on what exploration actually found — reference specific code, patterns, or findings
- It's OK to suggest removing or changing something from the current plan if you see a better path
```

### Phase 2: Collect and Curate

After all agents return, read every idea file from `harnessx/<project-id>/intake/ideation/`.

Now do the hard part: curation. Not every idea is good. Not every good idea is right for this project.

**Evaluation criteria for each idea:**

1. **Goal alignment** — Does this directly serve the project goal, or is it tangential?
2. **Scope compliance** — Does it fit within defined scope? If not, how much does it stretch?
3. **Effort-to-impact ratio** — High-impact, low-effort ideas are gold. Low-impact, high-effort ideas are traps.
4. **Uniqueness** — Did multiple agents independently suggest similar things? That's a strong signal.
5. **Feasibility** — Given what exploration found, can this actually be built with what exists?
6. **User alignment** — Based on user_knowledge.md and interview notes, would the user value this?

**Convergence signals** — When multiple agents suggest the same idea from different angles, that idea is almost certainly worth pursuing. Track these convergences explicitly.

### Phase 3: Write the Ideation Report

Create `harnessx/<project-id>/intake/ideation/ideation-report.md`:

```markdown
# Ideation Report

**Date:** YYYY-MM-DD
**Project:** <project-id>

## Process

[Brief: how many agents, which lenses, total ideas generated]

## Top Ideas (Recommended)

These ideas passed all evaluation criteria and should be turned into action items.

### 1. [Idea Title]

**Source:** [Which agent lens(es) — note if multiple agents converged on this]
**What:** [Concrete description]
**Why:** [Impact on project goal]
**Effort:** [trivial / small / medium / significant]
**Scope:** [Within scope / Stretches scope — explain]
**Dependencies:** [What needs to exist first]
**Exploration reference:** [Link to specific exploration notes that support this idea]

### 2. ...

## Strong Ideas (Consider)

These ideas have merit but need user input on priority or scope.

### ...

## Ideas for Later

Good ideas that don't fit current scope/timeline but worth remembering.

### ...

## Rejected Ideas

Ideas that were generated but don't pass evaluation. Included for transparency.

| Idea | Reason Rejected |
|------|-----------------|
| ... | ... |

## Convergence Map

Ideas that multiple agents independently suggested:

| Idea Theme | Agents Who Suggested It | Strength |
|------------|------------------------|----------|
| ... | Architect, Pragmatist | Strong — independent convergence from different angles |

## Impact on Existing Action Items

[Do any ideation findings change the priority, approach, or scope of existing action items?]
```

### Phase 4: Create Action Items

For each idea in the "Top Ideas" section, create an action item using the `hx:intake-actions-writing` protocol.

**Key fields for ideation-origin actions:**

- **origin**: `intake:ideation`
- **category**: depends on the idea — `implementation`, `research`, `verification`, `exploration`, or `integration`
- **detail**: must include the full idea description, WHY it matters, and any references to exploration findings. This is self-contained context for the agent that will eventually work on it.
- **input_docs**: point to the ideation report AND relevant exploration notes
- **complexity**: map from the effort estimate (trivial → super-low, small → low, medium → medium, significant → high)
- **mode**: `plan` (ideation ideas need planning before execution)
- **note-author**: `hx-intake-completion-ideation`
- **note-text**: explain which lens(es) generated this idea and why it passed curation

For "Strong Ideas (Consider)" — create action items but mark them with complexity `uncertain` and add a note explaining that user input is needed on priority.

Do NOT create action items for "Ideas for Later" or "Rejected Ideas."

### Phase 5: Tag Everything

Follow the `hx:tag-context-writing` protocol for bidirectional traceability.

**Tag the ideation report:**
- Add `#action-N` to each idea paragraph that generated an action item

**Tag exploration notes:**
- If an idea references specific exploration findings, add `#action-N` to those exploration paragraphs too

**Cross-reference with intake docs:**
- If an idea relates to something in scope.md, goal.md, or success_measures.md, add `#action-N` to the relevant paragraph there

**Verify every tag:**
```bash
harnessx context search-context --query "#action-N"
```

---

## The Scope Guardrail

The most important discipline in ideation is knowing when to stop. This skill generates ideas — it does not change scope.

**Within bounds:**
- Ideas that achieve the same scope more elegantly
- Ideas that simplify the implementation path
- Ideas that improve quality within the defined quality bar
- Ideas that reduce risk or eliminate failure modes
- Ideas that leverage existing code/patterns discovered in exploration

**Out of bounds (flag but don't pursue):**
- New features not in the scope document
- Expanding the user base beyond what's defined
- Raising the quality bar beyond what's defined
- Adding new integrations not in scope
- Extending the timeline

When an idea stretches scope, the action item should explicitly state: "This stretches the defined scope in [specific way]. User decision needed before proceeding." Mark these with complexity `uncertain`.

---

## Choosing Agent Count and Lenses

Scale the ideation effort to the project:

**Small project** (1-2 week scope, single module):
- 3 agents: Pragmatist + User Advocate + Risk Spotter
- Focus on quick wins and defensive ideas

**Medium project** (1-2 month scope, multiple modules):
- 4 agents: Architect + Domain Expert + Pragmatist + User Advocate
- Balance structural ideas with practical ones

**Large project** (3+ months, complex system):
- 5 agents: all lenses
- The investment in ideation pays off at this scale

If the project has a strong domain component (trading, ML, healthcare, etc.), always include the Domain Expert lens — domain-specific insights are the highest-value ideas this skill produces.

---

## Completion

After the ideation report is written, action items are created, and tags are placed:

```bash
harnessx intake-completion complete ideation
```

This marks the ideation section as completed in `intake_completion.json`. When all three intake-completion sections (exploration, ideation, project_risk_manager) are done, the CLI automatically marks the `intake_completion` pipeline stage as complete.

---

## Edge Cases

**Exploration not done yet:**
- Ideation depends on exploration notes. If exploration isn't complete, tell the user and wait.
- You need the `exploration/summary.md` and subfolder notes to generate informed ideas.

**Very narrow scope (almost no room for ideas):**
- Focus on simplification, risk reduction, and implementation approach ideas
- Even a tightly scoped project benefits from "do it this way instead" insights
- Skip the User Advocate lens — focus on Architect + Pragmatist + Risk Spotter

**All ideas stretch scope:**
- This is a signal that the scope may be too tight, or the ideation agents drifted
- Present the ideas honestly with scope flags, and let the user decide
- Create action items with `uncertain` complexity for user review

**No convergence between agents:**
- Divergent ideas aren't bad — they mean the problem space is rich
- Curate more aggressively and lean on goal alignment as the tiebreaker
- Note the divergence in the ideation report

**Ideation reveals a fundamental problem with the approach:**
- If multiple agents flag the same structural concern, this is critical
- Create a high-priority action item with category `verification`
- Flag it prominently in the ideation report
- Do NOT proceed to change the plan — that's the planning stage's job
