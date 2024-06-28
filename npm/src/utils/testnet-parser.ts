// Test preview chain ids end with -x followed by a hexadecimal string
export const isTestnetPreviewChainId = (chainId: string): boolean => {
  const previewRegex = /-x[0-9a-f]+$/i;
  return previewRegex.test(chainId);
};

export const deriveTestnetChainIdFromPreview = (previewChainId: string): string => {
  const index = previewChainId.lastIndexOf('-x');
  return previewChainId.substring(0, index);
};
