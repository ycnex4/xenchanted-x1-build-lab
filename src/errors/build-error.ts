export enum BuildErrorCode {
  NotImplemented = "NOT_IMPLEMENTED"
}

export class BuildError extends Error {
  public readonly code: BuildErrorCode;

  constructor(code: BuildErrorCode, message: string) {
    super(message);
    this.name = "BuildError";
    this.code = code;
  }
}
