import { it, describe, expect, beforeEach, afterAll } from 'vitest';
import fetchMock from 'fetch-mock';
import { RemoteClient } from './remote';
import { REGISTRY_BASE_URL } from './github';
import { ChainRegistryClient } from './client';
import * as Deimos8 from '../../registry/chains/penumbra-testnet-deimos-8.json';
import * as GlobalsJson from '../../registry/globals.json';

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
      body: Deimos8,
    });

    const registry = await rClient.get(chainId);

    expect(fetchMock.called(endpoint)).toBe(true);
    expect(registry.chainId).toEqual(Deimos8.chainId);
    expect(registry.ibcConnections).toEqual(Deimos8.ibcConnections);
    expect(registry.getAllAssets().length).toEqual(Object.keys(Deimos8.assetById).length);
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
      body: GlobalsJson,
    });

    const registry = await rClient.globals();

    expect(fetchMock.called(endpoint)).toBe(true);
    expect(registry.stakingAssetId.toJson()).toEqual(GlobalsJson.stakingAssetId);
    expect(registry.frontends).toEqual(GlobalsJson.frontendsV2);
    expect(registry.rpcs).toEqual(GlobalsJson.rpcs);
  });

  describe('getWithBundledBackup', () => {
    it('fetches remote when available', async () => {
      const chainId = 'test-chain-7';
      const endpoint = `${REGISTRY_BASE_URL}/chains/${chainId}.json`;
      fetchMock.mock(endpoint, {
        status: 200,
        body: Deimos8,
      });

      const registry = await rClient.getWithBundledBackup(chainId);

      expect(fetchMock.called(endpoint)).toBe(true);
      expect(registry.chainId).toEqual(Deimos8.chainId);
      expect(registry.ibcConnections).toEqual(Deimos8.ibcConnections);
      expect(registry.getAllAssets().length).toEqual(Object.keys(Deimos8.assetById).length);
    });

    it('fetches bundled when available', async () => {
      const chainId = 'penumbra-testnet-deimos-8';
      const endpoint = `${REGISTRY_BASE_URL}/chains/${chainId}.json`;
      fetchMock.mock(endpoint, {
        status: 404,
      });

      const registry = await rClient.getWithBundledBackup(chainId);

      expect(fetchMock.called(endpoint)).toBe(true);
      expect(registry.chainId).toEqual(Deimos8.chainId);
      expect(registry.ibcConnections).toEqual(Deimos8.ibcConnections);
      expect(registry.getAllAssets().length).toEqual(Object.keys(Deimos8.assetById).length);
    });
  });
});
