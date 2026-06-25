# Gateway profile preview status examples

This document describes the three user-facing preview statuses produced by the gateway profile preview flow.

The examples are fixture-based. They do not use live RPC and do not mutate protocol state.

## CREATE_BUILD

Meaning: the Build does not exist yet, but the scanned ETH/XC profile satisfies the minimum requirements.

Expected user meaning:

- Build can be created
- Core redeem history is present
- XNTD lock requirement is satisfied

Example command:

    npm run cli -- gateway:preview:fixture --file docs/gateway/gateway-profile-scan-fixture-create-build-example.json

Expected action:

    "action": "CREATE_BUILD"

## UPDATE_BUILD

Meaning: the Build already exists, and the scanned ETH/XC profile can update the Build state.

Expected user meaning:

- Build already exists
- New or full profile evidence can be applied
- The action is update, not activate

Example command:

    npm run cli -- gateway:preview:fixture --file docs/gateway/gateway-profile-scan-fixture-update-build-example.json

Expected action:

    "action": "UPDATE_BUILD"

## UNAVAILABLE

Meaning: the preview is not eligible for Build creation or update yet.

Expected user meaning:

- one or more requirements are missing
- missingRequirements explains what is missing
- this is not a permanent rejection

Example command:

    npm run cli -- gateway:preview:fixture --file docs/gateway/gateway-profile-scan-fixture-unavailable-example.json

Expected action:

    "action": "UNAVAILABLE"
