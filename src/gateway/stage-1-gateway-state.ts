import {
  createStage1MintCoreState,
  type Stage1MintCoreState,
} from "./stage-1-mint-core.js";
import {
  createStage1ProcessedBurnRegistry,
  type Stage1ProcessedBurnRegistry,
} from "./stage-1-processed-burn-registry.js";
import { type Stage1GuardianQuorumConfig } from "./stage-1-guardian-quorum.js";
export type Stage1GatewayRouteConfig = {
  sourceToken: string;
  targetX1NetworkId: string;
  targetMintCoreId: string;
};

export type Stage1GatewayState = {
  routeConfig: Stage1GatewayRouteConfig;
  guardianQuorum: Stage1GuardianQuorumConfig;
  processedBurnRegistry: Stage1ProcessedBurnRegistry;
  mintCoreState: Stage1MintCoreState;
};

export type CreateStage1GatewayStateInput = {
  routeConfig: Stage1GatewayRouteConfig;
  guardianQuorum: Stage1GuardianQuorumConfig;
  processedCanonicalEventKeyHexes?: string[];
  initialBalancesByX1Recipient?: Record<string, bigint>;
  totalMinted?: bigint;
};

export function createStage1GatewayState(
  input: CreateStage1GatewayStateInput,
): Stage1GatewayState {
  return {
    routeConfig: input.routeConfig,
    guardianQuorum: input.guardianQuorum,
    processedBurnRegistry: createStage1ProcessedBurnRegistry(
      input.processedCanonicalEventKeyHexes ?? [],
    ),
    mintCoreState: createStage1MintCoreState(
      input.initialBalancesByX1Recipient ?? {},
      input.totalMinted ?? 0n,
    ),
  };
}
