# XXXL Production Runtime Byte Layout Checkpoint

Stage XXXL Program v1 now has production-oriented runtime byte layout definitions.

New files:

- `src/xxxl/runtime-production-byte-layout.ts`
- `tests/xxxl/runtime-production-byte-layout.test.ts`
- `docs/xxxl/xxxl-production-runtime-byte-layout.md`

Defined account layouts:

- Mint State account: 176 bytes
- Gateway Config account: 256 bytes
- Guardian Set account: 320 bytes
- Processed Event account: 144 bytes
- Recipient Balance account: 144 bytes

Defined instruction layouts:

- Consume Gateway Mint instruction: 208 bytes

Key properties:

- fixed binary little-endian v1 encoding
- 8-byte discriminator first
- u16 version second
- contiguous fields
- explicit offsets
- explicit sizes
- explicit padding
- u128 fields are 16-byte aligned
- total layout sizes are 8-byte aligned
- route-aware fields remain explicit
- mint authority PDA field is explicit
- program upgrade authority and SPL Token mint authority remain separate

Expected validation baseline after this stage:

- TypeScript typecheck: passing
- Tests: 84 files / 644 tests passing
- Build: passing
