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
  DuplicateXenBurnEvent = "DUPLICATE_XEN_BURN_EVENT"
}

export class BuildError extends Error {
  public readonly code: BuildErrorCode;

  constructor(code: BuildErrorCode, message: string) {
    super(message);
    this.name = "BuildError";
    this.code = code;
  }
}
