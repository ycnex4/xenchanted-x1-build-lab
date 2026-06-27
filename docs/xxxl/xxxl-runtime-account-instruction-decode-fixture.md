# XXXL Runtime Account/Instruction Decode Fixture

Status: RUST_DECODE_FIXTURE_ONLY_NOT_DEPLOYABLE.

This stage hardens the native Rust/SVM decode boundary before SPL Token CPI.

## Purpose

The previous stages fixed the production byte layout, serialized runtime vectors, Rust/SVM scaffold, and real PDA derivation fixture. This stage makes the Rust side parse and reject malformed instruction/account bytes before any future CPI boundary can be reached.

## Instruction decode

The Rust `consume_gateway_mint` instruction decoder now fixes:

- total instruction size: 208 bytes
- discriminator: `f2f4a868bb89fe52`
- version: `1`
- account meta count: `9`
- route account index: `1`
- guardian set account index: `2`
- mint state account index: `0`
- processed event account index: `3`
- recipient balance account index: `4`

The decoder parses:

- route id
- guardian set id
- mint id
- canonical event key
- recipient
- amount
- source-chain weight in basis points

Invalid instruction length, discriminator, version, account meta count, or account index fails before handler continuation.

## Account decode

The Rust account views now check exact byte length, discriminator, and layout version for:

- Mint State: 176 bytes
- Gateway Config: 256 bytes
- Guardian Set: 320 bytes
- Processed Event: 144 bytes
- Recipient Balance: 144 bytes

The views expose small field readers only for fixture-critical fields. They do not yet perform owner checks, rent checks, ATA validation, mutation, CPI, or deployment logic.

## Negative coverage

Rust tests cover:

- wrong instruction length
- wrong instruction discriminator
- wrong instruction version
- wrong account meta count
- wrong account index
- wrong account discriminator
- wrong account version
- truncated account data

## Non-goals

This stage intentionally does not add:

- SPL Token `mint_to` CPI
- real initialized Mint handling
- recipient ATA validation
- deployment
- route activation
- authority freeze execution

## Next stage

The likely next stage is `stage-xxxl-spl-token-mint-to-cpi-fixture`.

That stage should use a real initialized Mint account, a real initialized recipient ATA matching the mint, PDA signer through `invoke_signed`, the real bump from `find_program_address`, owner checks, rent checks, and still no live route activation.
