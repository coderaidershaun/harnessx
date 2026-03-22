# Planning Hierarchy: Milestones, Epics, Stories, and Tasks

This document defines the four levels of work breakdown used by the harnessx planning agent. Each level answers a different question, operates at a different granularity, and serves a different purpose in the pipeline.

---

## Milestone

**Answers:** *"What does done look like at this checkpoint?"*

A milestone is a demonstrable state of the project — not a task, not a deliverable, but a moment where you could show someone the system and they'd see real, observable progress. Milestones are sequenced because later ones depend on earlier ones being true. They map naturally to the success measures and UAT criteria captured during intake.

A project should have roughly 3–7 milestones. More than that and they're too granular; fewer and there's no meaningful checkpoint between "started" and "done."

### Properties

| Field | Description |
|---|---|
| `id` | Sequential milestone ID (e.g., `milestone-1`) |
| `title` | Short, state-based description of what's true when this is reached |
| `description` | What someone would observe if they looked at the project at this checkpoint |
| `depends_on` | IDs of milestones that must be complete before this one is reachable |
| `success_measures` | References to intake success measures this milestone satisfies |
| `uat_criteria` | References to intake UAT scenarios this milestone enables |

### Example

> **Milestone: Live position data flowing through the pipeline and visible in a basic UI.**
>
> You could sit someone down and show them this is working. It's not "build the data pipeline" (that's work to be done). It's the state the project is in once certain work is finished. A reviewer can observe positions appearing in real time and verify correctness against a known source.

---

## Epic

**Answers:** *"What capability do we need to build to reach a milestone?"*

An epic is a coherent chunk of functionality that, when complete, gives the system a new capability it didn't have before. Epics belong to a milestone. They're independent enough that you could describe one to someone without referencing the others, but they collectively make the milestone true.

An epic typically maps to one or more action items from intake, and it often spans multiple categories (an ingestion epic might touch backend, infrastructure, and integration work).

### Properties

| Field | Description |
|---|---|
| `id` | Sequential epic ID (e.g., `epic-1`) |
| `title` | The capability being built |
| `description` | What the system can do once this epic is complete that it couldn't do before |
| `milestone` | The milestone this epic contributes to |
| `action_items` | References to intake action items this epic addresses |
| `categories` | Areas of work involved (e.g., `backend`, `infrastructure`, `integration`) |

### Example

> **Epic: DEX position ingestion** (under the live data milestone)
>
> The system can pull live positions from on-chain data sources, normalize them into a common format, and make them available to downstream consumers. This is one of three epics — alongside "PnL calculation engine" and "Position list view" — that together make the milestone true. None of them alone is sufficient; all three are necessary.

---

## Story

**Answers:** *"What's one thing a user or system can do once this is finished?"*

A story is a single, testable behaviour. When it's done, you can write one sentence describing what changed: "the system now does X" or "a user can now do Y." Stories belong to an epic. They're small enough that completing one doesn't require a context switch — you're working in one area of the codebase, one domain concept, one concern.

The test for whether something is a story vs. a task: if completing it produces a meaningful change in what the system can do (even a small one), it's a story. If it's a step toward that change, it's a task.

### Properties

| Field | Description |
|---|---|
| `id` | Sequential story ID (e.g., `story-1`) |
| `title` | One-sentence description of the new behaviour |
| `description` | What's testable once this story is complete |
| `epic` | The epic this story belongs to |
| `acceptance_criteria` | Concrete conditions that must be true for the story to be considered done |
| `action_items` | References to intake action items this story implements |

### Example

> **Story: Fetch open positions from Uniswap v3 subgraph** (under DEX position ingestion)
>
> When this is done, you can run the ingestion module and see live Uniswap v3 positions come back as structured data. It's a meaningful change — the system can now talk to a real data source — but it's scoped to a single integration point.
>
> Another story in the same epic: **Normalize position data into a common Position struct.** When done, downstream code has a clean type to work with regardless of which DEX the position came from.

---

## Task

**Answers:** *"What's the next concrete piece of work I sit down and do?"*

A task is an atomic unit of implementation. It has a clear start, a clear end, and you could finish it in a single focused session. Tasks belong to a story. They're where the harnessx action items ultimately land — the thing a specialist agent actually executes.

Tasks are where complexity ratings from action items matter most — a `super-low` task is a 15-minute change, a `high` task might need the full exploration → architecture → implementation → test pipeline from the Rust team.

### Properties

| Field | Description |
|---|---|
| `id` | Sequential task ID (e.g., `task-1`) |
| `title` | Specific, actionable description of the work |
| `description` | What's being done and why — enough context for an agent to execute without conversation history |
| `story` | The story this task belongs to |
| `complexity` | `super-low`, `low`, `medium`, `high`, `super-high`, or `uncertain` |
| `skills` | Which specialist skill(s) are needed to execute this task |
| `action_items` | References to intake action items this task fulfils |
| `mode` | Current phase: `plan`, `execute`, `review`, or `rework` |

### Example

> **Tasks under "Fetch open positions from Uniswap v3 subgraph":**
>
> 1. **Define the GraphQL query for open positions** — Write the subgraph query that returns all active liquidity positions with their token pairs, ranges, and liquidity amounts. (`complexity: low`)
> 2. **Implement the HTTP client with retry logic** — Build a client that handles subgraph rate limits, transient failures, and timeout with exponential backoff. (`complexity: medium`)
> 3. **Parse subgraph response into raw position data** — Deserialize the JSON response into typed Rust structs, handling missing fields and schema variations. (`complexity: low`)
>
> Each task is one sitting, one concern, one agent dispatch.

---

## How the Planning Agent Uses This Hierarchy

The planning agent reads all intake documents and action items, then works top-down:

1. **Identify milestones** by examining success measures, UAT criteria, and the natural dependency order of the project. Milestones are the checkpoints the user cares about.

2. **Group action items into epics** — clusters of related work that serve a milestone. Epics are capabilities, not categories. "Backend work" is not an epic; "PnL calculation engine" is.

3. **Define stories within each epic** as individual behavioural increments. Each story is a testable change in what the system can do.

4. **Break stories into tasks**, which is where the original action items from intake get their final home. Some action items become tasks directly, some get split across multiple tasks, and some become stories in their own right.

The output is a structured plan where every piece of work traces back to an intake action item and forward to a milestone, with no gaps and no orphans.
