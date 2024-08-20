import { EntityMetadata } from './registry';
import { AssetId } from '@penumbra-zone/protobuf/penumbra/core/asset/v1/asset_pb';
import { JsonGlobals } from './json';
import { sha256Hash } from './utils/sha256';

export class RegistryGlobals {
  readonly stakingAssetId: AssetId;
  readonly rpcs: EntityMetadata[];
  readonly frontends: EntityMetadata[];

  constructor(json: JsonGlobals) {
    this.rpcs = json.rpcs;
    this.frontends = json.frontendsV2;
    this.stakingAssetId = AssetId.fromJson(json.stakingAssetId);
  }

  async version(): Promise<string> {
    return sha256Hash(JSON.stringify(this));
  }
}
