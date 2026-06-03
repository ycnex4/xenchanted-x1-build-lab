export enum BuildErrorCode {
  NotImplemented = "NOT_IMPLEMENTED",
  DuplicateBuildOwner = "DUPLICATE_BUILD_OWNER",
  DuplicateEthereumIdentity = "DUPLICATE_ETHEREUM_IDENTITY",
  DuplicateBuildId = "DUPLICATE_BUILD_ID"
}

export class BuildError extends Error {
  public readonly code: BuildErrorCode;

  constructor(code: BuildErrorCode, message: string) {
    super(message);
    this.name = "BuildError";
    this.code = code;
  }
}
