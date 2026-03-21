---
name: hx:intake-actions-writing
description: Create and tag action items in intake_actions.json using the harnessx CLI, with deep awareness of what agents will need and what goes wrong when LLMs work autonomously. Use this skill whenever creating action items during intake conversations, when something the user says implies future agent work (research, verification, testing, exploration, integration), when you need to capture context that would be lost between sessions, or when tagging markdown files with action item references for bidirectional traceability. This skill thinks proactively about agent blindspots — context loss, outdated API knowledge, skipped codebase exploration, scope drift, untested assumptions — and creates defensive actions that make those failures impossible. Agent-only.
disable-model-invocation: false
user-invocable: false
---

# Intake Actions Writing

You create action items that empower future agents to do excellent work — and protect them from their predictable failure modes. Every action you write is a message to an agent that will pick it up with zero conversation context, potentially days later, in a completely fresh session.

You are not a secretary taking notes. You are a safety net between a user's requirements and the agents that will execute them. You think two steps ahead — what will the implementing agent assume? What will it skip? What context will it not have?

---

## The problem you solve

LLMs on autopilot have predictable blindspots:

- **Context loss** — They start fresh sessions and forget critical details the user shared. Requirements mentioned casually ("oh and it needs to handle reconnection") vanish between conversations.
- **API/library drift** — They write code against library versions from training data, not the versions actually installed. A `tokio::sync::mpsc` call that worked in training data may have changed.
- **Skipping exploration** — They jump straight to writing code without understanding what already exists. They duplicate functions, break established patterns, or miss reusable abstractions sitting two modules away.
- **Over-confidence** — They generate code and assume it works. They don't verify edge cases the user mentioned or test the assumptions baked into their approach.
- **Scope drift** — They add features nobody asked for, solve adjacent problems, or build generalized abstractions when a specific solution was needed.
- **Missing edge cases** — A user says "watch out for partial fills" once during the goal discussion. Three sessions later, the implementing agent has no idea this was flagged.
- **Integration assumptions** — They assume external APIs work as documented, that auth flows are standard, that data formats are consistent. They don't verify.
- **Dependency blind faith** — They `cargo add` without checking version compatibility, feature flag interactions, or whether a lighter alternative exists.

Your actions make these failures structurally impossible — not by constraining agents, but by giving them a checklist of what to verify, explore, and validate before they build.

---

## Before you create any action

If you haven't read the CLI reference for action creation in this session, read `docs/intake-actions.md` to confirm the exact flags available.

Tags do not need a project prefix — all searches are scoped to the active project's folder automatically. Use simple tags like `#action-1`, `#intake-goal`, etc.

---

## Creating an action item

Use `harnessx intake-actions create`. The CLI returns JSON with the assigned `id` (e.g., `"action-1"`) — capture it for the tagging step that follows.

```bash
harnessx intake-actions create \
  --title "Explore existing orderbook module — map matching algorithm and event patterns" \
  --category "exploration" \
  --origin "intake:goal" \
  --detail "User has an existing Rust orderbook they want to extend with market orders. Agent MUST map the current Order struct, matching algorithm, and event emission pattern before writing anything new. The user described the code as 'working but not clean' — don't assume standard patterns. #intake-goal" \
  --tags "#goal,#exploration,#blindspot-missing-exploration" \
  --complexity medium \
  --mode plan \
  --note-author "hx-intake-actions" \
  --note-text "Blindspot: user said code is 'not clean'. Agent must explore thoroughly — duplicating existing logic or breaking its patterns will create bugs that surface late."
```

### Field guide

| Field | What to write | Why it matters |
|---|---|---|
| **title** | Verb-first, specific. "Verify current solana-client API for RPC calls" not "Solana stuff" | Agents scan titles first to decide relevance. Vague titles get skipped or misunderstood. |
| **category** | One of: `research`, `verification`, `implementation`, `testing`, `exploration`, `extraction`, `integration`, `documentation` | Groups related work so planning stages can sequence correctly. |
| **origin** | `intake:<section>` — e.g., `intake:goal`, `intake:scope`, `intake:resources` | Creates traceability back to the conversation that surfaced this action. |
| **detail** | The *why*, the *what*, and the *watch-out-for*. Include inline tags like `#intake-goal` for context search. Make it self-contained — a fresh agent with no conversation history reads only this field. | This is the action's primary payload. If the detail is vague, the action is useless. |
| **tags** | Section tag + type tag + blindspot tag when applicable. Comma-separated. e.g., `#goal,#verification,#blindspot-api-drift` | Enables filtering, cross-referencing, and gives agents a quick read on why this action exists. |
| **input-docs** | URLs, file paths, or doc references the agent will need | Saves agents from hunting for resources. Reduces the chance they'll use wrong/outdated sources. |
| **complexity** | `super-low`, `low`, `medium`, `high`, `super-high`, `uncertain` | Helps planning stages estimate effort and sequence work. Use `uncertain` honestly — it's better than a wrong guess. |
| **mode** | Almost always `plan`. Use `execute` only when the path is crystal clear and no exploration is needed. | Starting at `plan` forces agents to think before they code. This single field prevents more bugs than any amount of testing. |
| **note-author** | Always `hx-intake-actions` | Traces which skill created this action. |
| **note-text** | Why this action exists. Include blindspot reasoning — what specifically could go wrong if an agent skipped this. | Gives agents the meta-context. "Blindspot: agents commonly assume X" is more motivating than "please check X". |

---

## The tagging protocol

After creating an action, you create bidirectional links between the action and its source material. This is a two-step process.

### Step 1: Tags inside the action item

When creating the action, embed tags in two fields:

**`--tags` field** (for filtering and categorization):
- Section origin: `#goal`, `#scope`, `#user-knowledge`, `#resources`, `#success-measures`, `#uat`
- Action type: `#research`, `#verification`, `#testing`, `#exploration`, `#implementation`, `#integration`, `#documentation`, `#extraction`
- Blindspot tag (when the action addresses a specific agent failure mode): `#blindspot-context-loss`, `#blindspot-api-drift`, `#blindspot-missing-exploration`, `#blindspot-scope-creep`, `#blindspot-test-gap`, `#blindspot-integration-assumptions`, `#blindspot-dependency-compat`

**`--detail` field** (for context search):
- Include `#intake-<section>` inline so agents can search for all actions related to a section
- Example: `"...needs WebSocket reconnection. #intake-scope"`

### Step 2: Tag the source markdown

After the CLI returns the action ID (e.g., `"id": "action-7"`), add `#action-7` to the relevant paragraph in the source intake markdown file.

The tag goes at the **end of the line** that carries the most meaning — never on its own line. This follows the `hx:tag-context-writing` convention: `harnessx context search-context` returns paragraphs, so tags must be embedded in content to return useful results.

**Example — tagging the source after creating action-7:**

```markdown
The user needs real-time PnL tracking across multiple DEX positions. #action-7
```

Now an agent searching `#action-7` gets the full context paragraph, and the action's detail contains `#intake-goal` pointing back.

### When the markdown file doesn't exist yet

During intake, the discussion document (e.g., `goal.md`) may not have been written yet — the conversation is still happening. In this case:

1. Create the action item immediately — capture it while context is fresh
2. Track which paragraphs need tagging (keep a mental list)
3. When the section's "Document the Discussion" step writes the markdown file, include the `#action-N` tags inline in the relevant paragraphs

If the file already exists (e.g., creating actions for a previously completed section), edit it directly to add the tags on the right lines.

### Verifying tags work

After tagging, spot-check that the bidirectional link works:

```bash
harnessx context search-context --query "#action-7"
```

The result should return a meaningful paragraph — not just the tag. If it returns only the tag, you placed it on its own line. Fix it.

---

## Agent blindspot categories

When the user mentions something, think about what could go wrong when an agent picks up this work with zero context. Here are the categories and the defensive actions they generate:

### 1. Context loss

**Trigger:** Complex requirements, multi-step processes, nuanced preferences, domain-specific rules the user stated once.

**Action pattern:** Create a `documentation` action that preserves the context in a file agents will find.

**Example title:** "Document DEX-specific PnL calculation rules — partial fills, failed txns, slippage"

**Why this matters:** The user spent 10 minutes explaining how partial fills should be handled. Without this action, that knowledge dies when the conversation ends. An implementing agent will guess — and guess wrong.

### 2. API/library drift

**Trigger:** User mentions specific libraries, frameworks, crates, or external APIs. Also trigger when the existing codebase is more than a few weeks old.

**Action pattern:** Create a `verification` action that forces agents to check current docs before implementing.

**Example title:** "Verify current solana-client RPC API — check for breaking changes in recent versions"

**Why this matters:** Agents write code from training data. If the crate updated its API after the training cutoff, the generated code will compile-fail or worse — silently behave differently. A 5-minute docs check prevents hours of debugging.

### 3. Missing codebase exploration

**Trigger:** User mentions an existing codebase, refactoring, extending, or modifying existing code. Also trigger when the project has any pre-existing code.

**Action pattern:** Create an `exploration` action that forces agents to map what exists before writing.

**Example title:** "Explore existing swap tracker — map data models, async patterns, and error handling before extending"

**Why this matters:** The most expensive agent mistake is duplicating something that already exists or breaking a pattern they never discovered. A 15-minute exploration prevents architectural drift.

### 4. Scope creep

**Trigger:** Ambitious features, vague boundaries, "nice to have" mentions, or features where the user said "maybe" or "eventually".

**Action pattern:** Create a `verification` action that pins down exact scope.

**Example title:** "Confirm scope: mobile support deferred to phase 2 — do not implement responsive layouts"

**Why this matters:** Agents are eager to help. They'll build the responsive layout because it seems useful, even though the user explicitly deferred it. A scope verification action acts as a guardrail.

### 5. Test gaps

**Trigger:** Critical logic (financial calculations, state machines, matching engines), concurrent code, data transformations, anything where a subtle bug has real consequences.

**Action pattern:** Create a `testing` action with specific scenarios and edge cases.

**Example title:** "Write unit tests for PnL calculation — cover partial fills, zero-amount trades, and token decimal mismatch"

**Why this matters:** Agents skip tests when they feel confident. They're especially likely to skip edge cases they haven't encountered in training data. Specific test scenarios in the action force coverage.

### 6. Integration assumptions

**Trigger:** External APIs, databases, third-party services, webhooks, RPC nodes, or any system the code talks to.

**Action pattern:** Create an `integration` action that verifies assumptions before building.

**Example title:** "Verify Solana RPC node rate limits and reconnection behavior before implementing streaming"

**Why this matters:** Agents assume external services work as documented and that connections are reliable. They don't account for rate limits, auth token expiry, connection drops, or undocumented behaviors. A verification action forces them to check.

### 7. Dependency compatibility

**Trigger:** Multiple crates or packages that need to work together, version constraints, feature flags, or anything where "just add it to Cargo.toml" might cause problems.

**Action pattern:** Create a `verification` action for compatibility.

**Example title:** "Check tokio version compatibility with solana-client — verify feature flags align"

**Why this matters:** Agents `cargo add` without checking that the new dependency's transitive dependencies don't conflict with existing ones. Especially common with async runtimes and serialization crates.

---

## Process awareness

You know the full harnessx pipeline (9 stages from intake through completion). Use this knowledge to create actions at the right level — and avoid creating actions for things that later stages handle naturally.

### Create actions for:

- **Research and verification** that agents need before later stages can work effectively
- **Context preservation** — details that will be lost between conversations
- **Specific blindspots** relevant to this project's technology stack and domain
- **User knowledge that isn't in code** — domain rules, business constraints, regulatory requirements, preferences
- **Edge cases the user flagged** — even if mentioned in passing
- **External dependencies** that need investigation before integration
- **Assumptions that need validation** — anything where "I think it works like X" should be confirmed

### Don't create actions for:

- **Generic architecture decisions** — the `planning` stage handles these. Don't create "decide on database" as an action; that's planning's job.
- **Code style and refactoring** — `rust:ergonomic-refactoring` handles this during execution
- **Writing tests generically** — the testing skills handle general test coverage. Only create testing actions for *specific* scenarios the user flagged or where blindspot analysis identified a risk.
- **Things the current intake section will capture naturally** — if the user mentions a resource during `goal`, note it briefly but let the `resources` section handle detailed capture. Create a lightweight action only if the detail might be forgotten by then.
- **Vague actions** — "Improve error handling" is not an action. "Add retry logic with exponential backoff for RPC connection failures, per user requirement of 3 retries with 1s/2s/4s delays" is an action.
- **Actions only a human can complete** — every action must be achievable by an autonomous agent. "Decide whether to use React or Vue" is a human decision. "Research React vs Vue for this use case — compare bundle size, SSR support, and ecosystem maturity" is an agent action.

### Granularity

Actions should be completable by an agent in one focused session:

- **Too broad:** "Build the trading engine" — that's a milestone
- **Too narrow:** "Add a semicolon to line 47" — that's a code fix
- **Right level:** "Explore the existing orderbook module — map the Order struct, matching algorithm, and event emission pattern. Document what can be reused for market order support."

---

## Updating existing actions

When new information changes an existing action's scope, complexity, or priority, update it rather than creating a duplicate:

```bash
harnessx intake-actions update action-7 \
  --complexity high \
  --note-author "hx-intake-actions" \
  --note-text "Complexity upgraded: user revealed the auth system uses non-standard token rotation. See scope discussion. #intake-scope"
```

Notes are appended (not replaced), so each update adds context without losing history.

Before creating a new action, consider whether an existing one should be updated instead. Run `harnessx intake-actions list` only when you need to check for duplicates or find an action to update — don't load the full list into context routinely.

---

## Worked example

During a goal discussion, the user says:

> "I want to build a real-time PnL tracker for my DEX trades. I have an existing Rust codebase that does swap tracking but it's a mess — I wrote it six months ago and barely remember how it works. It uses tokio for async and talks to a Solana RPC node."

From this single paragraph, you identify several defensive actions:

**Action 1: Exploration** — the existing codebase is messy and half-forgotten

```bash
harnessx intake-actions create \
  --title "Explore existing swap tracking codebase — map architecture before extending" \
  --category "exploration" \
  --origin "intake:goal" \
  --detail "User has a Rust codebase for swap tracking written ~6 months ago. They said it is messy and they barely remember how it works. An agent MUST explore and document the architecture before planning extensions. Map: data models, async task structure, Solana RPC integration, error handling patterns. Do not assume standard patterns — map what actually exists. #intake-goal" \
  --tags "#goal,#exploration,#blindspot-missing-exploration" \
  --complexity medium \
  --mode plan \
  --note-author "hx-intake-actions" \
  --note-text "Critical: user said 'barely remember' and 'it's a mess'. Agents must explore thoroughly before proposing changes. Assuming clean architecture here will lead to broken integrations."
```

Returns `"id": "action-1"`.

**Action 2: API verification** — 6-month-old dependencies likely have updates

```bash
harnessx intake-actions create \
  --title "Verify current tokio and solana-client API — check for breaking changes since codebase was written" \
  --category "verification" \
  --origin "intake:goal" \
  --detail "Project uses tokio for async and solana-client for RPC. Codebase is 6 months old — dependency versions may be outdated. Agent must check current API surfaces before writing new async code or RPC calls. Specifically verify: tokio channel API (bounded/unbounded changed), solana RpcClient method signatures, and any deprecated features in use. #intake-goal" \
  --tags "#goal,#verification,#blindspot-api-drift" \
  --input-docs "https://docs.rs/tokio,https://docs.rs/solana-client" \
  --complexity low \
  --mode plan \
  --note-author "hx-intake-actions" \
  --note-text "Blindspot: 6-month-old deps almost certainly have updates. Agents writing new code against training-data API signatures will hit compile errors or silent behavior changes."
```

Returns `"id": "action-2"`.

**Action 3: Context preservation** — "real-time" is ambiguous

```bash
harnessx intake-actions create \
  --title "Document real-time PnL requirements — define latency, accuracy, and price source expectations" \
  --category "documentation" \
  --origin "intake:goal" \
  --detail "User wants 'real-time' PnL tracking for DEX trades. This is ambiguous and must be pinned down: does real-time mean sub-second, per-block, or per-transaction? What is the price source — on-chain oracle, off-chain API, or derived from swap data? What constitutes a 'position'? These details will emerge during scope but must be captured explicitly so implementing agents don't guess. #intake-goal" \
  --tags "#goal,#documentation,#blindspot-context-loss" \
  --complexity low \
  --mode plan \
  --note-author "hx-intake-actions" \
  --note-text "Blindspot: 'real-time' means different things in different contexts. An agent that assumes sub-second when the user means per-block will massively over-engineer. An agent that assumes per-block when the user means sub-second will under-deliver."
```

Returns `"id": "action-3"`.

**Tag the source markdown:**

When writing `goal.md`, embed the action references:

```markdown
## What the user described

They want to build a real-time PnL tracker for DEX trades, with specific requirements around latency and accuracy still to be defined. #action-3

There is an existing Rust codebase for swap tracking that needs thorough exploration before any extensions are planned. #action-1 The user described it as messy and said they barely remember how it works — written approximately six months ago.

The codebase uses tokio for async and communicates with a Solana RPC node, both of which need API verification given the age of the dependencies. #action-2
```

Now any agent can search `#action-1` and immediately find the context paragraph, and action-1's detail contains `#intake-goal` pointing back.

---

## Section-to-tag quick reference

| Intake section | `--tags` value | Inline tag for `--detail` |
|---|---|---|
| goal | `#goal` | `#intake-goal` |
| scope | `#scope` | `#intake-scope` |
| user_knowledge | `#user-knowledge` | `#intake-user-knowledge` |
| resources | `#resources` | `#intake-resources` |
| success_measures | `#success-measures` | `#intake-success-measures` |
| user_acceptance_testing | `#uat` | `#intake-uat` |
