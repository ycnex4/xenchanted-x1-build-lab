import type {
  BuildId,
  EthereumAddress,
  X1Address,
} from "../model/build-state.js";
import type { BuildApplicationState } from "./build-service.js";
import {
  type GatewayProfileScanResult,
  type GatewayProfileScanner,
  buildGatewayFullProfileActivationBundleFromScan,
} from "./gateway-profile-scan.js";
import {
  type AppGatewayProfilePreviewDto,
  appGetGatewayProfilePreviewDto,
} from "./gateway-profile-preview-dto.js";

export interface AppGetGatewayProfilePreviewDtoFromScanInput {
  readonly app: BuildApplicationState;
  readonly scanner: GatewayProfileScanner;
  readonly buildId: BuildId;
  readonly owner: X1Address;
  readonly ethereumIdentity: EthereumAddress;
  readonly validatedAt: bigint;
}

export interface AppGatewayProfilePreviewDtoFromScanResult {
  readonly scanResult: GatewayProfileScanResult;
  readonly dto: AppGatewayProfilePreviewDto;
}

export function appGetGatewayProfilePreviewDtoFromScan(
  input: AppGetGatewayProfilePreviewDtoFromScanInput,
): AppGatewayProfilePreviewDtoFromScanResult {
  const scanResult = input.scanner.scan({
    buildId: input.buildId,
    owner: input.owner,
    ethereumIdentity: input.ethereumIdentity,
  });

  const bundle = buildGatewayFullProfileActivationBundleFromScan({
    scanResult,
    validatedAt: input.validatedAt,
  });

  const dto = appGetGatewayProfilePreviewDto({
    app: input.app,
    bundle,
  });

  return {
    scanResult,
    dto,
  };
}
