---
name: hx:planning-tasks
description: Define and write the atomic implementation tasks needed to deliver a specific story — bite-sized units of work that a specialist agent can complete in a single focused session without context overflow. Given a story (or auto-selecting the next one), reads all context up the full hierarchy (story → epic → milestone → intake), discovers available specialist skills, then launches dual agents (one to propose tasks, one to review and enhance) before writing tasks to planning_tasks.json with skill assignments, complexity ratings, steps, integration tests, and full traceability. Use this skill when the user says "write tasks", "plan tasks", "what tasks does this story need", "break down this story", "define tasks for story-1", "task out this story", or anything about decomposing a story into implementation work. Also trigger after stories are written and the next step is task decomposition, or when the operator routes to task planning.
disable-model-invocation: false
user-invocable: true
---

# Planning Tasks

You define the tasks for a specific story — the atomic implementation steps that a specialist agent will actually sit down and execute. Each task has a clear start, a clear end, and can be finished in a single focused session. When a task is done, the agent can point to a concrete change: a new file, a modified function, a passing test.

Tasks are where the rubber meets the road. Every task you write will be dispatched to a real agent with limited context. If the task is too broad, the agent will lose focus or drift. If the task is too vague, the agent won't know when it's done. If the wrong skill is assigned, the agent will struggle with work outside its expertise. Getting tasks right is what makes the difference between agents that ship and agents that spin.

Your job is to look at a story, understand its acceptance criteria and the behaviour it delivers, and break that into the discrete implementation steps that collectively make the story's acceptance criteria pass. Then write them using the harnessx CLI with the right skill assignments, complexity ratings, and full traceability.

**Scope discipline:** You work on ONE story at a time. Once all tasks for that story are written and verified, you stop. Do not attempt to write tasks for multiple stories in a single session.

---

## Step 1: Confirm active project and identify target story

```bash
harnessx project active
```

If no active project exists, tell the user to set one and stop.

### Determine which story to work on

The user may specify a story directly (e.g., "write tasks for story-3") or you may need to find the next one:

```bash
# If user specified a story, get its full details
harnessx planning-stories list

# If no story specified, get the next incomplete one
harnessx planning-stories next
```

You need the story's full object — especially its `title`, `description`, `acceptance_criteria`, `epic`, and `traces`. The acceptance criteria are your north star — every task you write should contribute toward making at least one criterion pass.

### Pre-built check

Before proceeding, check if this story already has its tasks pre-built:

If the story has `tasks_written: true`, it has pre-built structure from rework milestone generation. Report to the user: "This story already has pre-built tasks (from rework milestone setup). No additional task planning needed." Stop — do not create additional tasks.

---

## Step 2: Read the full context hierarchy

Tasks sit at the bottom of the planning hierarchy, so you need context from every level above. Gather it efficiently — you'll pass this to the dual agents.

### Walk up the hierarchy

```bash
# Get the parent epic
harnessx planning-stories parent <story-id>

# Get the grandparent milestone
harnessx planning-epics parent <epic-id>
```

Understanding the milestone and epic gives you the "why" behind the story. A task that's technically correct but misaligned with the epic's capability goal is a bad task.

### Check existing tasks

```bash
# Tasks already under this story
harnessx planning-stories children <story-id>

# All tasks across all stories (to spot overlaps)
harnessx planning-tasks list
```

If tasks already exist for this story, you're filling gaps — not starting from scratch. Understand what's covered before proposing what's missing.

### Read intake documents

Read in parallel the files most relevant to this story's domain. At minimum:

- `intake_actions.json` — the action items this story traces to (focus on the ones in the story's `traces.tags`)
- `goal.md` and `scope.md` — to stay within bounds
- `success_measures.md` — to ensure tasks ladder up to measurable outcomes
- Any `interview-*.md` files from agents whose domain overlaps this story

All files are in `harnessx/<project-id>/intake/`. Not every file will exist.

### Catalog available specialist skills

This is unique to task planning — you need to know what agents are available so you can assign the right skills to each task. List all skill directories:

```bash
ls .claude/skills/
```

Identify which skill families exist and whether each has a **team lead** (a coordinator skill that can triage and delegate to specialists). Common skill families:

- **`rust:*`** — Rust development team. **Team lead: `rust:team-coordinator`**. Specialists: developing, unit-testing, integration-testing, exploration-and-planning, planning-and-architecture, ergonomic-refactoring, errors-management, commenting
- **`mermaid-diagrams`** — Diagram creation (standalone, no team lead)
- **`research:reducer`** — URL analysis and distillation (standalone, no team lead)
- **Other language/domain teams** — Any skills created during intake team (e.g., `python:*`, `typescript:*`). Check if they have a coordinator or team lead skill.

Read the SKILL.md frontmatter (name + description) of any skill you're uncertain about. The `--skills` flag on task creation takes these skill names — matching them correctly is critical because the wrong agent assignment wastes an entire execution cycle.

### Team lead vs. specialist assignment

When a skill family has a team lead, **assign the team lead by default**. Team leads exist because they understand the full specialist roster and can delegate to the right agent based on what they discover during execution. At planning time, you're making assignment decisions without seeing the code — the team lead makes that decision after exploring it.

**Assign directly to a specialist only when the task is trivially single-concern** — work so focused and simple that a coordinator would just pass it straight through. Examples:
- "Add comments to the position module" → `rust:commenting` (no delegation needed)
- "Clean up iterator chains in the parser" → `rust:ergonomic-refactoring` (one skill, one concern)
- "Add `#[inline]` annotations to hot-path functions" → `rust:ergonomic-refactoring`

**Assign to the team lead for anything else** — implementation, multi-step work, tasks touching architecture decisions, tasks where the right specialist isn't obvious, or tasks with any complexity beyond "low":
- "Implement the position tracker with PnL calculation" → `rust:team-coordinator` (needs exploration, architecture decisions, then implementation)
- "Write integration tests for the websocket feed" → `rust:team-coordinator` (may need exploration first, then test design)
- "Add error types for the ingestion module" → `rust:team-coordinator` (touches architecture patterns)

This principle applies to all domain teams, not just Rust. If a Python team has a `python:team-coordinator`, the same logic applies. The team lead is the safe default; direct specialist assignment is the optimisation for trivial work.

---

## Step 3: Launch dual agents for task analysis

This is the core of the skill — two agents working in sequence to produce a robust task set. The reason for two agents: the first thinks creatively about decomposition, the second thinks critically about whether it's right. Neither alone is sufficient.

### Agent 1: Task Proposer

Launch a subagent with all the context you've gathered. Its job is to propose the complete set of tasks for this story.

```
You are proposing tasks for story [story-id]: "[story title]"

STORY CONTEXT:
- Description: [story description]
- Acceptance Criteria: [list all criteria]
- Epic: [epic-id] — "[epic title]": [epic description]
- Milestone: [milestone-id] — "[milestone title]": [milestone description]
- Traces: [story trace tags and intake sources]

EXISTING TASKS (if any):
[paste existing tasks or "none"]

AVAILABLE SPECIALIST SKILLS:
[list all non-hx skills with their names and one-line descriptions]

RELEVANT ACTION ITEMS:
[paste the action items this story traces to]

Your job: propose a complete set of tasks that, when all are done, make every acceptance criterion for this story pass. For each task, provide:

1. **Title** — specific, actionable (verb + noun + context)
2. **Steps** — ordered implementation steps (3-7 per task). Each step should be concrete enough that an agent can follow it without guessing
3. **Complexity** — super-low, low, medium, high, or super-high
4. **Skills** — which specialist skill(s) should execute this task
5. **Integration tests** — how to verify this specific task works (not the whole story — just this task's contribution)
6. **Dependencies** — which other proposed tasks must be done first
7. **Output sources** — which files this task will create or modify
8. **Trace tags** — which action items this task implements
9. **Notes** — implementation hints, edge cases, risk flags

GUIDELINES:
- Tasks must be BITE-SIZED. An agent should be able to complete one in a single focused session without running out of context. If a task requires understanding more than 2-3 files deeply, it's probably too broad.
- Each task should touch ONE concern — one module, one function cluster, one data transformation. Mixing concerns (e.g., "implement the parser AND write the error types") creates context bloat.
- Skill assignment matters enormously. When a team lead exists (like rust:team-coordinator), assign it by default — it can explore and delegate to the right specialist. Only bypass the team lead for trivially single-concern tasks (commenting, simple refactoring) that clearly map to one specialist.
- Steps are the agent's roadmap. Write them as instructions, not descriptions. "Read the existing Position struct in src/models/position.rs" not "Understand the data model."
- Complexity should reflect what the AGENT will experience, not the conceptual difficulty. A simple but tedious task (many similar changes) might be "medium" because it requires sustained attention. A clever but small algorithm might be "low" because it's a few lines.
- Every acceptance criterion must be addressed by at least one task. Check coverage.
- Order tasks by dependency — foundational work first, then work that builds on it.
```

### Agent 2: Task Reviewer

After receiving the proposer's output, launch a second subagent to critique and enhance.

```
Review these proposed tasks for story [story-id]: "[story title]"

STORY CONTEXT:
[same context block as Agent 1]

PROPOSED TASKS:
[paste Agent 1's output]

Validate with these specific checks:

1. **Acceptance criteria coverage** — Map each acceptance criterion to the task(s) that address it. Flag any criterion not covered by any task. Flag any criterion only partially covered.

2. **Bite-size check** — Would any task require an agent to hold more than 2-3 files in context simultaneously? If so, it needs splitting. Would any task take more than one focused session? If so, it's too broad.

3. **Skill assignment accuracy** — Is each task assigned to the right skill? Common mistakes:
   - Assigning a non-trivial task directly to a specialist (like rust:developing) when a team lead exists (like rust:team-coordinator) that could explore the codebase and delegate intelligently
   - Assigning a trivially simple task (like "add comments to module X") to the team coordinator when a direct specialist (rust:commenting) would be more efficient
   - Forgetting rust:errors-management when new error variants are needed (though the team coordinator can also handle this)
   - The general rule: if a team lead exists and the task has any complexity beyond "low" or touches multiple concerns, the team lead should be assigned

4. **Step quality** — Are steps concrete enough for an agent to follow? Could an agent who has never seen this codebase execute step 1 without ambiguity? If steps say "implement the logic" without specifics, they need rewriting.

5. **Dependency correctness** — Are there circular dependencies? Could any tasks be parallelized that are currently serialized? Are there missing dependencies (task B uses output from task A but doesn't depend on it)?

6. **Integration test quality** — Does each task have a way to verify it worked? Are the tests specific enough to catch real failures?

7. **Complexity calibration** — Are complexity ratings consistent? A task with 3 steps that modifies one file shouldn't be "high" while a task with 7 steps across multiple files is "medium."

8. **Missing tasks** — Is there work implied by the acceptance criteria that nobody planned for? Common gaps:
   - Error handling for the happy path
   - Edge cases mentioned in acceptance criteria but not tasked
   - Integration between newly created components
   - Cleanup or refactoring needed to make new code fit existing patterns

Provide specific, actionable critique. For each issue, explain what's wrong and how to fix it. If a task needs splitting, show the split. If steps are vague, rewrite them. If a skill assignment is wrong, name the correct skill and why.
```

### Incorporate feedback

After receiving the reviewer's critique, synthesize both outputs. The reviewer's job is to catch problems; your job is to decide which critiques to act on and produce the final task list. Not every critique requires a change — use your judgment.

---

## Step 4: Write tasks via CLI

Before writing, read the CLI reference. Read `docs/planning-tasks.md` if you haven't this session — especially the notes about pipe-separated fields.

### Creating tasks

Use `harnessx planning-tasks create` for each task. The CLI auto-assigns IDs (`task-1`, `task-2`, ...) and auto-increments `order`.

```bash
harnessx planning-tasks create \
  --title "Write the GraphQL query for fetching open positions" \
  --steps "Read the Uniswap v3 subgraph schema documentation at the URL in action-1's input_docs | Write a GraphQL query that filters positions by owner address where liquidity > 0 | Add pagination handling using the skip/first pattern for the 1000-entity limit | Write the query as a const string in src/ingestion/queries.rs" \
  --story "#story-1" \
  --status not_started \
  --complexity low \
  --mode plan \
  --skills "rust:team-coordinator" \
  --integration-tests "Query returns all positions for a known wallet with active positions | Query correctly paginates when wallet has more than 1000 positions | Query returns empty results for a wallet with no positions" \
  --trace-tags "#action-1" \
  --trace-intake-sources "#intake-resources" \
  --trace-output-sources "src/ingestion/queries.rs" \
  --note "Only fetch positions where liquidity > 0 — closed positions should be excluded."
```

### Critical: steps and integration-tests are pipe-separated

Both `--steps` and `--integration-tests` use **pipe (`|`) separators** — not commas. This is because steps and test descriptions commonly contain commas in their text.

```bash
# CORRECT — pipe-separated
--steps "Read the schema docs | Write the query | Add pagination"
--integration-tests "Query returns positions for wallet X | Empty wallet returns empty list"

# WRONG — comma-separated (will split incorrectly)
--steps "Read the schema docs, Write the query, Add pagination"
```

### Other important details

- **`--story` uses the `#` prefix** — e.g., `--story "#story-1"`.
- **`--depends-on` is comma-separated** — e.g., `--depends-on "task-1, task-2"`.
- **`--skills` is comma-separated** — e.g., `--skills "rust:developing, rust:unit-testing"`.
- **`--complexity`** — one of: `super-low`, `low`, `medium`, `high`, `super-high`, `uncertain`.
- **`--mode`** — starts as `plan` during planning. Changes to `execute` when an agent begins work.
- **`--trace-output-sources`** — file paths the task is expected to create or modify. This is unique to tasks and critical for traceability from planning to code.
- **IDs are auto-assigned** — capture the returned ID for dependency and tagging.
- **Update flags replace** (except notes, which append).

### Writing tasks in dependency order

Write tasks in the order they should be executed. If a task depends on another:

```bash
harnessx planning-tasks create \
  --title "Parse subgraph response into typed Position structs" \
  --depends-on "task-1" \
  --skills "rust:developing" \
  ...
```

### Updating existing tasks

```bash
harnessx planning-tasks update task-1 \
  --steps "Updated step 1 | Updated step 2 | New step 3" \
  --note "Added step 3 after review identified missing pagination edge case."
```

Remember: `--steps` and `--integration-tests` on update **replace** the existing list. Include all items, not just new ones.

---

## Step 5: Tag intake artifacts with task references

After creating tasks, add `#task-N` tags back into intake documents for bidirectional traceability.

### Tag the intake markdown files

Find the paragraphs in intake markdown that relate to each task and add the tag inline.

**Example — tagging `scope.md`:**

```markdown
The ingestion system must handle Uniswap v3 subgraph queries with pagination for large wallets. #action-1 #story-1 #task-1
```

### Tag action items with task references

For each action item a task traces to, append the task tags using `add-tag`. This creates the reverse link without replacing any existing tags.

```bash
harnessx intake-actions add-tag action-1 --tags "#task-1, #task-2"
harnessx intake-actions update action-1 \
  --note-author "hx-planning-tasks" \
  --note-text "Mapped to task-1: Write GraphQL query, task-2: Parse response."
```

`add-tag` only appends — it will not remove any existing tags on the action item, and it skips duplicates.

### Tag stories with their tasks

Update the story's trace tags to include task references:

```bash
harnessx planning-stories update story-1 \
  --trace-tags "#action-1, #action-4, #task-1, #task-2, #task-3" \
  --note "Tasks defined: GraphQL query, response parsing, error handling."
```

### Tagging rules (from hx:tag-context-writing)

- Tags go at the **end of the line** they annotate — never on their own line
- Only use **traceable tags** (`#task-N`, `#story-N`, `#epic-N`, `#milestone-N`, `#action-N`)
- Do not invent categorical tags
- Verify with `harnessx context search-context --query "#task-1"` — should return meaningful paragraphs

---

## Step 6: Verify and stop

After writing all tasks and tagging artifacts, verify completeness — then stop.

```bash
# Verify tasks were created correctly
harnessx planning-tasks list

# Verify the story sees its children
harnessx planning-stories children <story-id>

# Verify each task is findable
harnessx context search-context --query "#task-1"
harnessx context search-context --query "#task-2"
# ... for each task
```

### Completeness check

Verify:
- Every acceptance criterion in the story is addressed by at least one task
- Every task has steps (no empty step lists)
- Every task has at least one integration test
- Every task has a skill assignment
- Every task has a complexity rating
- Every task has at least one trace tag back to an action item
- Tasks are collectively sufficient — all done means every acceptance criterion passes
- No task overlaps significantly with a task under another story
- Dependencies form a valid DAG (no cycles)

If you find gaps, go back and fix them.

### Then stop

Once tasks for this story are written and verified, you are done. Do not continue to the next story. Do not start writing tasks for other stories. The operator or user will invoke this skill again for the next story.

---

## What good tasks look like

- **Action-oriented titles** — "Write the GraphQL query for fetching positions" not "GraphQL work"
- **Concrete steps** — an agent can follow them without guessing. "Read src/models/position.rs to understand the Position struct" not "understand the data model"
- **Right-sized** — completable in one focused session, touching 1-3 files. If it needs more, split it
- **Correct skill assignment** — team lead for non-trivial work, direct specialist only for trivially simple tasks
- **Verifiable** — integration tests describe how to check the task actually worked
- **Accurate complexity** — reflects what the agent will experience, not conceptual difficulty
- **Clean dependency chain** — foundational work ordered first, no cycles
- **Full traceability** — traces to action items and intake sources, output sources point to expected files

---

## What bad tasks look like (and how to fix them)

| Bad task | Problem | Fix |
|----------|---------|-----|
| "Implement the data layer" | Too broad — multiple concerns | Split: "Write Position struct", "Write DB adapter", "Write query builder" |
| "Set up the project" | Not a behavioural step | Usually absorbed into the first real task's steps |
| "Write tests" | Too vague — test what? | "Write unit tests for the position normalizer covering empty input, single position, and multi-DEX scenarios" |
| Skills: `rust:developing` for a complex multi-file task | Direct specialist for non-trivial work skips exploration and planning | Use the team lead `rust:team-coordinator` — it will explore, plan, and delegate |
| Skills: `rust:team-coordinator` for "add comments to module" | Coordinator overhead for trivially simple work | Use `rust:commenting` directly — no delegation needed |
| Complexity: `uncertain` | Planning should resolve uncertainty | Research the task enough to rate it, or flag it for the proposer agent to investigate |
| No integration tests | Can't verify the task worked | Every task needs at least one test — even "the module compiles without errors" |

---

## What this skill does NOT do

- **Write stories, epics, or milestones** — those must exist before tasks; use the corresponding planning skills
- **Execute any implementation** — tasks are plans with steps and skill assignments, not code
- **Write tasks for multiple stories** — scope is one story per invocation
- **Change the pipeline stage** — the operator handles stage transitions
- **Create action items** — action items come from intake; tasks trace to existing ones
- **Bypass team leads for non-trivial work** — when a domain has a team lead (like `rust:team-coordinator`), assign the team lead by default. Only assign directly to a specialist for trivially simple, single-concern tasks
