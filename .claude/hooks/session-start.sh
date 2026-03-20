#!/usr/bin/env bash
set -euo pipefail

HARNESS_DIR="$CLAUDE_PROJECT_DIR/.claude/harness"
PROJECT_JSON="$HARNESS_DIR/project.json"

# Output project.json so the agent knows the active project
if [ ! -f "$PROJECT_JSON" ]; then
  echo "No active project (project.json not found)"
  exit 0
fi

jq '.' "$PROJECT_JSON"

# Resolve project ID and run its init.sh if it exists
PROJECT_ID=$(jq -r '.project_id // empty' "$PROJECT_JSON")
if [ -z "$PROJECT_ID" ]; then
  echo "No project_id in project.json"
  exit 0
fi

INIT_SCRIPT="$HARNESS_DIR/projects/$PROJECT_ID/init.sh"
if [ -f "$INIT_SCRIPT" ]; then
  echo "Running init.sh for project: $PROJECT_ID"
  bash "$INIT_SCRIPT"
else
  echo "No init.sh found for project: $PROJECT_ID"
fi
