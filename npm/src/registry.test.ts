import { describe, expect, it } from 'vitest';
import { base64ToUint8Array } from './utils';
import { Registry } from './registry';
import { GithubRegistryResponse } from './github';
import { AssetId } from '@buf/penumbra-zone_penumbra.bufbuild_es/penumbra/core/asset/v1/asset_pb';

const testRegistry: GithubRegistryResponse = {
  chainId: 'penumbra-testnet-deimos-6',
  ibcConnections: [
    {
      addressPrefix: 'osmo',
      chainId: 'osmo-test-5',
      channelId: 'channel-4',
      counterpartyChannelId: 'channel-7780',
      displayName: 'Osmosis',
      images: [
        {
          svg: 'https://raw.githubusercontent.com/cosmos/chain-registry/f1348793beb994c6cc0256ed7ebdb48c7aa70003/osmosis/images/osmo.svg',
        },
      ],
    },
  ],
  rpcs: [
    {
      name: 'Penumbra Labs Testnet RPC',
      url: 'https://grpc.testnet.penumbra.zone',
      images: [
        {
          png: 'https://raw.githubusercontent.com/prax-wallet/registry/main/images/penumbra-favicon.png',
        },
      ],
    },
  ],
  assetById: {
    'KeqcLzNx9qSH5+lcJHBB9KNW+YPrBk5dKzvPMiypahA=': {
      denomUnits: [
        {
          denom: 'penumbra',
          exponent: 6,
        },
        {
          denom: 'mpenumbra',
          exponent: 3,
        },
        {
          denom: 'upenumbra',
        },
      ],
      base: 'upenumbra',
      display: 'penumbra',
      symbol: 'UM',
      penumbraAssetId: {
        inner: 'KeqcLzNx9qSH5+lcJHBB9KNW+YPrBk5dKzvPMiypahA=',
      },
      images: [
        {
          svg: 'https://raw.githubusercontent.com/prax-wallet/registry/main/images/um.svg',
        },
      ],
    },
    'reum7wQmk/owgvGMWMZn/6RFPV24zIKq3W6In/WwZgg=': {
      denomUnits: [
        {
          denom: 'test_usd',
          exponent: 18,
        },
        {
          denom: 'wtest_usd',
        },
      ],
      base: 'wtest_usd',
      display: 'test_usd',
      symbol: 'TestUSD',
      penumbraAssetId: {
        inner: 'reum7wQmk/owgvGMWMZn/6RFPV24zIKq3W6In/WwZgg=',
      },
      images: [
        {
          svg: 'https://raw.githubusercontent.com/prax-wallet/registry/main/images/test-usd.svg',
        },
      ],
    },
  },
  stakingAssetId: 'KeqcLzNx9qSH5+lcJHBB9KNW+YPrBk5dKzvPMiypahA=',
  numeraires: ['reum7wQmk/owgvGMWMZn/6RFPV24zIKq3W6In/WwZgg='],
};

describe('Registry', () => {
  it('get metadata successfully', () => {
    const registry = new Registry(testRegistry);
    const usdcId = base64ToUint8Array('reum7wQmk/owgvGMWMZn/6RFPV24zIKq3W6In/WwZgg=');
    const res = registry.getMetadata(new AssetId({ inner: usdcId }));
    expect(res.base).toEqual('wtest_usd');
  });

  it('throw when searching for metadata that does not exist', () => {
    const registry = new Registry(testRegistry);
    const cubeId = base64ToUint8Array('6KBVsPINa8gWSHhfH+kAFJC4afEJA3EtuB2HyCqJUws=');
    const getCubeMetadata = () => registry.getMetadata(new AssetId({ inner: cubeId }));
    expect(getCubeMetadata).toThrow();
  });

  it('get all assets successfully', () => {
    const registry = new Registry(testRegistry);
    const res = registry.getAllAssets();
    expect(res.length).toEqual(2);
  });
});
