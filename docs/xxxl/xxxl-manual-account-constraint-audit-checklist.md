# XXXL Manual Account Constraint Audit Checklist

Status: COMPLETED.

This stage records the manual account-constraint audit for the current XXXL SVM runtime scaffold before live handler wiring.

The goal is not to activate the live gateway route. The goal is to document exactly what the current runtime boundary already checks and what must remain mandatory before any live route execution can be enabled.

## Scope

Audited runtime area:

- consume_gateway_mint instruction boundary
- account ordering
- PDA mint authority boundary
- SPL Token mint_to CPI boundary
- program-owned account owner/rent checks
- SPL mint and recipient token account checks
- processed-event replay boundary
- recipient balance boundary
- atomic mutation ordering assumptions
- no live route activation policy

Out of scope:

- production deploy
- final Program ID
- live route activation
- guardian signature verification inside runtime
- admin/governance model
- freeze/finalization ceremony
- Mollusk/Trident fuzzing

## Baseline verification

Hard checks at the start of this stage:

- cargo fmt --check: pass
- cargo test: pass
- cargo clippy --all-targets -- -D warnings: pass
- cargo audit: pass
- cargo deny check licenses: pass
- cargo deny check bans: pass
- cargo deny check sources: pass

Rust tests observed:

- 63 passed
- 0 failed

Allowed cargo audit warnings remain:

- bincode 1.3.3
- libsecp256k1 0.6.0
- rand 0.7.3

These are warning-level findings in the current cargo audit configuration, not hard vulnerabilities in this stage.

## Canonical account order

consume_gateway_mint currently expects 9 accounts.

| Index | Account | Role |
|---:|---|---|
| 0 | mint_state | XXXL program-owned mint state account |
| 1 | gateway_config / route account | XXXL program-owned route configuration account |
| 2 | guardian_set | XXXL program-owned guardian-set/account metadata boundary |
| 3 | processed_event | XXXL program-owned replay-protection event account |
| 4 | recipient_balance | XXXL program-owned recipient balance/accounting account |
| 5 | spl_token_mint | SPL Token mint account for XXXL |
| 6 | recipient_token_account | recipient SPL token account / ATA boundary |
| 7 | mint_authority_pda | PDA signer used as SPL mint authority |
| 8 | token_program | canonical SPL Token program |

Instruction-level account index expectations:

- account_meta_count must equal 9
- route account index must equal 1
- guardian set account index must equal 2
- mint state account index must equal 0
- processed event account index must equal 3
- recipient balance account index must equal 4

## Program-owned account constraints

The following accounts are expected to be owned by the XXXL program and rent-exempt:

- mint_state
- gateway_config / route account
- guardian_set
- processed_event
- recipient_balance

Required checks:

- account.owner == program_id
- Rent::is_exempt(account.lamports(), account.data_len())
- account data discriminator matches expected account type
- account data version matches runtime layout version
- account data length matches or satisfies the expected layout boundary

Current status:

- owner and rent checks are present in the preparation boundary.
- account layout views reject wrong discriminators, wrong versions, and truncated data.
- this remains pre-live-runtime and does not execute the live mint route.

## SPL Token program constraint

The token program account must be the canonical SPL Token program:

- TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA

Required check:

- token_program.key == spl_token::id()

Current status:

- token_program key check is present before preparing the CPI boundary.

## SPL mint account constraints

The SPL mint account must be:

- owned by SPL Token
- rent-exempt
- initialized
- the target mint specified by the instruction and route/mint state
- configured with the expected mint authority PDA

Required checks:

- mint account owner is SPL Token
- rent exemption holds
- SPL Mint state is initialized
- mint authority equals gateway mint authority PDA
- mint pubkey matches args.mint_id
- decimals are read and carried into the prepared CPI boundary

Current status:

- initialized mint and mint authority checks are present.
- mint decimals are captured in PreparedConsumeGatewayMintCpi.
- live process_instruction still does not invoke mint_to.

## Recipient token account / ATA boundary

The recipient token account must be:

- owned by SPL Token
- rent-exempt
- initialized
- for the expected mint
- owned by the expected recipient

Required checks:

- token account owner is SPL Token
- rent exemption holds
- SPL token account state is initialized
- token account mint == args.mint_id
- token account owner == args.recipient

Current status:

- recipient ATA boundary checks are present.
- wrong recipient token mint and wrong owner are rejected by tests.

## Gateway mint authority PDA

Canonical seeds:

- xxxl
- gateway-mint-authority
- v1
- bump from find_program_address

Required checks:

- derive PDA with Pubkey::find_program_address
- supplied mint_authority_pda account key equals derived PDA
- supplied bump equals derived bump
- PDA signer seeds are used for invoke_signed
- PDA is not an external signer requirement
- PDA is used only as CPI mint authority

Current status:

- PDA derivation is real.
- wrong PDA is rejected before invoke_signed.
- wrong bump is rejected.
- invoke_signed boundary uses the PDA signer seed set.

## Route / gateway config constraints

The route configuration must match the consume_gateway_mint instruction.

Required checks:

- route_id matches args.route_id
- guardian_set_id matches expected guardian set linkage
- target mint matches args.mint_id
- source chain / route weight policy remains bounded by configured route policy
- disabled or non-activated routes must not mint

Current status:

- route mismatch is rejected.
- gateway config target mint mismatch is rejected.
- route activation remains blocked at process_instruction level.
- no live gateway route is activated in this stage.

## Guardian set boundary

Guardian signatures remain outside runtime for the current model.

Required account constraints:

- guardian_set account must be program-owned
- guardian_set account must be rent-exempt
- guardian_set layout/discriminator/version must be valid
- guardian_set_id must match the gateway config / route expectation

Current status:

- guardian set account is part of the canonical account list.
- guardian set account is included in program-owned owner/rent/layout boundary.
- signature verification is intentionally not moved into this runtime stage.

## Processed event replay boundary

The processed event account protects canonical source event replay.

Required checks before mutation:

- processed_event is not already consumed
- canonical_event_key matches args.canonical_event_key
- route_id matches args.route_id
- recipient matches args.recipient
- event/account layout is valid

Required mutation:

- mark processed event consumed
- write consumed amount
- write consumed slot

Current status:

- replay is rejected before crediting balance.
- consumed processed event is rejected.
- wrong canonical event key is rejected by state mutation tests.
- wrong route/recipient are rejected by boundary checks.

## Recipient balance boundary

Recipient balance account is the program-owned accounting mirror for the recipient and mint.

Required checks:

- recipient_balance.owner == args.recipient
- recipient_balance.mint == args.mint_id
- account layout is valid
- balance addition must not overflow

Required mutation:

- credit recipient balance
- write last event key / last mint metadata as defined by state layout

Current status:

- wrong recipient balance owner is rejected.
- overflow is rejected before marking processed.
- balance mutation succeeds in the atomic fixture.

## Amount constraints

Required checks:

- args.amount > 0
- args.amount <= u64::MAX before SPL mint_to
- amount used by accounting and amount used by SPL mint_to must be identical

Current status:

- zero amount is rejected.
- amount larger than u64::MAX is rejected.
- atomic execution plan rejects amount mismatch.

## Atomicity policy

Required order:

- validate all account constraints first
- prepare CPI boundary
- reject replay before any credit
- reject overflow before marking processed
- mark processed event consumed
- credit recipient balance
- invoke SPL mint_to only through PDA signer boundary when live route activation is explicitly enabled

Current status:

- current process_instruction remains scaffold-only and does not activate live minting.
- atomic mutation fixture preserves no-state-change-on-failure expectations for replay and overflow.
- prepared CPI boundary exists, but live process_instruction mint_to invocation remains disabled.

## Writable / executable constraints before live activation

Before live route activation, the live handler must explicitly enforce or rely on runtime-required writable/executable constraints for:

- processed_event writable
- recipient_balance writable
- SPL token mint writable
- recipient token account writable
- token_program executable / canonical SPL Token program
- mint_authority_pda present as PDA authority, not user signer

This checklist records these as mandatory before live route activation, even if some constraints are already enforced indirectly by SPL Token CPI or Solana runtime behavior.

## Current live-route status

Current status:

- scaffold-only
- not deployable as live gateway
- no route activation
- no live mint execution from process_instruction
- no manual mint
- no premine
- no founder allocation
- no hidden emission
- no Build-derived supply right

## Conclusion

The current SVM runtime scaffold has a clear account boundary and a documented manual account-constraint checklist.

The next implementation stage may wire the live handler only if these constraints remain preserved and if the route activation/finalization policy explicitly allows it.
