import { Rpc } from './registry';
import { AssetId } from '@buf/penumbra-zone_penumbra.bufbuild_es/penumbra/core/asset/v1/asset_pb';
import { JsonGlobals } from './json';

export interface RegistryGlobals {
  rpcs: Rpc[];
  frontends: string[];
  stakingAssetId: AssetId;
}

export const globalsFromJson = (json: JsonGlobals): RegistryGlobals => {
  return {
    ...json,
    stakingAssetId: AssetId.fromJson(json.stakingAssetId),
  };
};
