export enum BuildErrorCode {
  NotImplemented = "NOT_IMPLEMENTED",
  DuplicateBuildOwner = "DUPLICATE_BUILD_OWNER",
  DuplicateEthereumIdentity = "DUPLICATE_ETHEREUM_IDENTITY",
  DuplicateBuildId = "DUPLICATE_BUILD_ID",
  UnauthorizedRegistrar = "UNAUTHORIZED_REGISTRAR",
  DuplicateRegistrarMessage = "DUPLICATE_REGISTRAR_MESSAGE",
  InvalidBldAmount = "INVALID_BLD_AMOUNT",
  DuplicateRedeemEvent = "DUPLICATE_REDEEM_EVENT",
  InvalidRegistrarMessageKind = "INVALID_REGISTRAR_MESSAGE_KIND",
  InvalidXbpAmount = "INVALID_XBP_AMOUNT",
  DuplicateXenBurnEvent = "DUPLICATE_XEN_BURN_EVENT",
  GenesisOriginNotEligible = "GENESIS_ORIGIN_NOT_ELIGIBLE",
  InvalidXntdLockAmount = "INVALID_XNTD_LOCK_AMOUNT",
  DuplicateXntdCommitmentEvent = "DUPLICATE_XNTD_COMMITMENT_EVENT",
  NonIncreasingXntdLockEpoch = "NON_INCREASING_XNTD_LOCK_EPOCH",
  MissingAuthoritativeXcEpochMinimum = "MISSING_AUTHORITATIVE_XC_EPOCH_MINIMUM",
  MismatchedAuthoritativeXcEpochMinimum = "MISMATCHED_AUTHORITATIVE_XC_EPOCH_MINIMUM",
  InvalidXcEpochMinimumRecord = "INVALID_XC_EPOCH_MINIMUM_RECORD",
  XntdCommitmentNotAccepted = "XNTD_COMMITMENT_NOT_ACCEPTED",
  InvalidFeeContributionAmount = "INVALID_FEE_CONTRIBUTION_AMOUNT",
  InvalidFeeContributionTxCount = "INVALID_FEE_CONTRIBUTION_TX_COUNT",
  NonIncreasingFeeCheckpointSlot = "NON_INCREASING_FEE_CHECKPOINT_SLOT",
  UnauthorizedBuildIdentityUpdate = "UNAUTHORIZED_BUILD_IDENTITY_UPDATE",
  InvalidGatewayFullProfileActivation = "INVALID_GATEWAY_FULL_PROFILE_ACTIVATION",
}

export class BuildError extends Error {
  public readonly code: BuildErrorCode;

  constructor(code: BuildErrorCode, message: string) {
    super(message);
    this.name = "BuildError";
    this.code = code;
  }
}
