import { deriveTestnetChainIdFromPreview, isTestnetPreviewChainId } from './utils/testnet-parser';

export const swapIfPreviewChain = (chainId: string): string => {
  if (!isTestnetPreviewChainId(chainId)) return chainId;

  const derivedChainId = deriveTestnetChainIdFromPreview(chainId);
  if (!derivedChainId) {
    throw new Error(`Chain id could not be derived from testnet preview chain: ${chainId}`);
  }
  return derivedChainId;
};
