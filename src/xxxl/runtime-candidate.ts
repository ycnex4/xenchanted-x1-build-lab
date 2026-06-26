export const XXXL_RUNTIME_CANDIDATE_VERSION = 1;

export const XXXL_RUNTIME_ACCOUNT_KIND = {
  MintState: "MINT_STATE",
  GatewayConfig: "GATEWAY_CONFIG",
  GuardianSet: "GUARDIAN_SET",
  ProcessedEvent: "PROCESSED_EVENT",
  RecipientBalance: "RECIPIENT_BALANCE",
} as const;

export type XXXLRuntimeAccountKind =
  (typeof XXXL_RUNTIME_ACCOUNT_KIND)[keyof typeof XXXL_RUNTIME_ACCOUNT_KIND];

export const XXXL_RUNTIME_INSTRUCTION = {
  ConsumeGatewayMint: "CONSUME_GATEWAY_MINT",
} as const;

export type XXXLRuntimeInstruction =
  (typeof XXXL_RUNTIME_INSTRUCTION)[keyof typeof XXXL_RUNTIME_INSTRUCTION];

export const XXXL_RUNTIME_AUTHORITY_MODE = {
  GatewayOnly: "GATEWAY_ONLY",
  Frozen: "FROZEN",
} as const;

export type XXXLRuntimeAuthorityMode =
  (typeof XXXL_RUNTIME_AUTHORITY_MODE)[keyof typeof XXXL_RUNTIME_AUTHORITY_MODE];

export const XXXL_RUNTIME_ROUTE_STATUS = {
  Active: "ACTIVE",
  Frozen: "FROZEN",
} as const;

export type XXXLRuntimeRouteStatus =
  (typeof XXXL_RUNTIME_ROUTE_STATUS)[keyof typeof XXXL_RUNTIME_ROUTE_STATUS];

export const XXXL_RUNTIME_GUARDIAN_SET_STATUS = {
  Active: "ACTIVE",
  Retired: "RETIRED",
} as const;

export type XXXLRuntimeGuardianSetStatus =
  (typeof XXXL_RUNTIME_GUARDIAN_SET_STATUS)[keyof typeof XXXL_RUNTIME_GUARDIAN_SET_STATUS];

export const XXXL_RUNTIME_SCHEMA_ERROR = {
  WrongAccountKind: "WRONG_ACCOUNT_KIND",
  WrongMintAuthorityMode: "WRONG_MINT_AUTHORITY_MODE",
  WrongRouteStatus: "WRONG_ROUTE_STATUS",
  WrongGuardianSetStatus: "WRONG_GUARDIAN_SET_STATUS",
  WrongInstruction: "WRONG_INSTRUCTION",
  RouteMismatch: "ROUTE_MISMATCH",
  GuardianSetMismatch: "GUARDIAN_SET_MISMATCH",
  MintMismatch: "MINT_MISMATCH",
  RecipientMismatch: "RECIPIENT_MISMATCH",
  ProcessedEventMismatch: "PROCESSED_EVENT_MISMATCH",
  InvalidQuorumThreshold: "INVALID_QUORUM_THRESHOLD",
  EmptyGuardianSet: "EMPTY_GUARDIAN_SET",
} as const;

export type XXXLRuntimeSchemaErrorCode =
  (typeof XXXL_RUNTIME_SCHEMA_ERROR)[keyof typeof XXXL_RUNTIME_SCHEMA_ERROR];

export type XXXLRuntimeValidationResult = {
  readonly ok: boolean;
  readonly errors: XXXLRuntimeSchemaErrorCode[];
};

export type XXXLMintStateAccount = {
  readonly kind: typeof XXXL_RUNTIME_ACCOUNT_KIND.MintState;
  readonly version: number;
  readonly mintId: string;
  readonly decimals: number;
  readonly totalSupply: bigint;
  readonly authorityMode: XXXLRuntimeAuthorityMode;
  readonly upgradeAuthorityStatus: string;
};

export type XXXLGatewayConfigAccount = {
  readonly kind: typeof XXXL_RUNTIME_ACCOUNT_KIND.GatewayConfig;
  readonly version: number;
  readonly routeId: string;
  readonly sourceChainId: bigint;
  readonly sourceToken: string;
  readonly targetMintToken: string;
  readonly targetX1NetworkId: string;
  readonly targetMintCoreId: string;
  readonly guardianSetId: string;
  readonly quorumThreshold: number;
  readonly finalityRuleId: string;
  readonly status: XXXLRuntimeRouteStatus;
};

export type XXXLGuardianSetAccount = {
  readonly kind: typeof XXXL_RUNTIME_ACCOUNT_KIND.GuardianSet;
  readonly version: number;
  readonly guardianSetId: string;
  readonly guardianPublicKeys: readonly string[];
  readonly quorumThreshold: number;
  readonly status: XXXLRuntimeGuardianSetStatus;
};

export type XXXLProcessedEventAccount = {
  readonly kind: typeof XXXL_RUNTIME_ACCOUNT_KIND.ProcessedEvent;
  readonly version: number;
  readonly canonicalEventKey: string;
  readonly routeId: string;
  readonly consumed: boolean;
  readonly consumedAmount: bigint;
  readonly recipient: string;
};

export type XXXLRecipientBalanceAccount = {
  readonly kind: typeof XXXL_RUNTIME_ACCOUNT_KIND.RecipientBalance;
  readonly version: number;
  readonly mintId: string;
  readonly owner: string;
  readonly balance: bigint;
};

export type XXXLConsumeGatewayMintInstructionAccounts = {
  readonly mintState: XXXLMintStateAccount;
  readonly gatewayConfig: XXXLGatewayConfigAccount;
  readonly guardianSet: XXXLGuardianSetAccount;
  readonly processedEvent: XXXLProcessedEventAccount;
  readonly recipientBalance: XXXLRecipientBalanceAccount;
};

export type XXXLConsumeGatewayMintInstructionData = {
  readonly instruction: typeof XXXL_RUNTIME_INSTRUCTION.ConsumeGatewayMint;
  readonly routeId: string;
  readonly guardianSetId: string;
  readonly mintId: string;
  readonly canonicalEventKey: string;
  readonly recipient: string;
  readonly amount: bigint;
};

export type XXXLConsumeGatewayMintInstructionSchema = {
  readonly accounts: XXXLConsumeGatewayMintInstructionAccounts;
  readonly data: XXXLConsumeGatewayMintInstructionData;
};

export function validateXXXLRuntimeAccountLayout(
  accounts: XXXLConsumeGatewayMintInstructionAccounts,
): XXXLRuntimeValidationResult {
  const errors: XXXLRuntimeSchemaErrorCode[] = [];

  if (accounts.mintState.kind !== XXXL_RUNTIME_ACCOUNT_KIND.MintState) {
    errors.push(XXXL_RUNTIME_SCHEMA_ERROR.WrongAccountKind);
  }

  if (accounts.gatewayConfig.kind !== XXXL_RUNTIME_ACCOUNT_KIND.GatewayConfig) {
    errors.push(XXXL_RUNTIME_SCHEMA_ERROR.WrongAccountKind);
  }

  if (accounts.guardianSet.kind !== XXXL_RUNTIME_ACCOUNT_KIND.GuardianSet) {
    errors.push(XXXL_RUNTIME_SCHEMA_ERROR.WrongAccountKind);
  }

  if (accounts.processedEvent.kind !== XXXL_RUNTIME_ACCOUNT_KIND.ProcessedEvent) {
    errors.push(XXXL_RUNTIME_SCHEMA_ERROR.WrongAccountKind);
  }

  if (accounts.recipientBalance.kind !== XXXL_RUNTIME_ACCOUNT_KIND.RecipientBalance) {
    errors.push(XXXL_RUNTIME_SCHEMA_ERROR.WrongAccountKind);
  }

  if (accounts.mintState.authorityMode !== XXXL_RUNTIME_AUTHORITY_MODE.GatewayOnly) {
    errors.push(XXXL_RUNTIME_SCHEMA_ERROR.WrongMintAuthorityMode);
  }

  if (accounts.gatewayConfig.status !== XXXL_RUNTIME_ROUTE_STATUS.Active) {
    errors.push(XXXL_RUNTIME_SCHEMA_ERROR.WrongRouteStatus);
  }

  if (accounts.guardianSet.status !== XXXL_RUNTIME_GUARDIAN_SET_STATUS.Active) {
    errors.push(XXXL_RUNTIME_SCHEMA_ERROR.WrongGuardianSetStatus);
  }

  if (accounts.gatewayConfig.guardianSetId !== accounts.guardianSet.guardianSetId) {
    errors.push(XXXL_RUNTIME_SCHEMA_ERROR.GuardianSetMismatch);
  }

  if (accounts.gatewayConfig.quorumThreshold !== accounts.guardianSet.quorumThreshold) {
    errors.push(XXXL_RUNTIME_SCHEMA_ERROR.GuardianSetMismatch);
  }

  if (accounts.guardianSet.guardianPublicKeys.length === 0) {
    errors.push(XXXL_RUNTIME_SCHEMA_ERROR.EmptyGuardianSet);
  }

  if (
    accounts.guardianSet.quorumThreshold <= 0 ||
    accounts.guardianSet.quorumThreshold > accounts.guardianSet.guardianPublicKeys.length
  ) {
    errors.push(XXXL_RUNTIME_SCHEMA_ERROR.InvalidQuorumThreshold);
  }

  if (accounts.recipientBalance.mintId !== accounts.mintState.mintId) {
    errors.push(XXXL_RUNTIME_SCHEMA_ERROR.MintMismatch);
  }

  if (accounts.processedEvent.routeId !== accounts.gatewayConfig.routeId) {
    errors.push(XXXL_RUNTIME_SCHEMA_ERROR.RouteMismatch);
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

export function validateXXXLConsumeGatewayMintInstructionSchema(
  schema: XXXLConsumeGatewayMintInstructionSchema,
): XXXLRuntimeValidationResult {
  const accountLayout = validateXXXLRuntimeAccountLayout(schema.accounts);
  const errors: XXXLRuntimeSchemaErrorCode[] = [...accountLayout.errors];
  const { accounts, data } = schema;

  if (data.instruction !== XXXL_RUNTIME_INSTRUCTION.ConsumeGatewayMint) {
    errors.push(XXXL_RUNTIME_SCHEMA_ERROR.WrongInstruction);
  }

  if (data.routeId !== accounts.gatewayConfig.routeId) {
    errors.push(XXXL_RUNTIME_SCHEMA_ERROR.RouteMismatch);
  }

  if (data.guardianSetId !== accounts.guardianSet.guardianSetId) {
    errors.push(XXXL_RUNTIME_SCHEMA_ERROR.GuardianSetMismatch);
  }

  if (data.mintId !== accounts.mintState.mintId) {
    errors.push(XXXL_RUNTIME_SCHEMA_ERROR.MintMismatch);
  }

  if (data.canonicalEventKey !== accounts.processedEvent.canonicalEventKey) {
    errors.push(XXXL_RUNTIME_SCHEMA_ERROR.ProcessedEventMismatch);
  }

  if (data.recipient !== accounts.recipientBalance.owner) {
    errors.push(XXXL_RUNTIME_SCHEMA_ERROR.RecipientMismatch);
  }

  if (data.recipient !== accounts.processedEvent.recipient) {
    errors.push(XXXL_RUNTIME_SCHEMA_ERROR.RecipientMismatch);
  }

  if (data.amount !== accounts.processedEvent.consumedAmount) {
    errors.push(XXXL_RUNTIME_SCHEMA_ERROR.ProcessedEventMismatch);
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

export function xxxlRuntimeAccountWriteSet(
  schema: XXXLConsumeGatewayMintInstructionSchema,
): readonly XXXLRuntimeAccountKind[] {
  validateXXXLConsumeGatewayMintInstructionSchema(schema);

  return [
    XXXL_RUNTIME_ACCOUNT_KIND.MintState,
    XXXL_RUNTIME_ACCOUNT_KIND.ProcessedEvent,
    XXXL_RUNTIME_ACCOUNT_KIND.RecipientBalance,
  ];
}
