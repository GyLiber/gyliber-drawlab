# Authoritative lottery-rule PDF intake — 2026-10-14

## Decision and operator scope

**No official game profile enabled. No source file copied into the repository.** This record distinguishes located URL candidates from successfully acquired, byte-hashed, completely inspected PDF documents.

The South African government's 2025 operator appointment states that ITHUBA temporarily operated from 2025-06-01 for twelve months, while Sizekhaya's fourth operator appointment began **2026-06-01**:
https://www.gov.za/news/media-statements/minister-parks-tau-announces-appointment-temporary-licensee-respect-national

As of this intake, October 2026, ITHUBA documents are historical even when published in 2025. They **cannot** independently support a current Sizekhaya official-game implementation. The present operator lists on its own site: https://www.sizekhaya.co.za/ ; currently used game names are corroborated by Sizekhaya-era Nedbank legal terms https://personal.nedbank.co.za/legal/terms-and-conditions/lottery-games-terms-and-conditions.html

## PDF discovery (candidate, not verified)

| Candidate | Publisher/source | Edition clue | Status |
| --- | --- | --- | --- |
| [2025 LOTTO / PLUS 1 / PLUS 2](https://content.nationallottery.co.za/images/docs/LOTTO_Game_Rules%26Regulations_10Oct25.pdf) | Historical national-lottery content host | filename 10Oct25, indexed 31-page document | **unavailable securely**; prior browser TLS refusal. Not retried on blocked host. |
| [2025 PowerBall / PLUS](https://content.nationallottery.co.za/images/docs/PowerBall_Game_Rules%26Regulations_10Oct25.pdf) | Same | filename 10Oct25 | **unavailable securely**; same prior TLS refusal |
| [2025 DAILY LOTTO / PLUS](https://content.nationallottery.co.za/images/docs/DAILY_LOTTO_DAILY_LOTTO_PLUS_RULE_AND_REGULATIONS_21Sep25.pdf) | Same | filename 21Sep25 | **unavailable securely**; same prior TLS refusal |
| [DAILY LOTTO rules alternative](https://www.nationallottery.co.za/assets/documents/DAILY_LOTTO_GAME_RULES_AND_REGULATIONS.pdf) | National Lottery main web domain | edition unconfirmed | Newly found official-domain URL; web request returned an HTML loading response, not inspected PDF pages. Direct download unavailable in the execution environment (DNS failure). |
| [Retailer Manual](https://www.nationallottery.co.za/assets/documents/Retailer_Manual.pdf) | National Lottery main web domain | undetermined historical edition | Newly reconsidered HTTPS path; web retrieval timed out; direct download DNS failure. |
| [ITHUBA 2019 manual](https://ithubalottery.co.za/wp-content/uploads/2019/05/Retailer-Manual-Edit-29-May-2019.pdf) | ITHUBA operator domain | 2019 filename | Search-indexed historical document; direct download unavailable (DNS failure); not suitable to assert 2025/2026 applicability. |
| Current Sizekhaya game rule PDFs | Current operator | 2026 onward | No complete accessible, authoritative PDFs recovered in bounded search. **Unresolved**. |

**Integrity register:** all candidates currently have `retrieved_file: false`, `sha256: null`, `reviewed_pages: 0`, `edition_verified: false`, `effective_period_verified: false`. Search snippets and financial-channel terms may corroborate names or geometry but do **not** replace controlling operator game rules. Do not invent checksums.

## New sources and cross-checks

- Government appointment: https://www.gov.za/news/media-statements/minister-parks-tau-announces-appointment-temporary-licensee-respect-national
- Current operator: https://www.sizekhaya.co.za/
- Sizekhaya-era bank contract: https://personal.nedbank.co.za/legal/terms-and-conditions/lottery-games-terms-and-conditions.html
- Capitec FAQ explicitly says current game rules are obtainable from authorised retailers, the Player Helpline or online: https://www.capitecbank.co.za/were-here-to-help/faqs/?selectQuestionSection=Vouchers
- Regulator's game-rule approval role: https://www.nlcsa.org.za/national-lottery/

Those public HTML sources establish leads; they are **not authenticated complete rule PDFs** or a conformance table.

## Intake protocol when actual PDF bytes are available

1. Download over valid authenticated HTTPS from operator/regulator or receive a lawful copy. Do not weaken TLS or retry the previously rejected legacy hostname without a genuinely new safe endpoint.
2. Record source URL, publisher, UTC retrieval timestamp, byte size and SHA-256 of the actual downloaded file.
3. Confirm PDF integrity, edition, all pages, approvals/amendments, effective and withdrawal dates. Preserve citations as page/section references.
4. Extract only the minimum original facts needed: pool and pick; draw and bonus/extras; dependent add-ons; multiselect/multidraw; acceptance, results and applicable scope. Do not redistribute entire copyrighted manuals in the public repo.
5. Create an independent conformance matrix and tests for each verified game family. Preserve separate ITHUBA-era vs Sizekhaya IDs and version applicability.
6. Only after review enable an official profile. No number prediction, certified physical draw, authenticated draw or ticket-processing capability is thereby established.

## Blocker and next action

The critical outstanding action is acquisition of the actual complete PDF bytes from a verified custodian. The existing **unsent** historical rules request is at `docs/client/RULE-SOURCE-REQUEST.md`. A separate request for present Sizekhaya editions should be drafted and sent only with the owner's express sending permission.

**No actual source PDF acquired or processed; this intake is documentary progress, not completion of the PDF acquisition objective.**
