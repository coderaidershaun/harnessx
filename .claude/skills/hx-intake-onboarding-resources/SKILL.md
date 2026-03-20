---
name: hx:intake-onboarding-resources
description: Collect links, repos, documents, codebases, and any other resources the user has for the project, then store each as a well-documented action item with planned follow-up work. Use this skill when the intake process reaches the "resources" section, when the user shares links or references, mentions an existing codebase, points to documentation or repos, or says things like "here's the repo", "I have some links", "check out this doc", "the code is at", or "I found this resource". Also trigger when gathering materials that agents will need to research, explore, or extract from later.
---

# Intake Resources

You help collect and document every resource the user brings to the project — links, repos, codebases, docs, APIs, design files, spreadsheets, research papers, anything. Each resource gets stored as a well-structured action item so that downstream agents know what exists, where to find it, and what to do with it.

---

## Why resources need actions, not just bookmarks

A list of links is useless if nobody does anything with them. The real value of this section is pairing each resource with a concrete follow-up action. A GitHub repo isn't just "noted" — it becomes "Send exploratory agents to map the architecture and extract key patterns." A design doc isn't just "filed" — it becomes "Review and extract requirements that affect scope."

This is especially important for:
- **Existing codebases** being refactored or extended — agents will need to explore, understand, and extract from them
- **Reference implementations** — agents should study them for patterns and anti-patterns
- **Documentation and specs** — these need to be read and synthesised into actionable requirements
- **External APIs and services** — these need integration research

---

## Before you start

Check if the user has already mentioned any links or resources during earlier intake sections (goal, scope, user knowledge). If they have, collect those first — don't make the user repeat themselves.

```bash
harnessx project active
```

Read the project context to understand what kinds of resources would be relevant.

---

## How to run the conversation

### Start by asking what they've got

> "Let's gather up the resources for this project. Do you have any links, repos, existing code, docs, or reference materials? This includes anything you've already mentioned earlier — I'll make sure everything gets properly documented with follow-up actions."

### Prompt for common resource types

Users often forget resources they take for granted. Based on the project goal and scope, prompt for types they might not think to mention:

- **Code**: "Is there an existing codebase we'll be working with or building on? Where does it live?"
- **Repos**: "Any GitHub repos — yours or others' — that are relevant? Reference implementations, libraries you plan to use?"
- **Docs**: "Any specs, design docs, PRDs, or wikis that describe what we're building or how things currently work?"
- **APIs**: "Are there external APIs or services this project will interact with? Any docs or Swagger/OpenAPI specs for them?"
- **Design**: "Any Figma files, mockups, wireframes, or design systems?"
- **Data**: "Any datasets, databases, or data sources the project needs access to?"
- **Research**: "Any blog posts, papers, tutorials, or Stack Overflow threads that informed your thinking?"
- **Tools**: "Any internal tools, dashboards, or admin panels that are relevant?"
- **People**: "Any Slack channels, team wikis, or points of contact for specific areas?"

Don't run through all of these as a checklist — pick the ones relevant to the project type and ask naturally.

### For each resource, capture three things

1. **What it is** — a clear title and the URL or path
2. **Why it matters** — how does this resource relate to the project?
3. **What should be done with it** — the follow-up action

The follow-up action is the most important part. Help the user think about what agents should actually do with this resource:

| Resource type | Example follow-up actions |
|---|---|
| Existing codebase | "Send exploratory agents to map the architecture, identify key modules, and document the data flow" |
| Reference repo | "Analyse for patterns we want to adopt; note any anti-patterns to avoid" |
| API docs | "Extract endpoint signatures, auth requirements, and rate limits into a reference doc" |
| Design file | "Extract component inventory and map to technical implementation requirements" |
| Research paper | "Summarise key findings and identify which recommendations apply to our approach" |
| Legacy code for refactor | "Catalogue public interfaces, document current behaviour, identify refactoring entry points" |
| Spec/PRD | "Parse into discrete requirements and cross-reference against current scope" |

### Capture user notes

As the user talks about each resource, they'll often add context that isn't in the resource itself — "this repo has a really weird auth setup, watch out for that" or "the API docs are outdated, you'll need to test the actual endpoints." Capture these as notes on the action item. This context is invaluable for agents who'll work with the resource later.

---

## Creating the action items

For each resource, create an action item using `harnessx intake-actions create`. Structure them well:

```bash
harnessx intake-actions create \
  --title "Explore repo: project-name — map architecture and key patterns" \
  --category "research" \
  --origin "intake:resources" \
  --detail "GitHub repo at https://github.com/user/repo. Contains the existing implementation we're refactoring. Key areas to investigate: auth module (reportedly complex), data pipeline (core of the system), and test coverage (unknown)." \
  --tags "codebase,exploration,architecture" \
  --input-docs "https://github.com/user/repo" \
  --complexity medium \
  --mode plan \
  --note-author "hx-intake-specialist" \
  --note-text "User mentioned the auth module has a non-standard token rotation pattern — agents should document this before proposing changes."
```

### Guidelines for action items

- **Title**: Start with a verb that describes the follow-up action, then name the resource. "Explore repo: trading-engine" is better than "Trading engine repo."
- **Category**: Use categories that reflect the type of work:
  - `research` — for resources that need exploration and analysis
  - `extraction` — for code or data that needs to be pulled out and documented
  - `integration` — for APIs or services that need integration research
  - `reference` — for docs that inform decisions but don't need active work
- **Origin**: Always `intake:resources`
- **Detail**: Include the URL/path, why it matters, and any specific areas to focus on
- **Input-docs**: Put the URL or file path here — this is the machine-readable pointer to the resource
- **Tags**: Cross-cutting concerns (e.g., `security`, `performance`, `legacy`, `third-party`)
- **Notes**: User context that won't be obvious from the resource itself

### Multiple actions per resource

Sometimes a single resource warrants multiple action items. An existing codebase might need:
1. An exploration action (map the architecture)
2. An extraction action (document the public API surface)
3. A quality assessment action (evaluate test coverage and code quality)

Use your judgment — don't over-split simple resources, but don't under-split complex ones either.

---

## Knowing when you're done

You're done when:
- All resources the user has (or has previously mentioned) are captured as action items
- Each action item has a clear follow-up action, not just a bookmark
- User notes and context are attached where relevant
- The user confirms there's nothing else they want to add

Give a brief summary: "I've captured X resources with follow-up actions. The main ones are [quick list]." Then mark the section complete.

---

## Document the Discussion

Before marking this section complete, write a comprehensive markdown file that captures the full substance of the resources discussion. Get the active project ID:

```bash
harnessx project active
```

Then save the document to `harnessx/<project-id>/intake/resources.md`.

The document should include:

- **Date** of the discussion
- **All resources collected** — for each resource: the URL/path, what it is, why it matters to the project, and the planned follow-up action
- **User notes and context** — any caveats, warnings, or insider knowledge the user shared about specific resources (e.g., "the API docs are outdated", "the auth module is non-standard")
- **Resources from earlier sections** — any links or materials previously mentioned during goal, scope, or user knowledge that were captured here
- **Resource gaps identified** — types of resources that would be useful but the user doesn't have yet
- **Key decisions and reasoning** — how resources should be prioritized for follow-up, any sequencing decisions
- **Action items created** during this section (titles and brief descriptions, with the follow-up action for each)

Write this as a readable narrative document, not a raw chat log. The goal is that any agent or person reading this file later gets the complete inventory of project resources and knows exactly what to do with each one, without needing access to the original conversation.
