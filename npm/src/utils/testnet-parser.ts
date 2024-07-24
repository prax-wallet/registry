const testnetChainIdPattern = /-x?[0-9a-f]{8}$/;

/**
 * Preview chain ids end with '-x' followed by a hexadecimal string of length 8
 * Local chain ids end with a '-' followed by a hexadecimal string of length 8
 */
export const isTestnetPreviewChainId = (chainId: string): boolean => {
  return testnetChainIdPattern.test(chainId);
};

export const deriveTestnetChainIdFromPreview = (previewChainId: string): string => {
  return previewChainId.substring(0, previewChainId.search(testnetChainIdPattern));
};
