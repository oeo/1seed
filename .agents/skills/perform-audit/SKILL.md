---
name: perform-audit
description: Performs evidence-backed repository audits when the operator requests audit all, maturity assessment, release readiness, security review, or a gate before a consequential feature or release.
---

# Perform Audit

Audit current repository behavior for correctness, security, reliability,
architecture, testing, deployment, and release readiness.

Prefer Monk repository-mapping and source-ingestion tools. If unavailable or
insufficient, use available equivalents and report lost coverage. Stop when
required coverage cannot be completed.

## Safety

- Remain read-only during the audit.
- Never install or update tools or dependencies.
- Never alter source, configuration, lockfiles, tests, generated files, or
  existing worktree changes.
- Safe checks may write only ignored, disposable build or test output declared
  by the repository. Compare worktree state before and after verification.
- Require explicit approval before using credentials, production services,
  containers, VMs, persistent storage, physical devices, privileged host state,
  or network-dependent checks.
- Exclude dependency caches, generated output, and vendored code from source
  findings unless the dependency or generated contract is itself in scope.
- Do not create an audit artifact without operator approval.

## Current-State Independence

- Establish every finding from current `HEAD` and current worktree behavior.
- Do not use earlier audit artifacts or old findings as candidates.
- History may explain ownership and intent, but it is not finding evidence.
- Treat plans and security documents as claims to verify against current source,
  callers, tests, manifests, and executable probes.
- Report only issues independently supported by current-state evidence. Use a
  reproducible probe when safe; static source evidence remains valid when a
  probe would be destructive, privileged, or unsafe.

## Identity

Record the exact runtime model ID when available. If it is unknown, ask the
operator rather than guessing. For filenames, remove the provider prefix,
lowercase the ID, replace punctuation and spaces with hyphens, and collapse
repeated hyphens.

## Meditation

Before auditing:

1. Run `monk tree` on the repository.
2. Read applicable `AGENTS.md` or `CLAUDE.md` instructions and relevant skills.
3. Consult reflection history when requested or relevant, for context only.
4. Read existing task state when relevant.
5. Delegate only within the authorized helper count and scope. If coverage
   exceeds available context, report the blocker and obtain direction rather
   than silently narrowing scope or adding helpers.
6. Record threshold, estimated tokens, inspected paths, delegated scopes, and
   exclusions.

Read repository-owned source, contracts, callers, tests, manifests, CI,
packaging, deployment files, and colocated documentation needed to verify each
claim. Follow the current Meditative Ritual's full-ingestion requirements.

## Audit Scope

Cover every applicable dimension:

- `gate`: crashes, persistent data or state loss, unavailable core operation,
  broken recovery, and unsafe release paths.
- `sec`: authentication, authorization, trust boundaries, secrets, code
  execution, unsafe exposure, artifact integrity, and dependency risk.
- `bugs`: wrong behavior, races, stale state, ignored results, invalid
  assumptions, and cross-component contract failures.
- `perf`: unbounded work, resource exhaustion, blocking hot paths, uncontrolled
  memory or cardinality growth, and latency-sensitive regressions.
- `arch`: conflicting contracts, duplicate ownership, layer violations, and
  missing lifecycle boundaries.
- `debt`: obsolete or disabled paths, placeholders, stale generated contracts,
  missing migrations, and undocumented operational dependencies.
- `smell`: swallowed errors, ambiguous state, deep control flow, and state spread
  across unrelated owners.
- `split`: modules whose unrelated responsibilities obstruct verification.
- `mod`: consequential obsolete runtime patterns, not stylistic preference.
- `drift`: duplicated configuration, protocol facts, versions, environment
  names, generated consumers, and documentation claims.
- `tests`: active, skipped, disabled, flaky, empty, missing failure-path, and
  missing integration coverage.
- `build-release`: reproducibility, target compatibility, signing, manifests,
  installers, updates, rollback, and release gates.
- `maturity`: evidence level for each applicable product boundary.

Skip dimensions with no consequential findings.

For 1seed, always consider these durable impact classes: seed or derived key
disclosure through output, logs, files, or process memory; a derivation change
that silently alters existing keys for the same seed and realm; weakened KDF
parameters outside test builds; seed storage fallback that writes with wrong
permissions; and acceptance of unauthenticated or malformed ciphertext or
signatures. Audit each claimed operating system and credential store separately
where behavior differs.

## Delegation

Derive scopes from the mapped component graph and trust boundaries. Partition
independent areas when needed, such as seed storage, key derivation, encryption,
signing, CLI input handling, platform support, and release delivery.

When delegation is authorized, each agent must meditate on its scope before auditing. Agents return
current evidence with file and line references. Main agent deduplicates symptoms
under root causes and resolves disagreements.

## Verification

Discover checks from repository instructions, skills, CI, task runners,
manifests, and documentation. Do not infer commands from ecosystem conventions.

Classify each candidate before running it:

- `safe`: bounded, noninteractive, unprivileged, and limited to disposable
  repository-declared output.
- `approval required`: uses network access, credentials, services, containers,
  VMs, persistent storage, physical devices, or host-state changes.
- `unavailable`: missing toolchain, operating system, hypervisor, hardware,
  fixture, or access.

Inspect repository-owned command entrypoints, hooks, and scripts before running
them. Unknown or untrusted code requires isolation or operator approval even
when its documented command appears safe.

Run only safe checks without additional approval. Respect project timeout and
fail-fast requirements. Record exact command, working environment, exit status,
and observed result. Redact credentials, tokens, private user data, and other
secrets from recorded output. Report skipped and unavailable checks as
limitations, never passes. Passing tests do not override a demonstrated defect.

Label platform evidence as `source-inspected`, `compiled`, or
`executed on target`. Never treat mocks, compilation, or one host platform as
proof of behavior on another target.

## Findings

Report findings first, ordered critical, high, medium, low:

```text
c1  path:line[, path:line]  critical: observable problem. required fix direction.
h1  path:line[, path:line]  high: observable problem. required fix direction.
m1  path:line[, path:line]  medium: observable problem. required fix direction.
l1  path:line[, path:line]  low: observable problem. required fix direction.
```

Severity meanings:

- critical: arbitrary code execution, credential or trust-boundary compromise,
  durable corruption, financial loss, or malicious artifact acceptance.
- high: exploitable wrong behavior, serious denial of service, data loss,
  broken recovery, or unavailable claimed production path.
- medium: consequential reliability, portability, performance, lifecycle,
  drift, or verification deficiency with bounded blast radius.
- low: bounded inconsistency with measurable operational cost.

Do not report formatting preferences or generic best practices. Merge duplicate
symptoms under their root cause and preserve strongest evidence and relevant
locations.

## Maturity

Derive assessment areas from mapped architecture. Always assess:

- core behavior
- security and trust boundaries
- claimed target matrix
- testing
- build and release process

Add only applicable areas, such as persistence and recovery, key formats,
packaging, or an operator-requested feature.

Rate each area:

- `unverified`: executable evidence unavailable.
- `prototype`: primary path demonstrated.
- `alpha`: primary and major failure paths have automated evidence.
- `beta`: claimed targets and integrations pass repeatably.
- `release candidate`: artifacts, installation, update, recovery, and release
  gates pass.
- `production-ready`: no critical or high findings and every release claim has
  a reproducible gate.

Assess each claimed operating system and credential store separately. Overall
maturity cannot exceed the weakest release-critical area. Include
requested-feature readiness only when the operator named a feature.
Approve release only when no critical or high findings remain and every
release-critical claim has a reproducible gate. Otherwise block release and name
the missing gates.

## Audit Artifact

After presenting results, propose:

`audits/MM-DD-YYYY-<model-slug>.md`

Use a numeric suffix when that path exists. Ask:

> Write this audit to `<proposed-path>`?

Do not create a directory or file until approved.

Use this compact structure:

````markdown
---
schema: oneseed.audit.v1
audited_at: YYYY-MM-DD
model_id: provider/model
repository: owner/name
commit: full-commit-sha
branch: branch-name
scope: description
meditation_min: number
worktree: clean-or-dirty
worktree_digest: sha256-or-clean
---

# Audit: repository-name

## Verdict

Overall maturity: level

Release decision: blocked-or-approved

## Findings

```text
h1  path:line  high: problem. fix direction.
```

## Maturity

| Area | Maturity | Evidence |
|---|---|---|
| applicable area | level | concise evidence |

## Verification

| Command or probe | Environment | Exit | Result |
|---|---|---:|---|
| `command` | target | code | exact result |

## Required Gates

1. First independently verifiable gate.

## Coverage

State mapped paths, inspected scopes, threshold, delegated scopes, evidence
classes, target matrix, changed and untracked paths, and exclusions.

## Limitations

State unavailable tooling, operating systems, hypervisors, hardware,
credentials, services, checks, and assumptions. Write `None.` when no
limitations remain.
````

Add a requested-feature decision and maturity row only when the operator asked
for that gate.

## Writing

After approval:

1. Recheck that the proposed path does not exist.
2. Create `audits/` only when needed.
3. Write with the native file-edit tool.
4. Reread the artifact and validate provenance, findings, verification,
   coverage, and limitations.
5. Report the written path.
6. Do not commit unless explicitly requested.

## Completion

State overall maturity, release decision, verification summary, coverage,
limitations, and artifact status. Include requested-feature readiness only when
applicable.
