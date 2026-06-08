import { uint256BeToBigInt } from "./stage-1-verifier.js";
import { bytesToHex, type Stage1GatewayMintMessageFields } from "./stage-1-encoding.js";
import {
  authorizeStage1Mint,
  type Stage1MintAuthorizationInput,
  type Stage1MintAuthorizationResult,
} from "./stage-1-mint-authorization.js";

export const STAGE1_MINT_CORE_ERROR = {
  MintNotAuthorized: "MINT_NOT_AUTHORIZED",
} as const;

export type Stage1MintCoreErrorCode =
  (typeof STAGE1_MINT_CORE_ERROR)[keyof typeof STAGE1_MINT_CORE_ERROR];

export type Stage1MintCoreState = {
  balancesByX1Recipient: Map<string, bigint>;
  totalMinted: bigint;
};

export type Stage1MintCoreResult = {
  ok: boolean;
  minted: boolean;
  recipientHex: string;
  amount: bigint;
  balanceAfter: bigint;
  totalMintedAfter: bigint;
  errors: Stage1MintCoreErrorCode[];
  authorization: Stage1MintAuthorizationResult;
};

export type Stage1MintCoreInput = Stage1MintAuthorizationInput & {
  mintCoreState: Stage1MintCoreState;
};

export function createStage1MintCoreState(
  initialBalances: Record<string, bigint> = {},
  totalMinted = 0n,
): Stage1MintCoreState {
  return {
    balancesByX1Recipient: new Map(
      Object.entries(initialBalances).map(([recipientHex, balance]) => [
        recipientHex.toLowerCase(),
        balance,
      ]),
    ),
    totalMinted,
  };
}

export function stage1X1RecipientHex(x1RecipientBytes: Uint8Array): string {
  return bytesToHex(x1RecipientBytes).toLowerCase();
}

export function stage1MintAmountFromFields(
  fields: Pick<Stage1GatewayMintMessageFields, "xxxlMintAmount">,
): bigint {
  return uint256BeToBigInt(fields.xxxlMintAmount);
}

export async function executeStage1MintCore(
  input: Stage1MintCoreInput,
): Promise<Stage1MintCoreResult> {
  const authorization = await authorizeStage1Mint(input);
  const recipientHex = stage1X1RecipientHex(input.x1RecipientBytes);
  const amount = stage1MintAmountFromFields(input.fields);

  if (!authorization.ok) {
    return {
      ok: false,
      minted: false,
      recipientHex,
      amount,
      balanceAfter: input.mintCoreState.balancesByX1Recipient.get(recipientHex) ?? 0n,
      totalMintedAfter: input.mintCoreState.totalMinted,
      errors: [STAGE1_MINT_CORE_ERROR.MintNotAuthorized],
      authorization,
    };
  }

  const balanceAfter =
    (input.mintCoreState.balancesByX1Recipient.get(recipientHex) ?? 0n) +
    amount;
  const totalMintedAfter = input.mintCoreState.totalMinted + amount;

  input.mintCoreState.balancesByX1Recipient.set(recipientHex, balanceAfter);
  input.mintCoreState.totalMinted = totalMintedAfter;

  return {
    ok: true,
    minted: true,
    recipientHex,
    amount,
    balanceAfter,
    totalMintedAfter,
    errors: [],
    authorization,
  };
}
