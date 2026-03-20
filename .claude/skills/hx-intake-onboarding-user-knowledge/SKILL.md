---
name: hx:intake-onboarding-user-knowledge
description: Extract the user's background, expertise, and domain insights to inform project execution. Use this skill when the intake process reaches the "user_knowledge" section, when you need to understand what the user brings to the table professionally, or when their expertise should shape how the project is approached. Also trigger when the user says "here's my background", "I know a lot about", "in my experience", or when you need to capture domain-specific recommendations, technical preferences, or professional insights that should influence project decisions.
---

# Intake User Knowledge

You help surface what the user already knows — their professional background, domain expertise, technical preferences, and hard-won insights — so that everything downstream (planning, execution, review) is informed by their actual experience rather than generic assumptions.

This matters because users are experts in their own domains. A systems engineer thinks about a project differently than a data scientist. A researcher who's spent years in a field has instincts about what will and won't work that no amount of automated analysis can replace. Your job is to draw that knowledge out and capture it in a form that future agents can act on.

---

## Why this section exists

Most project tools treat the user as a requirements source — someone who says what they want and then waits for delivery. That's backwards. Users often have deep knowledge about:

- **The domain** — industry patterns, regulatory constraints, what competitors do, what users actually care about
- **The technology** — which tools they trust, which ones have burned them, architectural patterns they've seen succeed or fail
- **The process** — how they like to work, what kind of feedback loops they need, how they think about quality
- **The people** — who the stakeholders are, what politics exist, who needs to be convinced

If this knowledge stays in the user's head, agents will make worse decisions. The goal is to get it out, documented, and actionable.

---

## How to run the conversation

### Start by understanding who they are

Open with something genuine — not a form to fill out, but a real question:

> "Before we get deeper into planning, I'd like to understand your background and what you bring to this project. What's your area of expertise, and how long have you been working in it?"

Listen carefully. The first answer usually reveals the surface — job title, years of experience, primary skill. The interesting stuff is one or two questions deeper.

### Follow the expertise, not a checklist

Once you know their domain, follow threads that are relevant to the project. The questions you ask should be shaped by both their background and the confirmed goal/scope.

**For a technical user** (engineer, developer, architect):
- "You've worked with [technology]. Are there patterns or approaches from that experience you'd want applied here?"
- "Have you built something similar before? What would you do differently this time?"
- "Are there tools, libraries, or frameworks you have strong opinions about — things you'd want us to use or avoid?"
- "What's the most common way projects like this go wrong in your experience?"

**For a domain expert** (researcher, analyst, industry specialist):
- "What does someone outside your field usually get wrong about [domain]?"
- "Are there constraints or regulations in this space that might not be obvious?"
- "If you were advising someone new to this area, what would you tell them to watch out for?"
- "What data or resources do you already have access to that could accelerate this?"

**For a product/business user** (PM, founder, business analyst):
- "Who are the stakeholders, and what does each of them care about most?"
- "Have you tried to solve this problem before? What happened?"
- "What's the political landscape — are there people who need to be on board, or who might resist?"
- "What does your gut say is the riskiest part of this project?"

**For someone wearing multiple hats** (common in smaller teams or solo projects):
- "Of all the things you do, which ones are you most confident in for this project?"
- "Where do you feel like you're stretching into unfamiliar territory?"
- "What would be most valuable for the agents to handle so you can focus on what you're best at?"

These are starting points, not scripts. Let the conversation flow naturally and follow what's interesting and relevant.

### Take their expertise seriously

This is the core principle of this skill. When a user says "in my experience, X usually fails because Y," that's gold. Don't gloss over it. Probe it:

- "That's a really useful insight. Can you give me an example of when that happened?"
- "How would you recommend we account for that in this project?"
- "Should that inform how we structure the work, or is it more of a thing to watch for?"

Users who feel heard give you better information. Users who feel like they're filling out a form give you surface-level answers.

### Surface tacit knowledge

The most valuable knowledge is often stuff the user doesn't think to mention because it's so obvious to them. Techniques for surfacing it:

- **Contrast questions**: "How would someone with your background approach this differently than a generalist?"
- **Failure questions**: "What's the mistake a junior person in your field would make on this?"
- **Process questions**: "Walk me through how you'd actually do [specific task] — step by step."
- **Tool questions**: "What tools or resources do you reach for first when working on something like this?"

### Capture preferences and working style

Beyond domain knowledge, capture how the user prefers to work:

- How do they like to receive updates? (detailed vs. high-level, frequent vs. milestone-based)
- Do they want to be involved in implementation decisions, or just review outcomes?
- How do they think about quality vs. speed tradeoffs?
- Are there communication styles or patterns that work well or poorly for them?

These preferences shape how agents should interact with them throughout the project.

---

## What to capture

As insights emerge, create action items using `harnessx intake-actions create` with:

- **Origin**: `intake:user_knowledge`
- **Category**: Use categories that reflect the type of knowledge:
  - `domain-insight` — industry/field-specific knowledge
  - `technical-preference` — tool, framework, or architecture preferences
  - `process-preference` — how they like to work
  - `risk-insight` — things they've seen go wrong
  - `resource` — existing assets, data, or access they bring
  - `stakeholder` — people and politics

Include enough detail that an agent reading the action item months from now would understand the insight and why it matters. "User prefers React" is thin. "User has 8 years of React experience and has strong opinions about state management — prefers Zustand over Redux for projects of this scale because of boilerplate reduction; has been burned by prop drilling in past projects" is actionable.

---

## Knowing when you're done

You're done when you have a solid picture of:

1. **Who the user is** professionally — their background and primary expertise
2. **What they know** that's relevant to this project — domain insights, technical preferences, past experience
3. **How they work** — communication preferences, involvement level, quality expectations
4. **What they bring** — existing resources, relationships, or institutional knowledge

Not every project needs deep coverage of all four. A solo developer building a side project might just need #1 and #2. A complex enterprise project with multiple stakeholders needs all four explored thoroughly.

When you've covered the relevant ground, give a brief summary of the key insights captured, then mark the section complete.
