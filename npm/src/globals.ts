import { EntityMetadata } from './registry';
import { fromJson } from '@bufbuild/protobuf';
import { AssetIdSchema } from '@penumbra-zone/protobuf/penumbra/core/asset/v1/asset_pb';
import type { AssetId } from '@penumbra-zone/protobuf/penumbra/core/asset/v1/asset_pb';
import { JsonGlobals } from './json';
import { sha256Hash } from './utils/sha256';

export class RegistryGlobals {
  readonly stakingAssetId: AssetId;
  readonly rpcs: EntityMetadata[];
  readonly frontends: EntityMetadata[];

  constructor(json: JsonGlobals) {
    this.rpcs = json.rpcs;
    this.frontends = json.frontendsV2;
    this.stakingAssetId = fromJson(AssetIdSchema, json.stakingAssetId, {
      ignoreUnknownFields: true,
    });
  }

  async version(): Promise<string> {
    return sha256Hash(JSON.stringify(this));
  }
}
