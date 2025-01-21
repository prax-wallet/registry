import * as Deimos8 from '../../registry/chains/penumbra-testnet-deimos-8-x6de97e39.json';
import * as Penumbra1 from '../../registry/chains/penumbra-1.json';
import * as Phobos1 from '../../registry/chains/penumbra-testnet-phobos-1.json';
import * as Phobos2 from '../../registry/chains/penumbra-testnet-phobos-2.json';
import { Base64AssetId, Chain, EntityMetadata } from './registry';

export interface JsonRegistry {
  chainId: string;
  ibcConnections: Chain[];
  assetById: Record<Base64AssetId, JsonMetadata>;
  numeraires: Base64AssetId[];
}

export interface JsonGlobals {
  rpcs: EntityMetadata[];
  /** @deprecated use `frontendsV2` instead */
  frontends: string[];
  frontendsV2: EntityMetadata[];
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
  badges?: Image[];
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
  theme?: {
    primaryColorHex?: string;
    circle?: boolean;
    darkMode?: boolean;
  };
}

export const allJsonRegistries: Record<string, JsonRegistry> = {
  'penumbra-testnet-deimos-8-x6de97e39': Deimos8,
  'penumbra-1': Penumbra1,
  'penumbra-testnet-phobos-1': Phobos1,
  'penumbra-testnet-phobos-2': Phobos2,
};
