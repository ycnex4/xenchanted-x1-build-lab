# Current Design Checkpoint

Current branch target:

    stage-41k6-b2-valid-quorum-live-gated-success

Base:

    main @ c5e3c38

Closed:

    Phase 41K.6 B1 guardian quorum authorization
    D2 mint bypass fix
    B1 post-closure payload binding hardening

Current phase:

    B2: valid quorum live-gated success test

Current goal:

    Prove the positive gated ConsumeGatewayMint path:
    valid prior Ed25519 evidence
    -> payload v2 match
    -> guardian membership
    -> unique quorum
    -> B1C7 authorization
    -> CPI gate
    -> processed_event mark
    -> SPL mint

Production activation:

    Not part of B2.
    B2 remains test-gated.
    B4 remains the activation decision point.

Primary spec:

    docs/gateway/phase-41k6-b2-valid-quorum-live-gated-success.md
