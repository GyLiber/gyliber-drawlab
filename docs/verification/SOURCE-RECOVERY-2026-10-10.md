# Bounded source recovery: 2026-10-10

No official profile or rule claim changed. No complete source document recovered.
The owner's blocked content-host links were not retried; no TLS bypass was used.

| Source / action | Observed result | Consequence |
|---|---|---|
| Official-domain searches for National Lottery game rules and operator transition | Indexed historical PowerBall PDFs and retailer manuals; no complete new rules obtained | Discovery only; no source digest or conformance claim |
| https://www.nationallottery.co.za/images/docs/PowerBallPowerBall-Plus-RulesRegulations-V2-1Oct19.pdf | Direct web retrieval returned 403 | Historical candidate remains unverified; not current-rule evidence |
| https://sizekhaya.co.za/ | Extracted page contained tracking/image links, no usable game-rule text | This retrieval does not establish that the site lacks rules |
| https://www.nlcsa.org.za/contact-us/ | Official page readable; Information Centre email published | Routing contact verified; specific document custodian remains unconfirmed |

Search scope: two search calls and direct reads of an alternative historical PDF,
the current operator homepage and regulator contact page. Third-party reposts were
not accepted as primary rules. These are tool observations, not global availability
claims. No files were downloaded, so no file hashes are recorded.

Outcome: [unsent source request](../client/RULE-SOURCE-REQUEST.md) and
[client demonstration guide](../client/DEMONSTRATION-2026-10-10.md). Stop repeated
retrieval of these URLs until a new authoritative lead or supplied document exists.
