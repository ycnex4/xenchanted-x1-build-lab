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
