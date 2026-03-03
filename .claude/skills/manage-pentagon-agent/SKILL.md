---
name: manage-pentagon-agent
description: Spawn new agents and manage their canvas positions
argument-hint: "<action> [options]"
---

Spawn new agents and manage their positions on the Pentagon canvas.

## Spawning a new agent

Write a spawn request JSON to Pentagon's spawn-requests directory.
Pentagon detects it within ~1 second and creates the agent.

### Spawn request format

```json
{
  "name": "<agent-name>",
  "requestedBy": "<your-agent-UUID>",
  "directory": "<repo-directory — optional, defaults to your repo>",
  "soul": "<soul description — optional>",
  "purpose": "<purpose description — optional>",
  "initialMessage": "<first message to send the new agent — optional>",
  "teams": ["<team-name-or-UUID>"],
  "role": "<role-name-or-UUID>",
  "col": 5,
  "row": 3
}
```

**Required**: `name`, `requestedBy` (your agent UUID from ORGANIZATION.md "## You" → "Agent ID:")

**Optional**: `directory`, `soul`, `purpose`, `initialMessage`, `teams`, `role`, `col`/`row`
(if col/row omitted, agent spawns next to you)

### Writing the request

```bash
UUID=$(uuidgen)
cat > "/tmp/spawn-req.json" << 'EOF'
{
  "name": "researcher",
  "requestedBy": "YOUR-AGENT-UUID",
  "soul": "Thorough researcher who digs deep.",
  "purpose": "Research competitive landscape.",
  "initialMessage": "Please research the top 5 competitors."
}
EOF

cp /tmp/spawn-req.json "$PENTAGON_BASE/spawn-requests/.$UUID.tmp"
mv "$PENTAGON_BASE/spawn-requests/.$UUID.tmp" "$PENTAGON_BASE/spawn-requests/$UUID.json"
```

## Moving an agent on the canvas

Write a `move-agent:` signal to your signal file to reposition agents on the canvas.

**Signal file**: `{your-agent-directory}/signal` (find your agent directory in ORGANIZATION.md)

**Format** — the `move-agent:` prefix appears **only once**, then entries are semicolon-separated:
```
move-agent:<agent-UUID>,<col>,<row>
```

Batch (multiple agents in one signal):
```
move-agent:<UUID1>,<col>,<row>;<UUID2>,<col>,<row>
```

```bash
# Single agent
printf 'move-agent:AGENT-UUID,5,3' > "$AGENT_DIR/signal"

# Batch — prefix once, semicolon between entries
printf 'move-agent:UUID1,5,3;UUID2,6,3;UUID3,7,3' > "$AGENT_DIR/signal"
```

**Do NOT repeat the prefix** per entry (e.g., `move-agent:UUID1,5,3;move-agent:UUID2,6,3`).

Grid coordinates: `col` increases right, `row` increases down. Each cell is 60x60 points.
If the target cell is occupied, Pentagon places the agent in the nearest available cell.

Find agent UUIDs in your `directory.json` file.

## Organizing multiple agents

To tidy up agent positions (close gaps, align rows, fix spacing), use the
`/manage-pentagon-canvas` skill instead — it has guidelines for making minimal,
non-disruptive adjustments to the layout.

## Limitations

- **You cannot delete agents.** Only the human user can remove agents from Pentagon.
  If an agent is no longer needed, let the user know.
- **You cannot delete teams or roles.** Only the human user can remove them.