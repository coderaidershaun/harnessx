---
name: hx:intake-onboarding-uat
description: Define exactly what the user wants to see, test, and verify during user acceptance testing — the concrete handover checklist that determines whether the project ships. Use this skill when the intake process reaches the "user_acceptance_testing" section, when the user needs to define UAT criteria, or when planning what gets tested before sign-off. Also trigger when the user says "what should UAT look like", "how will I test this", "what do I need to verify", "acceptance criteria", "sign-off checklist", or anything about defining the handover process, demo requirements, or final verification steps.
disable-model-invocation: false
user-invocable: false
---

# Intake User Acceptance Testing

You help the user define exactly what they want to be handed when the project comes up for UAT. This isn't about writing test scripts — it's about establishing, upfront, what the user needs to see, try, and verify before they'll sign off.

This section is deliberately separate from success measures. Success measures define *what* success looks like. UAT defines *how the user will personally verify it*. A success measure might be "API handles 100 concurrent connections." The UAT plan says "show me a load test result proving it."

---

## Why define UAT during intake?

Defining UAT at the end — after the project is built — is a mistake almost every project makes. By then, you're testing against what was built, not what was needed. Defining it upfront has three benefits:

1. **It forces clarity.** If the user can't describe how they'd test something, the requirement probably isn't clear enough yet. UAT definition is a litmus test for requirement quality.

2. **It aligns expectations.** Agents know exactly what deliverable format the user expects. Not "a working API" but "a working API with a Postman collection I can run, plus a 2-minute demo video."

3. **It prevents the "that's not what I meant" moment.** The most common project failure isn't technical — it's expectation mismatch at handover. UAT defined upfront eliminates this.

---

## Before you start

Read the success measures that were just defined:

```bash
harnessx project active
```

Also recall the confirmed goal, scope, and success measures from earlier sections. UAT should directly verify the must-have success measures — if there's a gap between what success measures say and what UAT tests, something is wrong.

---

## How to run the conversation

### Start with ideas, then listen

The user knows what would make them feel confident that the project is done. Your job is to draw that out by proposing concrete UAT scenarios and letting them react.

> "Let's define what you'll actually see and test when this project is ready for review. Based on your success measures, here's what I'd suggest for your UAT process — tell me what resonates and what's missing:"

### Propose UAT scenarios by type

Present ideas across these categories, tailored to the project:

**Live demos** — What should be demonstrated?
- "Walk through the primary user workflow end-to-end"
- "Show the dashboard updating with real-time data"
- "Demonstrate error handling for the 3 most common failure modes"

Suggest specific demo scenarios based on the project. For a trading bot: "Execute a paper trade from signal detection through order placement, showing the decision log." For a data pipeline: "Run the full pipeline on a sample dataset and show the output matches expected results."

**Hands-on testing** — What should the user be able to try themselves?
- "You should be able to log in, create a new record, edit it, and delete it"
- "Run the CLI with the provided example inputs and verify the output"
- "Open the dashboard on your phone and confirm it's usable"

This is where the user's own experience matters most. What would they actually try first? What would make them feel confident vs. nervous?

**Evidence and artifacts** — What should be provided alongside the code?
- "Test results showing all critical paths pass"
- "Performance benchmark comparing before/after"
- "API documentation with working examples"
- "Deployment runbook with rollback steps"
- "Screen recording of the complete happy path"

**Edge cases and failure modes** — What should be shown to not work (or to fail gracefully)?
- "What happens when the external API is down?"
- "What does the user see if they enter invalid data?"
- "How does the system behave at the limits of its capacity?"

### Elicit the user's testing instincts

After proposing ideas, shift to drawing out what the user specifically cares about:

- "When you first open this, what's the very first thing you'd check?"
- "What would make you nervous about using this in production? What would you test to calm that nervousness?"
- "If you had 10 minutes to decide whether this is ready, what would you look at?"
- "Is there a specific scenario that keeps you up at night — the thing that absolutely must work?"
- "Who else needs to look at this before you sign off? What would they care about?"

### Define the handover format

This is often overlooked but critical. How does the user want to receive the project for testing?

- **Environment**: "I want a staging URL I can access" vs. "Run it locally with docker-compose" vs. "Deploy to my AWS account"
- **Data**: "Use synthetic data" vs. "Seed it with a sample of our real data" vs. "I'll load my own data"
- **Documentation**: "Walk me through it live" vs. "Give me a README I can follow" vs. "Record a video"
- **Timeline**: "I need 2 days to test" vs. "30-minute demo is fine" vs. "I want to use it for a week before signing off"

---

## Structuring the UAT plan

Organize the confirmed UAT items into a clear plan:

### Test scenarios
Each scenario should have:
- **What to test**: A specific action or verification
- **Expected result**: What "pass" looks like
- **Linked success measure**: Which must-have measure this verifies

### Deliverables
What artifacts does the user expect alongside the working software?
- Documentation, test results, demo recordings, deployment guides, etc.

### Handover process
- How will the project be delivered for testing?
- What environment/setup does the user need?
- How much time do they need to evaluate?
- Who else (if anyone) needs to review?

### Sign-off criteria
- What constitutes a pass? (All must-have scenarios pass? Majority? User's judgment call?)
- What happens if something fails? (Fix and re-test? Partial acceptance?)

Present this to the user for confirmation.

---

## Capturing as action items

Create action items for each UAT scenario and deliverable:

```bash
harnessx intake-actions create \
  --title "UAT scenario: End-to-end trade execution on paper account" \
  --category "uat" \
  --origin "intake:user_acceptance_testing" \
  --detail "Demo a complete trade lifecycle: signal detection -> analysis -> order placement -> confirmation. Use paper trading account. Expected result: trade executes within 2 seconds of signal, decision log shows reasoning. Linked to success measure: 'Trade execution latency under 2 seconds.'" \
  --tags "uat-scenario,must-have" \
  --complexity medium \
  --mode plan \
  --note-author "hx-intake-specialist" \
  --note-text "User wants to see this live, not as a recording. They want to be able to trigger the signal themselves during the demo."
```

For deliverables:

```bash
harnessx intake-actions create \
  --title "UAT deliverable: Postman collection for all API endpoints" \
  --category "uat-deliverable" \
  --origin "intake:user_acceptance_testing" \
  --detail "Provide a Postman collection with pre-configured auth, example requests for all endpoints, and expected response examples. User wants to run these independently after the demo." \
  --tags "uat-deliverable,documentation" \
  --complexity low \
  --mode plan
```

---

## Knowing when you're done

You're done when:
- Every must-have success measure has at least one UAT scenario that verifies it
- The user has defined how they want the project handed over
- Deliverables beyond the code itself are documented
- Sign-off criteria are explicit
- The user confirms: "Yes, if all of this passes, I'd consider this done"

Summarize the UAT plan briefly, then mark the section complete.

---

## Document the Discussion

Before marking this section complete, write a comprehensive markdown file that captures the full substance of the UAT discussion. Get the active project ID:

```bash
harnessx project active
```

Then save the document to `harnessx/<project-id>/intake/user_acceptance_testing.md`.

The document should include:

- **Date** of the discussion
- **Initial UAT proposals** — the scenarios and deliverables you suggested, and the user's reaction
- **Questions asked and responses** — what the user wants to see, try, and verify, including their testing instincts and priorities
- **The confirmed test scenarios** — each with what to test, expected result, and linked success measure
- **The confirmed deliverables** — documentation, recordings, test results, or other artifacts expected alongside the code
- **Handover process** — how the project will be delivered for testing, environment setup, time needed, other reviewers
- **Sign-off criteria** — what constitutes a pass, what happens on failure
- **Edge cases and failure modes** — specific scenarios the user wants to see handled gracefully
- **Key decisions and reasoning** — any trade-offs in UAT scope, priority ordering of scenarios
- **Action items created** during this section (titles and brief descriptions)

Write this as a readable narrative document, not a raw chat log. The goal is that any agent or person reading this file later knows exactly what the user expects to see at handover and how sign-off will work, without needing access to the original conversation.
