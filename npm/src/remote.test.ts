import { it, describe, expect, beforeEach, afterEach } from 'vitest';
import fetchMock from 'fetch-mock';
import { toJson } from '@bufbuild/protobuf';
import { AssetIdSchema } from '@penumbra-zone/protobuf/penumbra/core/asset/v1/asset_pb';
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
    fetchMock.mockGlobal();
  });

  afterEach(() => {
    fetchMock.removeRoutes();
    fetchMock.clearHistory();
    fetchMock.unmockGlobal();
  });

  it('should fetch registry remotely and parse it', async () => {
    const chainId = 'test-chain-7';
    const endpoint = `${REGISTRY_BASE_URL}/chains/${chainId}.json`;
    fetchMock.get(endpoint, {
      status: 200,
      body: Phobos1,
    });

    const registry = await rClient.get(chainId);

    expect(fetchMock.callHistory.called(endpoint)).toBe(true);
    expect(registry.chainId).toEqual(Phobos1.chainId);
    expect(registry.ibcConnections).toEqual(Phobos1.ibcConnections);
    expect(registry.getAllAssets().length).toEqual(Object.keys(Phobos1.assetById).length);
  });

  it('should throw if there is not a remote version', async () => {
    const chainId = 'test-not-real-3242';
    const endpoint = `${REGISTRY_BASE_URL}/chains/${chainId}.json`;
    fetchMock.get(endpoint, {
      status: 404,
    });
    await expect(rClient.get(chainId)).rejects.toThrow(`Failed to fetch from: ${endpoint}`);
  });

  it('should fetch globals remotely and parse it', async () => {
    const endpoint = `${REGISTRY_BASE_URL}/globals.json`;
    fetchMock.get(endpoint, {
      status: 200,
      body: GlobalsJson,
    });

    const registry = await rClient.globals();

    expect(fetchMock.callHistory.called(endpoint)).toBe(true);
    expect(toJson(AssetIdSchema, registry.stakingAssetId)).toEqual(GlobalsJson.stakingAssetId);
    expect(registry.frontends).toEqual(GlobalsJson.frontendsV2);
    expect(registry.rpcs).toEqual(GlobalsJson.rpcs);
  });

  describe('testnet preview', () => {
    it('fetches falls back if available', async () => {
      const testnetPreviewChainId = 'penumbra-testnet-phobos-1-x6de97e39';
      const firstCall = `${REGISTRY_BASE_URL}/chains/${testnetPreviewChainId}.json`;
      fetchMock.get(firstCall, {
        status: 404,
      });
      const fallbackChainId = 'penumbra-testnet-phobos-1';
      const secondCall = `${REGISTRY_BASE_URL}/chains/${fallbackChainId}.json`;
      fetchMock.get(secondCall, {
        status: 200,
        body: Phobos1,
      });

      const registry = await rClient.get(testnetPreviewChainId);

      expect(fetchMock.callHistory.called(firstCall)).toBe(true);
      expect(fetchMock.callHistory.called(secondCall)).toBe(true);
      expect(registry.chainId).toEqual(Phobos1.chainId);
      expect(registry.ibcConnections).toEqual(Phobos1.ibcConnections);
      expect(registry.getAllAssets().length).toEqual(Object.keys(Phobos1.assetById).length);
    });

    it('throws if falls back not available', async () => {
      const testnetPreviewChainId = 'penumbra-testnet-phobos-1-x6de97e39';
      const firstCall = `${REGISTRY_BASE_URL}/chains/${testnetPreviewChainId}.json`;
      fetchMock.get(firstCall, {
        status: 404,
      });
      const fallbackChainId = 'penumbra-testnet-phobos-1';
      const secondCall = `${REGISTRY_BASE_URL}/chains/${fallbackChainId}.json`;
      fetchMock.get(secondCall, {
        status: 404,
      });

      await expect(rClient.get(testnetPreviewChainId)).rejects.toThrow(
        `Failed to fetch from: ${secondCall}`,
      );

      expect(fetchMock.callHistory.called(firstCall)).toBe(true);
      expect(fetchMock.callHistory.called(secondCall)).toBe(true);
    });
  });

  describe('getWithBundledBackup', () => {
    it('fetches remote when available', async () => {
      const chainId = 'test-chain-7';
      const endpoint = `${REGISTRY_BASE_URL}/chains/${chainId}.json`;
      fetchMock.get(endpoint, {
        status: 200,
        body: Phobos1,
      });

      const registry = await rClient.getWithBundledBackup(chainId);

      expect(fetchMock.callHistory.called(endpoint)).toBe(true);
      expect(registry.chainId).toEqual(Phobos1.chainId);
      expect(registry.ibcConnections).toEqual(Phobos1.ibcConnections);
      expect(registry.getAllAssets().length).toEqual(Object.keys(Phobos1.assetById).length);
    });

    it('fetches bundled when available', async () => {
      const chainId = 'penumbra-testnet-phobos-1';
      const endpoint = `${REGISTRY_BASE_URL}/chains/${chainId}.json`;
      fetchMock.get(endpoint, {
        status: 404,
      });

      const registry = await rClient.getWithBundledBackup(chainId);

      expect(fetchMock.callHistory.called(endpoint)).toBe(true);
      expect(registry.chainId).toEqual(Phobos1.chainId);
      expect(registry.ibcConnections).toEqual(Phobos1.ibcConnections);
      expect(registry.getAllAssets().length).toEqual(Object.keys(Phobos1.assetById).length);
    });
  });
});
