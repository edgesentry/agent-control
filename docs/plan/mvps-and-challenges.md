# MVPs and challenge selection

## Two MVPs — do not conflate

| Horizon | Audience | Goal |
|---------|----------|------|
| **Submission MVP** (this plan, **by 30 Jun 2026**) | Cap Vista evaluators | Credible OSS repo + live demo + CoC annex evidence that Phase 1 is feasible |
| **Phase 1 trial MVP** (post-award, if shortlisted) | End users in funded trial | CS02: 1 month · CS01: 2 months — **on-prem / portable lab bundle** primary |

Submission MVP **proves readiness**; Phase 1 MVP **executes the trial contract** on evaluator-hosted or air-gapped hardware where required.

Phase 1 detail: [Phase 1 trial](phase1-trial.md).

## Challenge selection

Apply to **both CS01 and CS02** with one platform story:

| CS | Primary / secondary | Repo focus |
|----|---------------------|------------|
| **CS02** | **Primary** — OWASP coverage is an explicit KPI | `apps/lab`, `catalog/`, Guardian as red-team oracle |
| **CS01** | **Secondary** — same Guardian reused for production SOC guardrails | `apps/soc`, analyst approval gate |

Unified narrative: **CS02 lab proves security; CS01 reuses the same policy pack in operations.**

CS01 submission slice: [CS01 SOC slice](cs01-soc-slice.md).
