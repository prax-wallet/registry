import { it, describe, expect, beforeEach, afterAll } from 'vitest';
import fetchMock from 'fetch-mock';
import { RemoteClient } from './remote';
import { REGISTRY_BASE_URL } from './github';
import { ChainRegistryClient } from './client';
import * as Phobos1 from '../../registry/chains/penumbra-testnet-phobos-1.json';
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
      body: Phobos1,
    });

    const registry = await rClient.get(chainId);

    expect(fetchMock.called(endpoint)).toBe(true);
    expect(registry.chainId).toEqual(Phobos1.chainId);
    expect(registry.ibcConnections).toEqual(Phobos1.ibcConnections);
    expect(registry.getAllAssets().length).toEqual(Object.keys(Phobos1.assetById).length);
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

  describe('testnet preview', () => {
    it('fetches falls back if available', async () => {
      const testnetPreviewChainId = 'penumbra-testnet-phobos-1-x6de97e39';
      const firstCall = `${REGISTRY_BASE_URL}/chains/${testnetPreviewChainId}.json`;
      fetchMock.mock(firstCall, {
        status: 404,
      });
      const fallbackChainId = 'penumbra-testnet-phobos-1';
      const secondCall = `${REGISTRY_BASE_URL}/chains/${fallbackChainId}.json`;
      fetchMock.mock(secondCall, {
        status: 200,
        body: Phobos1,
      });

      const registry = await rClient.get(testnetPreviewChainId);

      expect(fetchMock.called(firstCall)).toBe(true);
      expect(fetchMock.called(secondCall)).toBe(true);
      expect(registry.chainId).toEqual(Phobos1.chainId);
      expect(registry.ibcConnections).toEqual(Phobos1.ibcConnections);
      expect(registry.getAllAssets().length).toEqual(Object.keys(Phobos1.assetById).length);
    });

    it('throws if falls back not available', async () => {
      const testnetPreviewChainId = 'penumbra-testnet-phobos-1-x6de97e39';
      const firstCall = `${REGISTRY_BASE_URL}/chains/${testnetPreviewChainId}.json`;
      fetchMock.mock(firstCall, {
        status: 404,
      });
      const fallbackChainId = 'penumbra-testnet-phobos-1';
      const secondCall = `${REGISTRY_BASE_URL}/chains/${fallbackChainId}.json`;
      fetchMock.mock(secondCall, {
        status: 404,
      });

      await expect(rClient.get(testnetPreviewChainId)).rejects.toThrow(
        `Failed to fetch from: ${secondCall}`,
      );

      expect(fetchMock.called(firstCall)).toBe(true);
      expect(fetchMock.called(secondCall)).toBe(true);
    });
  });

  describe('getWithBundledBackup', () => {
    it('fetches remote when available', async () => {
      const chainId = 'test-chain-7';
      const endpoint = `${REGISTRY_BASE_URL}/chains/${chainId}.json`;
      fetchMock.mock(endpoint, {
        status: 200,
        body: Phobos1,
      });

      const registry = await rClient.getWithBundledBackup(chainId);

      expect(fetchMock.called(endpoint)).toBe(true);
      expect(registry.chainId).toEqual(Phobos1.chainId);
      expect(registry.ibcConnections).toEqual(Phobos1.ibcConnections);
      expect(registry.getAllAssets().length).toEqual(Object.keys(Phobos1.assetById).length);
    });

    it('fetches bundled when available', async () => {
      const chainId = 'penumbra-testnet-phobos-1';
      const endpoint = `${REGISTRY_BASE_URL}/chains/${chainId}.json`;
      fetchMock.mock(endpoint, {
        status: 404,
      });

      const registry = await rClient.getWithBundledBackup(chainId);

      expect(fetchMock.called(endpoint)).toBe(true);
      expect(registry.chainId).toEqual(Phobos1.chainId);
      expect(registry.ibcConnections).toEqual(Phobos1.ibcConnections);
      expect(registry.getAllAssets().length).toEqual(Object.keys(Phobos1.assetById).length);
    });
  });
});
