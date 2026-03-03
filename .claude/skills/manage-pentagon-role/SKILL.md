---
name: manage-pentagon-role
description: Create roles in the Pentagon organization
argument-hint: "<name> [options]"
---

Create roles in your Pentagon organization.

## Creating a new role

Write a role request JSON to Pentagon's role-requests directory.
Pentagon detects it within ~1 second and creates the role.

### Role request format

```json
{
  "name": "<role-name>",
  "requestedBy": "<your-agent-UUID>",
  "emoji": "🔍"
}
```

**Required**: `name`, `requestedBy` (your agent UUID from ORGANIZATION.md "## You" → "Agent ID:")

**Optional**: `emoji` (auto-picked from a curated set if omitted)

### Writing the request

```bash
UUID=$(uuidgen)
cat > "/tmp/role-req.json" << 'EOF'
{
  "name": "reviewer",
  "requestedBy": "YOUR-AGENT-UUID",
  "emoji": "🔍"
}
EOF

cp /tmp/role-req.json "$PENTAGON_BASE/role-requests/.$UUID.tmp"
mv "$PENTAGON_BASE/role-requests/.$UUID.tmp" "$PENTAGON_BASE/role-requests/$UUID.json"
```

**Duplicate names are silently ignored.** If a role with the same name already exists,
the request is skipped.

## Limitations

- **You cannot delete roles.** Only the human user can remove roles.
- **You cannot assign roles to agents.** Only the human user can assign roles.