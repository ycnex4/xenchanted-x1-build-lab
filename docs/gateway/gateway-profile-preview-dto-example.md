# Gateway Profile Preview DTO Example

This document shows the JSON-safe gateway profile preview DTO shape used by UI/API layers before Build creation or activation.

The preview DTO is display-only.

It is not protocol state, does not create a Build, does not mutate registry state, does not touch replay sets, and does not reserve eligibility.

Only gateway activation stores verified contribution facts, and activation must validate the submitted full-profile bundle again before mutating state.

## Boundary

Gateway Build requires both minimum participation requirements:

- minimum Core redeem history
- minimum XNTD lock for the current XC epoch

Preview may show that the participant is eligible, but eligibility must still be revalidated during activation.

## JSON safety

All bigint-like values are encoded as decimal strings, for example:

- "121"
- "1000"
- "100000000"

This makes the DTO safe for standard JSON.stringify / JSON transport.

## Eligible preview

Example: participant has Core redeem history, XEN.burn history, and the required XNTD lock.

Eligible preview JSON:

{
"preview": {
"buildExists": false,
"owner": "x1-owner",
"buildId": "build-1",
"ethereumIdentity": "0x0000000000000000000000000000000000000001",
"coreRedeemScanCompleted": true,
"xenBurnScanCompleted": true,
"xntdLockScanCompleted": true,
"coreRedeemProofCount": 1,
"xenBurnProofCount": 1,
"hasXntdLockProof": true,
"existingHistoryBld": "0",
"incomingHistoryBld": "121",
"totalPreviewHistoryBld": "121",
"existingHistoryXbp": "0",
"incomingHistoryXbp": "1000",
"totalPreviewHistoryXbp": "1000",
"previewLockedXntd": "100000000",
"previewRequiredXntdLock": "100000000",
"previewLockEpoch": 0,
"hasMinimumCoreRedeemHistory": true,
"hasMinimumXntdLock": true,
"eligible": true,
"missingRequirements": []
},
"action": "CREATE_BUILD",
"canCreateOrActivateBuild": true,
"title": "Build creation preview",
"summary": "Eligible to create Build."
}

## Ineligible preview

Example: scans completed, but the participant has no minimum Core redeem history and no minimum XNTD lock.

Ineligible preview JSON:

{
"preview": {
"buildExists": false,
"owner": "x1-owner",
"buildId": "build-1",
"ethereumIdentity": "0x0000000000000000000000000000000000000001",
"coreRedeemScanCompleted": true,
"xenBurnScanCompleted": true,
"xntdLockScanCompleted": true,
"coreRedeemProofCount": 0,
"xenBurnProofCount": 0,
"hasXntdLockProof": false,
"existingHistoryBld": "0",
"incomingHistoryBld": "0",
"totalPreviewHistoryBld": "0",
"existingHistoryXbp": "0",
"incomingHistoryXbp": "0",
"totalPreviewHistoryXbp": "0",
"previewLockedXntd": "0",
"previewRequiredXntdLock": "0",
"previewLockEpoch": null,
"hasMinimumCoreRedeemHistory": false,
"hasMinimumXntdLock": false,
"eligible": false,
"missingRequirements": [
"MINIMUM_CORE_REDEEM_HISTORY",
"MINIMUM_XNTD_LOCK"
]
},
"action": "UNAVAILABLE",
"canCreateOrActivateBuild": false,
"title": "Build creation preview",
"summary": "Missing requirements: MINIMUM_CORE_REDEEM_HISTORY, MINIMUM_XNTD_LOCK"
}

## UI rule

The UI may render:

- action = CREATE_BUILD -> show Create Build button
- action = ACTIVATE_BUILD -> show Activate Build button
- action = UNAVAILABLE -> disable action and show missing requirements

The UI must not treat preview eligibility as final protocol authorization.
