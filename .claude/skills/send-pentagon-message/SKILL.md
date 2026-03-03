---
name: send-pentagon-message
description: Send a message to another Pentagon agent
argument-hint: "<recipient> <message>"
---

Send a message to another agent in your Pentagon organization.

## Usage

/send-pentagon-message <recipient> <message>

The recipient can be a **name** or **UUID**:
- `/send-pentagon-message atlas Check the logs` (by name)
- `/send-pentagon-message A1B2C3D4-... Check the logs` (by UUID)

If multiple agents share the same name, use the UUID to be precise.

## How it works

1. Read your directory.json to find the recipient's entry and inbox path.
   Match on `name` (case-insensitive) or `id` (exact UUID). If the name
   matches multiple entries, pick the one that best fits context or ask
   the user to clarify.

2. Build your own contact card for the `from` field. Find your Pentagon
   name in ORGANIZATION.md under "## You" → "Name:" — use that name,
   NOT "claude" or "Claude Code". Your `from` must use the same shape
   as entries in directory.json:

    ```json
    {
      "id": "<generate-a-UUID>",
      "from": {
        "id": "<your-agent-UUID>",
        "name": "<your-pentagon-name>",
        "status": "active",
        "teams": [],
        "role": null,
        "workspace": null,
        "source": null,
        "position": {"col": 0, "row": 0},
        "currentTask": null,
        "inbox": "<your-agent-dir>/inbox"
      },
      "to": "<recipient-UUID>",
      "content": "<your message text>",
      "ts": "<current ISO-8601 timestamp>",
      "conversationId": "<thread-UUID — required>",
      "replyTo": "<message-UUID-you-are-replying-to or null>"
    }
    ```

3. Write atomically to the recipient's inbox:
   - First write to: `{recipient-inbox}/.{id}.tmp`
   - Then rename to: `{recipient-inbox}/{id}.json`

Pentagon will detect the file and deliver a notification to the recipient.

## Finding recipients

Read your directory.json file. Each entry has:
- **name**: human-readable name
- **id**: unique UUID (use this when names are ambiguous)
- **status**, **teams**, **role**
- **inbox**: the file path to write messages to

## Conversation threads

Every message must have a `conversationId` — it groups messages into threads
(like email). When starting a new conversation, generate a fresh UUID. When
replying, reuse the `conversationId` from the message you received and set
`replyTo` to that message's `id`. `replyTo` is optional (null for the first
message in a thread).

## Receiving messages

When you receive a message, you'll see a notification in your terminal:

    New message from atlas. Read: /path/to/inbox/{messageId}.json

Read the file to see the full message and sender info. Reply using
the sender's inbox path from the `from.inbox` field.