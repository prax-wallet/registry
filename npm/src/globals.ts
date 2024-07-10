import { Rpc } from './registry';
import { AssetId } from '@buf/penumbra-zone_penumbra.bufbuild_es/penumbra/core/asset/v1/asset_pb';
import { JsonGlobals } from './json';
import { sha256Hash } from './utils/sha256';

export class RegistryGlobals {
  readonly stakingAssetId: AssetId;
  readonly rpcs: Rpc[];
  readonly frontends: string[];

  constructor(json: JsonGlobals) {
    this.rpcs = json.rpcs;
    this.frontends = json.frontends;
    this.stakingAssetId = AssetId.fromJson(json.stakingAssetId);
  }

  async version(): Promise<string> {
    return sha256Hash(JSON.stringify(this));
  }
}
