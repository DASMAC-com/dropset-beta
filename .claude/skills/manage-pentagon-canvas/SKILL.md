---
name: manage-pentagon-canvas
description: Organize agents and teams on the Pentagon canvas with minimal movement
argument-hint: "[what to fix]"
---

Make small, targeted adjustments to agent and team positions on the Pentagon canvas.

## Core principle: MINIMAL MOVEMENT

**Only move things that need to move, and move them as little as possible.**

The user has already placed agents and teams where they want them. Your job is to make
small corrections — close gaps, align a row, nudge a straggler into the group — not to
redesign the entire layout. Think of it like tidying a desk, not rearranging furniture.

**Rules:**
- Never move an agent or team that is already well-positioned
- Move to nearby cells, not across the map
- Preserve the existing layout structure — don't relocate groups
- If asked to "clean up a team," that means fix spacing/alignment in place, not move
  the team somewhere else
- Fewer moves is always better than more moves

## How it works

1. **Read current positions**: Read your `directory.json` file to see all agents and
   their current grid positions. Also read ORGANIZATION.md for your own position.

2. **Identify what's wrong**: Which specific agents or teams are out of place? Most
   should stay put. Ask yourself before each move: is this truly necessary?

3. **Plan minimal moves**: Prefer 1-2 cell adjustments over large jumps.

4. **Write the signal**: Write move commands to your signal file.

   **Signal file**: `{your-agent-directory}/signal`

   ### Moving agents

   ```
   move-agent:<agent-UUID>,<col>,<row>
   ```

   Semicolon-separated for batch — the `move-agent:` prefix appears **only once**,
   then each entry is `UUID,col,row` separated by `;`. If the target is occupied,
   Pentagon finds the nearest available cell.

   ```bash
   # CORRECT — prefix once, entries separated by semicolons:
   printf 'move-agent:UUID1,5,3;UUID2,6,3;UUID3,7,3' > "$AGENT_DIR/signal"
   ```

   **Do NOT repeat the prefix** per entry (e.g., `move-agent:UUID1,5,3;move-agent:UUID2,6,3`).

   ### Repositioning team pods

   ```
   reposition-team:<team-UUID>,<col>,<row>,<width>,<height>
   ```

   Moves and/or resizes a team rug. Pass current values for any dimension you don't
   want to change.

   ```bash
   printf 'reposition-team:TEAM-UUID,8,2,5,4' > "$AGENT_DIR/signal"
   ```

   **Important**: Write the entire signal in a single `printf`. Pentagon reads and
   clears the signal file on change. Agent and team signals must be separate writes.

5. **Verify**: Re-read `directory.json` to confirm the new positions.

## Grid coordinates

- `col` increases right, `row` increases down
- Each cell is 60x60 canvas points
- For agents: occupied cells → nearest available
- For teams: col,row is the top-left origin; width,height is rug size (min 2, max 20)

## Moving or resizing a single agent or team

If you just need to move one agent or reposition one team (not organizing the whole
canvas), you can use `/manage-pentagon-agent` or `/manage-pentagon-team` directly.
This skill is for when you're tidying up the overall layout.

## Example

```bash
# alice: col 3, row 2
# bob:   col 3, row 5  ← gap of 3 rows, should be 1
# carol: col 3, row 6

# Only move bob — alice and carol are fine
printf 'move-agent:%s,3,3' "$BOB_UUID" > signal
```

Notice: only 1 agent moved, by 2 rows. Alice and carol stay put.