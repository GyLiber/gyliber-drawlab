# Development and contract governance

Client and owner: Gyile / GyLiber.

Practices follow GyLiber/gyliber-command-center at
9e2eabf352c0f0c723eea46826129c5dbfc3fe92: Conventional Commits, small coherent
units, PR verification evidence, reserved intellectual property, and documented
architecture corrections. Its deployment-specific `[skip render]` marker is
not copied because DrawLab has no Render deployment.

Work on short-lived feature branches. Keep main releasable. Do not force-push
shared history. A release requires working behavior, passing tests, reviewed
security implications, current documentation and a reproducible client demo.
Use prerelease versions while any mandatory release gate remains unresolved.

Domain boundaries: mathematics, rule evidence, CLI, browser adapter/UI, security
and release engineering. New collaborators own focused domains through PRs;
shared interface changes require a recorded ADR and affected tests. CODEOWNERS
identifies the present maintainer, not independent review or enforced protection.

This repository is public. All committed content must be public-safe. Never
publish credentials, customer data, private source documents or production logs.

## Owner settings

On 2026-10-08 the GitHub ruleset API verified active main-protection (24749025),
targeting refs/heads/main with no bypass actors. It requires PRs, resolved review
threads, up-to-date branches and the GitHub Actions checks Core and browser
verification and Dependency advisory audit. Deletion and force pushes are blocked.
Required approvals are zero while there is only one maintainer; require an
independent approval once a second maintainer exists. This does not establish
independent review. The owner reported private vulnerability reporting and account
protection completed; those account/security settings were not independently read.
Recheck actual settings when resuming; documentation alone does not enforce them.

## Checkpointed batch protocol

Choose one coherent reviewable packet, normally 1-4 related Conventional Commits.
Respect the owner's session time limit; reserve time for verification and handoff.
Complete the packet, run relevant checks and fix failures within that same scope.
Push the work to GitHub and record the branch, exact head SHA, changed behavior,
observed checks and remaining blockers. Never describe pending checks as passed.
Update the remaining next steps before returning control to the owner.

Stop after the checkpoint. Start the next packet only when Gyile says continue.
Do not silently expand the batch into integration, publication or another feature.
If blocked, preserve completed work and give the precise manual action or decision
needed. A UI interruption is recovered by reading the remote head, working-tree
state and checkpoint; never assume the previous write failed or repeat it blindly.
Use [NEXT-STEPS](../NEXT-STEPS.md) as the continuation entry point.

## Client acceptance for every minor release

Record the client-visible capability, commands to reproduce it, automated test
results, security/rules limitations, exact commit, known issues and next unit.
A demonstration must generate a valid output and show at least one rejected
invalid request. A green build alone is not client acceptance or certification.
