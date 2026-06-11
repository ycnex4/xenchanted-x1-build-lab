# Stage 2.5 SPL Token Decision

This document records the token program decision for the first Stage 2.5 token mint CPI prototype.

## Decision

Stage 2.5 will use the standard SPL Token program for the first XXXL mint CPI prototype.

Selected token program:

- SPL Token
- TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA

Token-2022 is not used in the first Stage 2.5 CPI prototype.

## Reason

The first Stage 2.5 objective is to prove gateway mint atomicity on X1 testnet:

    guardian approval + context-bound message_hash + ProcessedBurnEntry + mint CPI

The goal is not to prove advanced token extension behavior.

SPL Token is preferred for the first prototype because:

- simpler CPI interface
- fewer token extension edge cases
- simpler mint account validation
- simpler recipient token account validation
- lower account and compute complexity
- better fit for proving the atomic gateway mint path

## Token-2022 status

Token-2022 remains a future compatibility option, not a Stage 2.5 requirement.

Token-2022 may become relevant later if XXXL needs:

- non-transferable token behavior
- mint-level extensions
- transfer hooks
- metadata extensions
- future X1 Forge or Stake token constraints

None of these are required for the first Stage 2.5 CPI prototype.

## Theo review conclusion

Theo agreed that SPL Token is the correct choice for Stage 2.5.

Key review conclusion:

- SPL Token is the right path for the first CPI prototype
- Token-2022 adds complexity without benefit for the current objective
- Stage 2.5 is proving gateway atomicity, not extension behavior
- Token-2022 can remain a separate future upgrade if there is a concrete need

## Implementation implication

Stage 2.5 CPI implementation should target SPL Token mint_to.

The next design decisions are:

1. XXXL mint account creation path
2. gateway PDA mint authority seed model
3. recipient token account policy
4. compute budget strategy
5. mint CPI failure rollback tests

## Current conclusion

Prerequisite 1 for Stage 2.5 is closed.

The first Stage 2.5 CPI prototype will use SPL Token.


## Prototype-only mint authority boundary

Stage 2.5 may use a gateway mint_authority PDA for the first CPI prototype.

This is explicitly prototype-only.

It is not the final XXXL production authority model.

Reason:

- Stage 2.5 proves gateway verification + replay protection + mint CPI atomicity.
- Stage 2.5 does not decide all future XXXL mint sources.
- Future X1-side mechanics may also mint XXXL.
- Stake redeem may require reward minting.
- Forge redeem may require token minting.
- Other future protocol mechanics may introduce additional mint paths.

Therefore, the final XXXL authority model remains open until Stake and Forge architecture are defined.

Likely future production direction:

- a separate XXXL Core/Minter authority program
- or another shared authority model that can support multiple approved protocol mint paths

Gateway PDA authority must not be treated as the final production model.

## Theo authority review conclusion

Theo confirmed:

- gateway PDA mint authority is acceptable for Stage 2.5 prototype-only CPI evidence
- it should not be treated as the final authority model
- the final XXXL authority model remains TBD before production
- Stake redeem, Forge redeem, and other X1-side mechanics may require a separate Core/Minter authority layer
- no blocker exists against using gateway PDA for Stage 2.5 if this limitation is documented

Current authority status:

| Stage | Mint authority | Status |
| --- | --- | --- |
| Stage 2.5 CPI prototype | Gateway PDA | Prototype-only |
| Stake/Forge design | TBD | Open |
| Production | Likely Core/Minter PDA | Future decision |
