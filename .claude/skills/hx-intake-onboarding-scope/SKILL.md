---
name: hx:intake-onboarding-scope
description: Define clear, thorough project scope boundaries from a confirmed goal. Use this skill when the intake process reaches the "scope" section, when the user needs help defining what's in and out of scope, or when project boundaries feel vague or unbounded. Also trigger when the user says "define scope", "what should be in scope", "help me scope this", "what are the boundaries", or asks about MVP vs full version, feature prioritization for scope, or what to cut or defer.
---

# Intake Scope

You help users turn a confirmed goal into a well-defined scope — the concrete boundaries that tell everyone (humans and agents) what this project will and won't do.

Scope is where projects either get focused or fall apart. A goal says *what* and *why*. Scope says *how much* and *where the lines are*. Without clear scope, a project to "build a dashboard" could mean a weekend prototype or a six-month platform. Your job is to make that distinction explicit.

---

## Why scope is hard

Most people struggle with scope for predictable reasons:

1. **They don't know what they don't know.** A user building their first API might not realize they need to think about auth, rate limiting, or versioning. They're not being sloppy — they just haven't hit those decisions yet.

2. **Everything feels important.** When you're excited about a project, cutting features feels like compromising. Help them see that deferring isn't cutting — it's sequencing.

3. **Scope feels abstract.** "Define your scope" is vague advice. People need concrete categories to think through. That's what you provide.

---

## Before you start

You need context from the goal section. Run:

```bash
harnessx project active
```

Read the project's description and takeaway line — these are the confirmed goal. Your scope conversation builds directly on top of this. If the goal feels unclear, ask the user to clarify rather than guessing.

---

## The scope dimensions

Scope isn't one thing — it's a set of boundaries across several dimensions. Walk through each one with the user. You don't need to hit every dimension for every project (a personal script doesn't need a user scope discussion), but consider each and skip only the ones that genuinely don't apply.

### 1. Feature scope — What will it do?

This is what people usually think of as "scope." List the concrete capabilities the project will deliver.

**How to draw it out:**

- Start from the goal: "Your goal mentions [X]. What are the specific things a user needs to be able to do for that goal to be met?"
- For each capability, probe one level deeper: "When you say 'manage users', do you mean create/edit/delete? Roles and permissions? Self-service signup?"
- Actively suggest capabilities they might not have considered: "Projects like this usually need [X] — is that something you're thinking about, or is that out of scope?"

**The in/out/later framework:**

For each capability or feature area, classify it:
- **In scope** — Will be delivered as part of this project
- **Out of scope** — Explicitly not part of this project (and won't be)
- **Deferred** — Valuable but not for this round; will be addressed later

The "deferred" category is important. It gives users a place to put things they care about without bloating the current scope. It also creates a natural backlog for future work.

### 2. User scope — Who is this for?

Different users have different needs, and trying to serve everyone at once is a scope trap.

- Who are the primary users? (the ones the project is *for*)
- Are there secondary users? (people who benefit but aren't the focus)
- Who is explicitly *not* a target user?

For a dashboard project, the primary users might be the ops team, secondary users might be engineering leads, and explicitly out of scope might be external customers.

### 3. Technical scope — What's the technical boundary?

This covers the technical decisions that constrain the project:

- **Platform/environment**: Web? Mobile? CLI? Desktop? Which browsers/OS versions?
- **Integrations**: What external systems does this touch? What APIs does it consume or expose?
- **Data**: What data sources? What volume? Any migration needed?
- **Infrastructure**: Where does it run? What's the deployment target?
- **Performance**: Any hard requirements? (response time, throughput, concurrent users)

Don't turn this into a full architecture session — that comes later. You're drawing the boundary, not designing the system. The goal is to know what technical territory is in play.

### 4. Quality scope — How good does it need to be?

This is the dimension people most often leave implicit, and it causes the most grief later.

- **Error handling**: Happy path only? Graceful degradation? Full resilience?
- **Testing**: What level of test coverage? Unit? Integration? E2E?
- **Documentation**: User docs? API docs? None for now?
- **Accessibility**: A11y compliance level? Not applicable?
- **Security**: Auth requirements? Data encryption? Compliance standards?

Match the quality scope to the project's stakes. A prototype for a hackathon has different quality needs than a financial services API.

### 5. Timeline scope — What are the time boundaries?

If there are time constraints, they shape everything else:

- Is there a deadline? Hard or soft?
- Are there milestones or phases?
- What's the relationship between timeline and feature scope? (i.e., if time runs short, what gets cut?)

---

## How to run the conversation

### Start with what they know

Don't march through the dimensions like a checklist. Start by asking the user what they already have in mind for scope:

> "You've got a clear goal. Now let's figure out the boundaries — what's in, what's out, and what can wait. What's your initial sense of scope?"

Listen to their answer and identify which dimensions they've naturally covered and which have gaps.

### Fill the gaps proactively

This is where you earn your keep. Based on the goal and what the user has said, *suggest* scope items they haven't mentioned. This is the core value of the skill — you've seen enough projects to know what gets forgotten.

Frame suggestions as questions, not mandates:
- "Most projects like this end up needing [X]. Have you thought about whether that's in scope?"
- "One thing that often catches people off guard is [Y]. Worth deciding now whether that's in or out."
- "I notice you haven't mentioned [Z] — is that intentionally out of scope, or just something you haven't gotten to yet?"

### Push for concrete boundaries

Vague scope is almost worse than no scope. Push gently for specifics:

- Instead of "support multiple users" → "support up to 10 concurrent users in the MVP"
- Instead of "good performance" → "dashboard loads in under 2 seconds on a standard connection"
- Instead of "some testing" → "unit tests for business logic, manual testing for UI"

### Know when to stop

Not every project needs exhaustive scope definition. Match your depth to the project:

- **Weekend project**: Feature scope and maybe technical scope. That's probably enough.
- **Multi-week project**: All five dimensions, but keep it pragmatic.
- **Large/team project**: All five dimensions with real rigor, especially user scope and quality scope.

If the user is getting fatigued, wrap up what you have. Incomplete-but-documented scope is better than nothing.

---

## Converging on the scope statement

Once you've covered the relevant dimensions, synthesize everything into a structured scope summary. Present it to the user for confirmation.

**Format:**

```
## Scope Summary

### In Scope
- [Concrete capability or boundary]
- [Another one]

### Out of Scope
- [Explicitly excluded item]
- [Another one]

### Deferred
- [Item for future consideration]
- [Another one]

### Key Constraints
- [Timeline, platform, performance, or quality constraint]
- [Another one]
```

Ask: "Does this capture the boundaries? Anything to add, move, or remove?"

Iterate until the user confirms.

---

## After scope is confirmed

The scope conversation generates rich context that should be captured as intake actions. As scope items emerge during the conversation, create action items using `harnessx intake-actions create` with:

- **Origin**: `intake:scope`
- **Category**: Match the scope dimension (e.g., `feature`, `technical`, `infrastructure`, `quality`)
- Detailed descriptions that include the *why* — future agents won't have this conversation context

The scope summary itself doesn't get written to a project metadata field (unlike the goal, which becomes the description). Instead, it lives in the action items and will be shaped into milestones and epics during planning.

---

## Document the Discussion

Before marking this section complete, write a comprehensive markdown file that captures the full substance of the scope discussion. Get the active project ID:

```bash
harnessx project active
```

Then save the document to `harnessx/<project-id>/intake/scope.md`.

The document should include:

- **Date** of the discussion
- **The user's initial sense of scope** — what they said when first asked about boundaries
- **Questions asked and responses** — paraphrased, covering each scope dimension explored (feature, user, technical, quality, timeline)
- **The confirmed scope summary** — the full In Scope / Out of Scope / Deferred / Key Constraints breakdown the user approved
- **Key decisions and reasoning** — why specific items were placed in scope vs. deferred vs. excluded, any trade-offs discussed
- **Proactive suggestions made** — scope items you raised that the user hadn't considered, and their response
- **Areas of uncertainty** — scope questions that remain open or need further exploration
- **Action items created** during this section (titles and brief descriptions)

Write this as a readable narrative document, not a raw chat log. The goal is that any agent or person reading this file later gets the full picture of what was discussed and decided, without needing access to the original conversation.
