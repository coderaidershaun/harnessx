---
name: hx:user-troubleshooting
description: Investigate why a harnessx project pipeline is blocked, diagnose failures from integration test results, and work with the user to resolve them. Use this skill when the pipeline hits `user_input_required`, when the user asks "why did my project fail", "what went wrong", "what's blocking my project", "troubleshoot", "debug the pipeline", or when an agent needs to understand and resolve a project failure before work can continue.
---

# HX User Troubleshooting

You are helping the user understand why their project pipeline is blocked and what they need to provide or fix before work can continue.

## Context

The harnessx pipeline tracks progress through ordered stages. When something goes wrong — an integration test fails, a required input is missing, or an agent encounters an unresolvable issue — the pipeline sets the `user_input_required` stage to a non-completed status. The failure details are written to `harnessx/<project-id>/integration-tests/failing.md`.

Your job is to read that file, understand the failure, and walk the user through resolving it.

## Step 1: Gather context

Run these commands to understand the current state:

```bash
harnessx project active
```

Extract the project `id` from the response. Then:

```bash
git log --oneline -2
```

This shows the user what recent work led to the current state.

## Step 2: Read the failure report

Read the failing integration tests file:

```
harnessx/<project-id>/integration-tests/failing.md
```

This file documents what failed and why. It may contain:
- The specific integration test(s) that failed
- Error messages or stack traces
- What input or action is needed from the user
- Links to relevant code, APIs, or resources

If the file doesn't exist, check `harnessx/<project-id>/` for any other diagnostic files that might explain the blockage.

## Step 3: Present the diagnosis

Summarise clearly for the user:
1. **What happened** — the recent commits and what stage the pipeline reached before stopping.
2. **What failed** — the specific failure(s) from `failing.md`, presented in plain language.
3. **What's needed** — the concrete action(s) the user must take to unblock the pipeline. Be specific: if it's a missing API key, say so. If it's a logic error that needs a design decision, frame the options.

## Step 4: Work with the user to resolve

Once the user responds with the required input or decision:

1. Apply the fix or capture the input as needed.
2. If the fix involves code changes, use the appropriate skills:
   - `rust-ergonomic-refactoring` for Rust code quality
   - `rust-unit-testing` or `rust-integration-testing` to verify the fix
3. Re-run any failing tests if possible to confirm resolution.
4. Once resolved, mark the stage complete:

```bash
harnessx progress complete user_input_required
```

This unblocks the pipeline so the next stage's agent can take over.

## Behavior

- Be direct about what failed and why — don't sugarcoat or add unnecessary preamble.
- If multiple issues are in `failing.md`, prioritize them and tackle one at a time.
- If you can fix something without user input (e.g., a typo, a missing import), just fix it and tell the user what you did.
- Only ask the user for input when a genuine decision or external information is required.
- Do not advance past `user_input_required` until the underlying issue is actually resolved.
