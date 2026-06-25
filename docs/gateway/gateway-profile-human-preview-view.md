# Gateway profile human preview view

The human preview view is a UI-ready layer built on top of the JSON-safe gateway profile preview DTO.

It does not change gateway eligibility logic.

It only converts the DTO into user-facing sections.

## Statuses

READY_TO_CREATE

User meaning:

- Build does not exist yet
- requirements are satisfied
- user can create Build

Primary action label:

    Create Build

READY_TO_UPDATE

User meaning:

- Build already exists
- scanned profile can update the Build state
- user can update Build

Primary action label:

    Update Build

NEEDS_REQUIREMENTS

User meaning:

- Build cannot be created or updated yet
- missing requirements explain what is needed
- this is not a permanent rejection

Primary action label:

    none

## Cards

The human preview exposes UI-ready cards:

- Build status
- Core redeem history
- XEN burn power
- XNTD lock
- Requirements

## Next steps

For ready states, nextSteps contains the available action.

For missing-requirement states, nextSteps contains the unsatisfied requirements.

## CLI output

Stage 6.8 adds a CLI command that returns the human preview directly from a fixture file.

Command:

    npm run cli -- gateway:preview:fixture:human --file docs/gateway/gateway-profile-scan-fixture-update-build-example.json

The command returns the UI-ready human preview structure:

- status
- tone
- title
- summary
- primaryActionLabel
- canProceed
- cards
- nextSteps

## Generated examples

Stage 6.9 adds generated JSON examples for the three human preview states.

Generated files:

- docs/gateway/generated/gateway-human-preview-create-build.json
- docs/gateway/generated/gateway-human-preview-update-build.json
- docs/gateway/generated/gateway-human-preview-unavailable.json

These files are produced from the fixture CLI command and can be used as stable UI examples for the future XC site gateway preview page.
