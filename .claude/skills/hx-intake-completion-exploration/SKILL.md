---
name: hx-intake-completion-exploration
description: Deep-dive exploration of all project resources collected during intake — dispatches multi-agents to explore codebases, documents, APIs, and research materials, then produces thorough notes and action items with bidirectional tagging. Use this skill when the pipeline reaches the intake_completion stage, when intake-completion exploration needs to run, or when the user says "explore my resources", "deep dive the codebase", "analyze the repos", "review the research". Also trigger when the operator routes to the intake_completion stage or when all intake_team sections are complete and the pipeline advances.
disable-model-invocation: false
user-invocable: false
---

# Intake Exploration

You are the exploration orchestrator. Your job is to deeply understand every resource the user provided during intake, produce comprehensive notes that future agents can rely on, and create action items that capture what was learned and what still needs to happen.

This skill runs after intake onboarding and intake team are complete. By now, the project has a clear goal, defined scope, collected resources, success measures, UAT criteria, and a team of specialist skills ready to work. Your job is the bridge between "we know what we want" and "we understand what we have to work with."

## Why This Matters

Agents that skip exploration make expensive mistakes: they reinvent code that already exists, misunderstand data formats, build on wrong assumptions about APIs, or duplicate effort across modules. Every hour spent exploring saves many hours of rework during execution. Be thorough — this is the last stop before planning begins.

---

## Startup Sequence

Run these commands and read the outputs carefully:

```bash
harnessx project active
harnessx intake-onboarding status
harnessx intake-completion status
harnessx intake-actions list
```

Then read ALL intake markdown files in `harnessx/<project-id>/intake/`:
- `goal.md` — the project goal and metadata
- `scope.md` — what's in and out of scope
- `user_knowledge.md` — user's background and expertise
- `resources.md` — collected resources with planned follow-up actions
- `success_measures.md` — what "done" looks like
- `uat.md` — user acceptance testing plan
- Any interview markdowns (`interview-*.md`)

Also read `harnessx/<project-id>/intake/intake_actions.json` to see existing action items — especially those with `origin: "intake:resources"` which describe the exploration work that was already planned.

Build a mental model of: what is the project trying to achieve, what resources exist, what has the user already told us about those resources, and what do the specialist agents need to know.

---

## Resource Classification

After reading all intake docs, classify each resource and plan the exploration strategy.

### Resource Types and Agent Assignment

| Resource Type | Agent Skill | Exploration Depth | Notes Subfolder |
|---------------|-------------|-------------------|-----------------|
| **Rust codebase** | `rust:exploration-and-planning` | Extremely thorough | `exploration/code-<repo-name>/` |
| **Other codebase** (Python, JS, etc.) | General-purpose agent with architecture focus | Extremely thorough | `exploration/code-<repo-name>/` |
| **API documentation** | General-purpose agent | Thorough | `exploration/api-<service-name>/` |
| **URL (article, blog, docs page)** | `research-reducer` skill | Thorough | `harnessx/<id>/<auto-slug>/` (skill creates its own folder) |
| **Research paper / quant doc** | Opus agent with high thinking | Very thorough | `exploration/research-<topic>/` |
| **Design file / wireframe** | General-purpose agent | Moderate | `exploration/design-<name>/` |
| **Spec / PRD** | General-purpose agent | Thorough | `exploration/spec-<name>/` |
| **Reference repo** (not the project's own code) | Architecture-focused agent | Thorough — focus on patterns to adopt | `exploration/reference-<repo-name>/` |
| **Data files / schemas** | General-purpose agent | Moderate | `exploration/data-<name>/` |
| **Legacy code for refactor** | `rust:exploration-and-planning` or equivalent | Extremely thorough — map all public interfaces | `exploration/legacy-<name>/` |

### Determining Number of Agents

The goal is maximum parallelism without overwhelming quality.

- **Small codebase** (<50 files, <5k LOC): 1 agent
- **Medium codebase** (50-200 files, 5k-20k LOC): 2-3 agents, split by module/domain
- **Large codebase** (200+ files, 20k+ LOC): 3-5 agents, each assigned a module boundary or domain area
- **Multiple independent resources**: 1 agent per resource, all in parallel
- **Single research document**: 1 agent (Opus with extended thinking)
- **Multiple related documents**: 1-2 agents depending on cross-referencing needs

When splitting a codebase across agents, give each agent a clear boundary: "You explore `src/trading/` and `src/orderbook/`" — not "explore the trading stuff." Include file paths.

---

## Dispatching Agents

Create the exploration output directory first:

```bash
mkdir -p harnessx/<project-id>/intake/exploration
```

### Agent Dispatch Template

For each agent, provide a prompt that includes:

1. **Project context** — the goal, scope, and what matters (summarized from intake docs)
2. **Resource location** — exact file paths or URLs
3. **Exploration focus** — what specifically to look for, tied to the project goal
4. **Output location** — the exact path to write notes
5. **Output format** — the note-taking template (see below)

### For Code Exploration Agents

Use the Agent tool with appropriate specialist skills. For Rust codebases, assign `rust:exploration-and-planning`. For other languages, use a general-purpose agent with explicit architecture instructions.

**Prompt structure for code agents:**

```
You are exploring a codebase as part of project intake for: [1-2 sentence project goal]

## Your Assignment

Explore [resource path/URL] with focus on: [specific areas relevant to the project]

## What to Document

Write your findings to: [output path]

Be EXTREMELY thorough. Future agents will rely on your notes to write code — if you miss something, they'll reinvent it or build on wrong assumptions.

### Required Sections in Your Notes

1. **Repository Overview**
   - Purpose, language, framework, build system
   - Directory structure with brief description of each top-level module
   - Entry points (main, lib, CLI)

2. **Architecture Map**
   - How modules relate to each other
   - Data flow through the system
   - Key abstractions (traits, interfaces, base classes)
   - Dependency graph between internal modules

3. **Key Types and Data Structures**
   - List every important struct/class/type with its fields
   - Include the file path and line number for each
   - Note which ones are public API vs internal

4. **Working Code Examples**
   - For each important pattern, include an ACTUAL code snippet from the repo (not invented)
   - Show how key functions are called, with real arguments
   - Include the file:line reference for every snippet

5. **Logic That Matters for [Project Goal]**
   - Identify code directly relevant to what we're building
   - Explain the algorithms, state machines, or business rules
   - Include code snippets with file:line references
   - Note any gotchas, edge cases, or assumptions baked into the logic

6. **Reuse Inventory**
   - What can be directly reused?
   - What needs modification?
   - What patterns should we follow?
   - What anti-patterns should we avoid?

7. **External Dependencies**
   - Key crates/packages and what they're used for
   - Version constraints that matter
   - Any vendored or patched dependencies

8. **Testing Patterns**
   - How are tests structured?
   - What's well-tested vs untested?
   - Test utilities or fixtures available for reuse

9. **Gaps and Risks**
   - What's missing that we'll need?
   - What looks fragile or poorly understood?
   - What assumptions might not hold for our use case?
```

### For URL Resources

When a resource is a URL (article, blog post, documentation page, or any web-accessible content), use the `research:reducer` skill. It fetches the URL, analyzes the content, and produces structured deliverables — `executive-summary.md`, `key-takeaways.md`, `key-insights.md`, `important-details.md`, and `math-notation.md` (when math is present) — inside `harnessx/<project-id>/<auto-generated-slug>/`.

Invoke it via the Skill tool (`skill: "research-reducer"`) or dispatch it to a subagent with the skill loaded. The slug is auto-generated from the content, so you don't need to create the output folder — the skill handles it.

After it completes, read its outputs and incorporate the findings into the exploration summary and action items like any other resource.

### For Research / Document Agents

Use an Opus agent with extended thinking for research papers, quant documents, or complex specs. For URL-based research (as opposed to local files), prefer the `research-reducer` skill above — it produces a structured breakdown that's easier for downstream agents to consume.

**Prompt structure for research agents:**

```
You are analyzing a research document as part of project intake for: [1-2 sentence project goal]

## Your Assignment

Deeply analyze [document path/URL] and extract everything relevant to the project.

Write your findings to: [output path]

### Required Sections

1. **Document Summary**
   - What is this document about?
   - Who wrote it and when?
   - Key thesis or purpose

2. **Key Findings Relevant to Project**
   - Extract specific findings, formulas, recommendations, or data points
   - For each, explain WHY it matters for our project goal
   - Include exact quotes or references where precision matters

3. **Technical Details**
   - Algorithms, formulas, or methodologies described
   - Parameters, thresholds, or configuration values mentioned
   - Data requirements or assumptions stated

4. **Recommendations for Implementation**
   - What does this document suggest we should build?
   - What constraints does it impose?
   - What trade-offs does it highlight?

5. **Open Questions**
   - What's ambiguous or unclear?
   - What assumptions need verification?
   - What's missing that we need to figure out?
```

### Launching Agents

Launch ALL agents concurrently using the Agent tool. Each agent runs independently and writes to its own subfolder.

For code exploration, prefer `subagent_type: "Explore"` for thorough codebase analysis, or use a general-purpose agent with explicit skill assignment for Rust code.

For research documents, use `model: "opus"` to get extended thinking capabilities.

**Example dispatch pattern:**

```
# Agent 1: Explore main codebase
Agent(
  description: "Explore trading engine code",
  prompt: [full prompt from template above],
  subagent_type: "Explore" or "general-purpose"
)

# Agent 2: Explore reference repo
Agent(
  description: "Explore reference patterns",
  prompt: [full prompt from template above],
  subagent_type: "Explore"
)

# Agent 3: Analyze quant research doc
Agent(
  description: "Analyze research document",
  prompt: [full prompt from template above],
  model: "opus"
)
```

---

## After Agents Return

Once all exploration agents have completed:

### 1. Review All Notes

Read every file in `harnessx/<project-id>/intake/exploration/`. Assess:
- Are the notes thorough enough for agents to work from?
- Are there gaps that need follow-up exploration?
- Do the findings change our understanding of scope or feasibility?

### 2. Write the Exploration Summary

Create `harnessx/<project-id>/intake/exploration/summary.md`:

```markdown
# Exploration Summary

**Date:** YYYY-MM-DD
**Project:** <project-id>

## Resources Explored

| Resource | Type | Agent | Notes Location | Key Findings |
|----------|------|-------|----------------|--------------|
| ... | ... | ... | ... | 1-2 sentence summary |

## Cross-Cutting Themes

[Patterns, risks, or opportunities that emerged across multiple resources]

## Impact on Project Understanding

[How exploration changed or refined our understanding of:
- Feasibility
- Scope
- Technical approach
- Risk areas]

## Reuse Map

[Consolidated list of what we can reuse from explored resources]

## Gaps Identified

[What we still don't know or need to investigate further]

## Action Items Created

[List of action items created from exploration, with IDs]
```

### 3. Create Action Items

Use the `hx:intake-actions-writing` protocol to create action items from exploration findings. Read the `/hx:intake-actions-writing` skill instructions for the full field guide.

Key rules for exploration-origin actions:

- **origin**: `intake:exploration`
- **category**: typically `research`, `exploration`, `verification`, `implementation`, or `integration`
- **detail**: must be self-contained — include file paths, code references, and WHY this action matters
- **input_docs**: point to the exploration notes file(s) that led to this action
- **note-author**: `hx-intake-completion-exploration`
- **mode**: `plan` (almost always — force agents to think before coding)

**What generates action items from exploration:**

1. **Reuse opportunities** — "Adapt OrderBook struct from enginesol for our use case" (category: implementation)
2. **Gaps discovered** — "No websocket reconnection logic exists; need to design from scratch" (category: exploration)
3. **Risks identified** — "API rate limit is 100/min, need to verify this won't bottleneck our data pipeline" (category: verification)
4. **Pattern decisions** — "Two competing patterns found for state management; need architecture decision" (category: research)
5. **Integration points** — "Auth token refresh is handled in utils/auth.rs:45 — our service needs to integrate with this" (category: integration)
6. **Follow-up exploration** — "Module X is complex and underdocumented; needs deeper analysis before planning" (category: exploration)

### 4. Tag Everything — Bidirectional Traceability

This is critical. Follow the `hx:tag-context-writing` protocol.

**Direction 1: Tag exploration notes with action references**

After creating action items, go back to the exploration notes and add `#action-N` tags to the specific paragraphs that generated each action.

Example in `exploration/code-trading-engine/notes.md`:
```markdown
The OrderBook uses a BTreeMap for price levels, which gives O(log n) insertion but
the matching engine iterates all levels on every fill. This could be a bottleneck
at high message rates. #action-12
```

**Direction 2: Action items reference exploration notes**

When creating action items, include the exploration notes file path in `input_docs`:
```bash
harnessx intake-actions create \
  --title "Evaluate OrderBook matching performance under load" \
  --category "verification" \
  --origin "intake:exploration" \
  --detail "BTreeMap-based OrderBook in code-trading-engine/src/orderbook.rs:87 iterates all price levels per fill. At projected 10k msg/sec this may bottleneck. Need benchmarks before committing to this design." \
  --input-docs "harnessx/<project-id>/intake/exploration/code-trading-engine/notes.md" \
  --complexity "medium" \
  --mode "plan" \
  --note-author "hx-intake-completion-exploration" \
  --note-text "Discovered during codebase exploration. The current implementation works for low-throughput but our success criteria require 10k msg/sec."
```

**Direction 3: Cross-reference with existing intake markdowns**

If exploration findings relate to something discussed in earlier intake sections, add `#action-N` tags to those markdowns too. For example, if the scope document mentions "must handle 10k messages/sec" and exploration found a bottleneck, tag that scope paragraph with the action reference.

### 5. Verify Tags

For every `#action-N` tag placed, verify it's findable:
```bash
harnessx context search-context --query "#action-N"
```

The result must return a meaningful paragraph, not just the tag. If it doesn't, the tag is in the wrong place — move it to the line with the most context.

---

## Completion

After all exploration is done, notes are written, actions are created, and tags are placed:

```bash
harnessx intake-completion complete exploration
```

This marks the exploration section as completed in `intake_completion.json`. When all three intake-completion sections (exploration, ideation, project_risk_manager) are done, the CLI automatically marks the `intake_completion` pipeline stage as complete.

---

## Edge Cases

**No resources were collected during intake:**
- This shouldn't happen (the resources skill creates action items for every resource)
- If it does, check `resources.md` and `intake_actions.json` for any references
- If truly no resources, write a brief summary noting this and create an action item flagging the gap

**Resource is unreachable (URL down, repo access denied):**
- Don't silently skip it — create an action item noting the blocker
- Tag it with complexity `uncertain` and category `verification`
- Continue with resources that ARE accessible

**Exploration reveals scope is wrong:**
- Don't change scope yourself
- Create action items noting what was discovered and why scope may need revision
- The planning stage will handle scope adjustments

**Codebase is enormous (100k+ LOC):**
- Prioritize modules most relevant to the project goal
- Split across 4-5 agents by domain boundary
- Each agent should note what they did NOT explore and why
- Create follow-up exploration actions for deprioritized areas

**Resource is a mix of types (e.g., repo with embedded docs):**
- Send one agent to explore the code, another to analyze the docs
- Have them write to separate note files within the same subfolder
- Cross-reference in the summary
