import * as Deimos6 from '../../registry/chains/penumbra-testnet-deimos-6.json';
import * as Deimos7 from '../../registry/chains/penumbra-testnet-deimos-7.json';
import * as Deimos8 from '../../registry/chains/penumbra-testnet-deimos-8.json';
import * as GlobalsJson from '../../registry/globals.json';

import { Base64AssetId, Chain, Rpc } from './registry';
import { base64ToUint8Array } from './utils/base64';
import { AssetId } from '@buf/penumbra-zone_penumbra.bufbuild_es/penumbra/core/asset/v1/asset_pb';

export interface RegistryGlobals {
  rpcs: Rpc[];
  frontends: string[];
  stakingAssetId: string;
}

export interface JsonRegistry {
  chainId: string;
  ibcConnections: Chain[];
  assetById: Record<Base64AssetId, JsonMetadata>;
  numeraires: Base64AssetId[];
}

interface JsonMetadata {
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
  'penumbra-testnet-deimos-6': Deimos6,
  'penumbra-testnet-deimos-7': Deimos7,
  'penumbra-testnet-deimos-8': Deimos8,
};

export const registryGlobals: RegistryGlobals = {
  ...GlobalsJson,
  stakingAssetId: new AssetId({ inner: base64ToUint8Array(GlobalsJson.stakingAssetId) }),
};
