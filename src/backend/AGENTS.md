# AI Agent Logging and Error Message Standards

## Purpose and Scope
- AI agents may **only modify logging, tracing, and error‑message behavior** within this backend.  
- Agents **must not** alter business logic, API behavior, database models, security logic, or side‑effects.  
- These standards apply to all files under `src/backend/` unless a more specific `AGENTS.md` overrides them.

## Tracing and Log Levels
The project uses the [`tracing`](https://docs.rs/tracing/latest/tracing/) crate with levels:

- `debug` — internal visibility only; stripped in production.
- `info` — surfaced to operators; meaningful high‑level events.
- `warn` — recoverable issues; requires clear operator‑action context.
- `error` — failures preventing intended work; must include actionable context.

## Logging Rules

### 1. Audience and Clarity
- `info`, `warn`, and `error` logs must be understandable **without reading code**.
- Avoid jargon; use short, direct, operator‑focused language.

### 2. Structured Context
- Prefer structured fields over message concatenation:
  - `debug!(user_id, session_id, "create session started")`
- Include identifiers needed to trace the event: IDs, operation names, resource names.
- **Never** log secrets or sensitive personal data.

### 3. Level Usage
- **debug**
  - Use freely: function entry/exit, branches, retries, external calls.
  - Document important variables (sanitized) and execution flow.
- **info**
  - Startup, shutdown, major lifecycle transitions.
  - Successful user‑facing or operator‑visible actions.
- **warn**
  - Recoverable anomalies: timeouts, degraded performance, transient failures.
  - Provide remediation hints where appropriate.
- **error**
  - Non‑recoverable failures or aborted operations.
  - Include outcome, failing subsystem, and relevant identifiers.

### 4. Error Reporting and Mapping
- Map internal errors to clear user/operator‑facing messages.  
- Internal details belong in structured `debug` logs.  
- When propagating errors upward, note:
  - what failed,
  - why it matters,
  - what the impact is.

### 5. Consistency
- Use imperative mood: *"fetch user"*, *"persist record failed"*.
- Prefer keywords indicating outcome:
  - `started`, `completed`, `failed`, `skipped`, `retrying`.
- Never log success at `warn` or `error`.

### 6. Safety and Redaction
- Never log:
  - passwords
  - auth tokens
  - secrets/keys
  - full PII  
- Redact or hash where necessary.

### 7. Placement Requirements
Agents should insert at least:

- Function entry/exit logs (`debug`).
- Logs before/after:
  - DB operations  
  - external HTTP calls  
  - filesystem interactions  
- Branch decisions:
  - permission checks  
  - fallbacks  
  - retries/backoff  
- All error paths.

### 8. Formatting
- Prefer single‑line messages under 120 chars.
- Use lowercase, no trailing punctuation.
- Example:

  ```
  debug!(user_id, "update permissions started")
  error!(user_id, "update permissions failed")
  ```

### 9. Testing and Review
- Ensure logs compile in both `debug` and `release`.
- Review for appropriate levels and clarity before merging.

## Agent Responsibilities
### Agents **may**:
- Add, remove, or adjust logging/tracing statements.
- Improve error messages and error‑mapping logic.
- Add missing context fields to logs.
- Ensure compliance with this standard.

### Agents **may not**:
- Modify business logic, algorithms, backend behavior, or API responses (other than error messages).
- Change database queries or schemas.
- Add new features, flows, or side‑effects.

### Escalation
- Any required change outside logging/error scope **must receive explicit human approval** before modification.
