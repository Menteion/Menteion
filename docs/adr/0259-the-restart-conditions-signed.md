---
status: accepted
date: 2026-11-15
depends-on: ADR-0258
title: The restart conditions signed
kind: decision
summary: The maintainers signed ADR-0258's conditions for resuming and reopening Menteion as drafted, with 2027-06-30 as the date by which Mente's M5 must be reached. Mente's register holds the same text. Development stays paused.
decision-makers: Menteion maintainers
---

# ADR-0259: The restart conditions signed

**In brief**
- **Decision.** The conditions [ADR-0258](0258-development-paused.md) drafted for resuming and reopening Menteion are signed as written. The one blank is filled: Mente's M5 not reached by 2027-06-30 reopens development in Rust by an ADR. Mente's register, in the Mente repository, holds the same text with the same signature. Nothing else changes, and development stays paused.
- **Why.** ADR-0258 left the conditions a draft until the maintainers signed them with Mente's register, and left the date open.
- **Left open.** Nothing new. The pause, its parked list and its clean-up stand as ADR-0258 wrote them.

## Context and Problem Statement

ADR-0258 paused development on 2026-11-15 and drafted four conditions for the restart, to be signed together with Mente's register. The second kind of change it allows during the pause is an ADR that records that signature.

On 2026-11-15, in the session that manages Mente, the maintainers were given the four conditions in full and the three kinds of change the pause allows. They were told that naming the date signs the conditions as written, and to say so if they wanted any of them changed. That session recommended 2027-03-31 and offered 2027-06-30. The maintainers answered "2027-06-30" and changed nothing else. They confirmed it in this repository's session the same day.

## Decision Drivers

- **The maintainers' signature** (2026-11-15).
- **One text in two registers**: Menteion's ADR and Mente's register hold the same conditions, so neither project can read them differently.

## Considered Options

1. **The date**: (a) 2027-06-30, the maintainers' answer; (b) 2027-03-31, the recommendation.

## Decision Outcome

**Option 1(a).** The conditions are in force from 2026-11-15:
1. **A kernel in Mente, around Mente's M5.** A kernel of [docs/mente-port.md](../mente-port.md) is ported as Mente's systems workload. It is held bit for bit to the tests that document names at the frozen tree (the tag `menteion-paused-2026-11-15`), or at the commit that adds a pinning test during the pause. It is Mente's work; it changes nothing here but by the second kind of change below.
2. **The engine weighed after Mente's M7.** The maintainers decide, by an ADR, whether the engine resumes in Mente, resumes in Rust, or stays paused.
3. **Reopened in Rust sooner**, by an ADR, when any of these holds:
   - a research question the maintainers judge cannot wait for Mente, named in that ADR;
   - Mente's M5 is not reached by 2027-06-30;
   - a defect in the frozen tree that Mente's port depends on, which a pinning test or a change to the document cannot meet.
4. **What resuming does first.** The resuming ADR restores the weekly schedule in `.github/workflows/ci.yml` and the whitepaper's directive. It calibrates the whole tree at `scope=exhaustive` on its own commit before any round, on whatever toolchain `rust-toolchain.toml` then pins, and rewrites `docs/STATUS.md`. It reads ADR-0258's parked list.

**During the pause** two kinds of change may still merge, each by a pull request of its own, now that the signature is recorded: the ADR that resumes or reopens development, and a change Mente's port needs that changes no behaviour of the engine. The second is a test that pins a kernel already in the tree, or a correction to [docs/mente-port.md](../mente-port.md), which then names it.

## Consequences

- **Good**: the restart has a fixed rule in both projects, with a date by which a slow Mente reopens the work in Rust.
- **Neutral**: nothing in the engine, the pause or the parked list changes.

## Alternatives considered and why rejected

- **Option 1(b), 2027-03-31**: the maintainers chose the later date.

## Confirmation

`docs/STATUS.md` names the date. `npm run spec` holds the links, the tables and the version.
