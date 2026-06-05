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
  GenesisOriginAlreadyClaimed = "GENESIS_ORIGIN_ALREADY_CLAIMED",
  GenesisOriginNotEligible = "GENESIS_ORIGIN_NOT_ELIGIBLE",
  InvalidXntdLockAmount = "INVALID_XNTD_LOCK_AMOUNT",
  DuplicateXntdCommitmentEvent = "DUPLICATE_XNTD_COMMITMENT_EVENT",
  NonIncreasingXntdLockEpoch = "NON_INCREASING_XNTD_LOCK_EPOCH",
  MissingAuthoritativeXcEpochMinimum = "MISSING_AUTHORITATIVE_XC_EPOCH_MINIMUM",
  MismatchedAuthoritativeXcEpochMinimum = "MISMATCHED_AUTHORITATIVE_XC_EPOCH_MINIMUM",
  XntdCommitmentNotActive = "XNTD_COMMITMENT_NOT_ACTIVE",
  InsufficientAvailableBldForRelock = "INSUFFICIENT_AVAILABLE_BLD_FOR_RELOCK",
  InvalidFeeContributionAmount = "INVALID_FEE_CONTRIBUTION_AMOUNT",
  InvalidFeeContributionTxCount = "INVALID_FEE_CONTRIBUTION_TX_COUNT",
  NonIncreasingFeeCheckpointSlot = "NON_INCREASING_FEE_CHECKPOINT_SLOT"
}

export class BuildError extends Error {
  public readonly code: BuildErrorCode;

  constructor(code: BuildErrorCode, message: string) {
    super(message);
    this.name = "BuildError";
    this.code = code;
  }
}
