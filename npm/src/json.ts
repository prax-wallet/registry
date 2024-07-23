import * as Deimos8 from '../../registry/chains/penumbra-testnet-deimos-8.json';
import * as Penumbra1 from '../../registry/chains/penumbra-1.json';
import { Base64AssetId, Chain, Frontend, Rpc } from './registry';

export interface JsonRegistry {
  chainId: string;
  ibcConnections: Chain[];
  assetById: Record<Base64AssetId, JsonMetadata>;
  numeraires: Base64AssetId[];
}

export interface JsonGlobals {
  rpcs: Rpc[];
  websites: Frontend[];
  /** @deprecated use `websites` instead */
  frontends: string[];
  stakingAssetId: { inner: string };
}

export interface JsonMetadata {
  description?: string;
  denomUnits: DenomUnit[];
  base: string;
  display: string;
  name?: string;
  symbol: string;
  penumbraAssetId: PenumbraAssetId;
  images?: Image[];
}

interface DenomUnit {
  denom: string;
  exponent?: number;
}

interface PenumbraAssetId {
  inner: string;
}

interface Image {
  png?: string;
  svg?: string;
}

export const allJsonRegistries: Record<string, JsonRegistry> = {
  'penumbra-testnet-deimos-8': Deimos8,
  'penumbra-1': Penumbra1,
};
