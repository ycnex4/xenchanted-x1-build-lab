# XXXL Program v1 Design Boundary Checkpoint

Stage XXXL Program v1 has started as a docs-first and model-first boundary stage.

Authoritative design document:

- `docs/xxxl/xxxl-program-v1-design-boundary.md`

Initial model files:

- `src/xxxl/program-v1.ts`
- `tests/xxxl/program-v1.test.ts`

Main decisions:

- XXXL is the canonical X1-native token.
- XXXL starts in a gateway-only Genesis Phase.
- XXXL minting does not require Build.
- Build activation remains a separate history / identity operation.
- Build must not derive rights from current XXXL balance.
- XXXL must not use Build state as a gateway mint gate.
- There is no manual mint.
- There is no premine.
- There is no founder allocation.
- There is no hidden emission path.
- Temporary upgradeability is allowed only for staged protocol finalization.
- Future upgrades may only add deterministic user-action protocol mechanics.
- Final X1-native emission must be Core / Forge / Stake-like logic, not admin mint.
- After the planned X1-native emission mechanics are complete, upgrade authority must be removed / frozen.

Genesis Phase invariant:

    XXXL total supply = sum(valid accepted gateway mint amounts)

Replay boundary:

- accepted gateway mint marks canonical event key as processed
- replayed canonical event key fails
- failed mint does not mutate supply
- failed mint does not mutate replay state
- runtime mapping must preserve atomic mint + replay mark

Status:

- design boundary started
- deterministic model scaffold added
- no production runtime code
- no deployment logic
- no RPC usage
- no secrets required
