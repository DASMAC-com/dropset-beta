---
name: pr-title-description
description: Write or update a PR title and description for the current branch, matching the style of recent PRs.
disable-model-invocation: true
user-invocable: true
---

# `pr-title-description`

Write (or update) the title and description
for the pull request on the current branch.

## Steps

1. Identify the current branch and its PR
   (if one exists) using
   `gh pr list --head <branch>`.

1. Get the full diff against `main`:
   `git diff main..HEAD` and
   `git log main..HEAD --oneline`.

1. Fetch the body of the 3 most recent merged
   PRs to match their style:

   ```sh
   gh pr list --state merged --limit 3 \
     --json number,title,body
   ```

1. Write a concise PR description that mirrors
   the format and tone of those recent PRs.
   Typically this means a `# Changes` section
   with a numbered list. Add a `# Background`
   section only if the changes need non-obvious
   context.

1. If a PR already exists for the branch, update
   it with `gh pr edit <number> --body "..."`.
   Otherwise, report the description so the user
   can create the PR.

1. Show the user the PR URL when done.
