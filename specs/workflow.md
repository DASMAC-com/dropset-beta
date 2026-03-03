# Agent Workflow Specification

This project uses a spec-driven development workflow managed by Pentagon agents.

## Organization

### Roles

| Role | Responsibility |
|------|---------------|
| **Root** | Receives user requirements, delegates to team managers. Never writes code. |
| **Manager** | Breaks down tasks, writes specs, reviews work. Never writes code. |
| **Worker** | Implements code following specs. Works in isolated git worktrees. |

### Teams

| Team | Function |
|------|----------|
| **Pipeline** | CI/CD setup, testing infrastructure, dev tooling |
| **Spec** | Technical specifications, API design, acceptance criteria |
| **Audit** | Adversarial QA — finds flaws, gaps, and bugs in specs and implementations |

Each team has a **manager** (lead) and one or more **workers**.

## Development Lifecycle

Every change follows this sequence:

### 1. Specification First

No code is written without a spec. The Spec team produces a specification that
includes:

- **Objective**: what is being built and why
- **Requirements**: functional and non-functional
- **API/interface design**: signatures, data models, protocols
- **Acceptance criteria**: concrete, testable conditions for "done"
- **Edge cases**: known pitfalls and how to handle them

Specs are committed to `specs/` and reviewed by the Audit team before
implementation begins.

### 2. Audit Review (Pre-Implementation)

The Audit team reviews the spec adversarially:

- Are requirements ambiguous or contradictory?
- Are edge cases missing?
- Are acceptance criteria testable?
- Are there security or performance concerns?

The spec is revised until Audit approves.

### 3. Implementation

Workers implement from the approved spec. Each worker operates in an isolated
git worktree branched from `main`:

```
agent/<agent-name>-<uuid>
```

Workers must:

- Follow the spec exactly — no undocumented deviations
- Write tests that cover the acceptance criteria
- Keep changes focused — one spec per branch

### 4. Audit Review (Post-Implementation)

The Audit team reviews the implementation adversarially:

- Does the code match the spec?
- Do tests cover all acceptance criteria and edge cases?
- Are there bugs, security issues, or regressions?
- Does CI pass?

### 5. Merge

Once Audit approves, the branch is merged to `main` via pull request.

## Git Conventions

- **Branch naming**: `agent/<agent-name>-<uuid>` (e.g., `agent/ci-worker-626eb4cb`)
- **Commit style**: semantic — `type(scope): description`
  - Types: `feat`, `fix`, `chore`, `docs`, `test`, `refactor`, `ci`
  - Messages should tell the repo what to do (e.g., "Add endpoint", not "Added endpoint")
  - Example: `feat(api): Add user authentication endpoint`
- **PR title**: semantic, matching commit style
- **PR description**:
  - `## Summary` with a numbered list describing what the PR does
  - `## Testing` explaining how the changes were tested or can be tested
  - No bullet lists — use numbered lists
- **Specs directory**: `specs/` at repo root

## CI Requirements

All branches must pass CI before merge. The Pipeline team maintains:

- Linting and formatting checks
- Unit and integration test runners
- Build verification
- Any project-specific validation

## Pentagon Skills

Agents coordinate using Pentagon skills committed to `.claude/skills/`:

| Skill | Purpose |
|-------|---------|
| `/manage-pentagon-agent` | Spawn agents, move them on the canvas |
| `/manage-pentagon-team` | Create teams, reposition team rugs |
| `/manage-pentagon-role` | Create roles |
| `/manage-pentagon-canvas` | Tidy up canvas layout |
| `/send-pentagon-message` | Send messages between agents |

Managers use `/send-pentagon-message` to assign tasks to workers and coordinate
across teams.
