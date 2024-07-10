import { it, describe, expect, beforeEach, afterAll } from 'vitest';
import fetchMock from 'fetch-mock';
import { RemoteClient } from './remote';
import { REGISTRY_BASE_URL } from './github';
import { ChainRegistryClient } from './client';

const mockRegistryResponse = {
  chainId: 'penumbra-testnet-deimos-8',
  ibcConnections: [
    {
      addressPrefix: 'osmo',
      chainId: 'osmo-test-5',
      channelId: 'channel-9',
      counterpartyChannelId: 'channel-8343',
      displayName: 'Osmosis',
      images: [
        {
          svg: 'https://raw.githubusercontent.com/cosmos/chain-registry/f1348793beb994c6cc0256ed7ebdb48c7aa70003/osmosis/images/osmo.svg',
        },
      ],
    },
  ],
  assetById: {
    'nDjzm+ldIrNMJha1anGMDVxpA5cLCPnUYQ1clmHF1gw=': {
      denomUnits: [
        {
          denom: 'pizza',
        },
      ],
      base: 'pizza',
      display: 'pizza',
      symbol: 'PIZZA',
      penumbraAssetId: {
        inner: 'nDjzm+ldIrNMJha1anGMDVxpA5cLCPnUYQ1clmHF1gw=',
      },
      images: [
        {
          svg: 'https://raw.githubusercontent.com/prax-wallet/registry/main/images/pizza.svg',
        },
      ],
    },
    'o2gZdbhCH70Ry+7iBhkSeHC/PB1LZhgkn7LHC2kEhQc=': {
      denomUnits: [
        {
          denom: 'test_btc',
          exponent: 8,
        },
        {
          denom: 'test_sat',
        },
      ],
      base: 'test_sat',
      display: 'test_btc',
      symbol: 'TestBTC',
      penumbraAssetId: {
        inner: 'o2gZdbhCH70Ry+7iBhkSeHC/PB1LZhgkn7LHC2kEhQc=',
      },
    },
  },
  numeraires: ['nDjzm+ldIrNMJha1anGMDVxpA5cLCPnUYQ1clmHF1gw='],
};

const mockGlobalsResponse = {
  rpcs: ['some-rpc'],
  frontends: ['some-frontend'],
  stakingAssetId: {
    inner: 'KeqcLzNx9qSH5+lcJHBB9KNW+YPrBk5dKzvPMiypahA=',
  },
};

describe('RemoteClient', () => {
  let rClient: RemoteClient;

  beforeEach(() => {
    const client = new ChainRegistryClient();
    rClient = client.remote;
    fetchMock.reset();
  });

  afterAll(() => {
    fetchMock.restore();
  });

  it('should fetch registry remotely and parse it', async () => {
    const chainId = 'test-chain-7';
    const endpoint = `${REGISTRY_BASE_URL}/chains/${chainId}.json`;
    fetchMock.mock(endpoint, {
      status: 200,
      body: mockRegistryResponse,
    });

    const registry = await rClient.get(chainId);

    expect(fetchMock.called(endpoint)).toBe(true);
    expect(registry.chainId).toEqual(mockRegistryResponse.chainId);
    expect(registry.ibcConnections).toEqual(mockRegistryResponse.ibcConnections);
    expect(registry.getAllAssets().length).toEqual(
      Object.keys(mockRegistryResponse.assetById).length,
    );
  });

  it('should throw if there is not a remote version', async () => {
    const chainId = 'test-not-real-3242';
    const endpoint = `${REGISTRY_BASE_URL}/chains/${chainId}.json`;
    fetchMock.mock(endpoint, {
      status: 404,
    });
    await expect(rClient.get(chainId)).rejects.toThrow(`Failed to fetch from: ${endpoint}`);
  });

  it('should fetch globals remotely and parse it', async () => {
    const endpoint = `${REGISTRY_BASE_URL}/globals.json`;
    fetchMock.mock(endpoint, {
      status: 200,
      body: mockGlobalsResponse,
    });

    const registry = await rClient.globals();

    expect(fetchMock.called(endpoint)).toBe(true);
    expect(registry.stakingAssetId.toJson()).toEqual(mockGlobalsResponse.stakingAssetId);
    expect(registry.frontends).toEqual(mockGlobalsResponse.frontends);
    expect(registry.rpcs).toEqual(mockGlobalsResponse.rpcs);
  });

  it('should calculate version hash correctly', async () => {
    const chainId = 'test-chain-7';
    fetchMock.mock(`${REGISTRY_BASE_URL}/chains/${chainId}.json`, {
      status: 200,
      body: mockRegistryResponse,
    });
    fetchMock.mock(`${REGISTRY_BASE_URL}/globals.json`, {
      status: 200,
      body: mockGlobalsResponse,
    });

    const version = await rClient.version(chainId);
    expect(version).toBe('08150a0b5f4dffbbb0a794ad3f6c58d36dc571ce77359c9c7eec3d34cb7a2b15');
  });
});
