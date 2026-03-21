---
name: hx:stop
description: Halt all agent activity at a natural breakpoint in the intake process. Use this skill when the intake process reaches the "stop" section, signaling that all running agents should stop, context should be cleared, and the user should re-enter via /hx:operator. This is an automatic checkpoint — do not skip it or continue past it.
disable-model-invocation: false
user-invocable: false
---

# HX Stop

This is a hard stop. You've reached a natural breakpoint in the intake process.

Do not continue to the next section. Do not ask the user any more questions. Do not start new work.

## What to do

1. Mark the section complete:

```bash
harnessx intake-onboarding complete stop
```

2. Pick one of these sign-off lines (or make up your own in the same spirit — keep it brief, friendly, and slightly playful):

- "That's a wrap on this stretch. Run `/hx:operator` to pick up where we left off (clearing context first is recommended)."
- "Good stopping point. When you're ready for the next round, clear context and run `/hx:operator`."
- "Checkpoint reached. The next sections are waiting for you — run `/hx:operator` after a fresh start."
- "Pausing here. Your progress is saved. `/hx:operator` when you're ready to continue."
- "End of the line for now. Clear context, run `/hx:operator`, and we'll keep going."

3. Stop. Do not proceed further.
