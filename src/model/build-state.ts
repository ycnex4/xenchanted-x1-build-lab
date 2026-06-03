export type BuildId = string;
export type X1Address = string;
export type EthereumAddress = string;

export interface BuildState {
  owner: X1Address;
  buildId: BuildId;
  version: number;
  createdAt: bigint;
  updatedAt: bigint;
}
