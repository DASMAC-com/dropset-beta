---
name: manage-pentagon-team
description: Create teams and manage their position and size on the canvas
argument-hint: "<action> [options]"
---

Create teams and manage their position and size on the Pentagon canvas.

## Creating a new team

Write a team request JSON to Pentagon's team-requests directory.
Pentagon detects it within ~1 second and creates the team rug on the canvas.

### Team request format

```json
{
  "name": "<team-name>",
  "requestedBy": "<your-agent-UUID>",
  "col": 5,
  "row": 3,
  "gridWidth": 3,
  "gridHeight": 3,
  "color": "#06b6d4",
  "emoji": "🛡️"
}
```

**Required**: `name`, `requestedBy` (your agent UUID from ORGANIZATION.md "## You" → "Agent ID:")

**Optional**: `col`/`row` (top-left corner; defaults near you), `gridWidth`/`gridHeight`
(default 3×3), `color` (hex; auto-assigned if omitted), `emoji`

### Writing the request

```bash
UUID=$(uuidgen)
cat > "/tmp/team-req.json" << 'EOF'
{
  "name": "security",
  "requestedBy": "YOUR-AGENT-UUID",
  "gridWidth": 4,
  "gridHeight": 3,
  "emoji": "🛡️"
}
EOF

cp /tmp/team-req.json "$PENTAGON_BASE/team-requests/.$UUID.tmp"
mv "$PENTAGON_BASE/team-requests/.$UUID.tmp" "$PENTAGON_BASE/team-requests/$UUID.json"
```

Agents positioned inside the rug's bounds are automatically assigned to the team.

**Duplicate names are silently ignored.** If a team with the same name already exists,
the request is skipped. Use the reposition signal below to modify existing teams.

## Repositioning or resizing an existing team

Write a `reposition-team:` signal to your signal file to move and/or resize a team pod.

**Signal file**: `{your-agent-directory}/signal` (find your agent directory in ORGANIZATION.md)

**Format** (semicolon-separated for batch):
```
reposition-team:<team-UUID>,<col>,<row>,<width>,<height>
```

- `col`,`row`: new top-left corner in grid coordinates
- `width`,`height`: new size in grid cells (min 2, max 20)

```bash
# Resize Security team to 5x5 and move its origin to col 8, row 2
printf 'reposition-team:%s,8,2,5,5' "$TEAM_UUID" > "$AGENT_DIR/signal"
```

To move without resizing, pass the current width and height.
To resize without moving, pass the current col and row.

Find team UUIDs from your `directory.json` (each agent entry lists its teams with IDs).

**Important**: Write the entire signal in a single `printf` or `echo -n`. Pentagon
reads and clears the signal file on change, so partial writes may be lost.

## Organizing teams on the canvas

To tidy up team positions alongside agents (close gaps, fix spacing), use the
`/manage-pentagon-canvas` skill instead — it has guidelines for making minimal,
non-disruptive adjustments to the layout.

## Limitations

- **You cannot delete teams.** Only the human user can remove teams.
- **You cannot rename teams.** Only the human user can rename or reconfigure teams.