const TESTNET_PREVIEW_PATTERN = /^(penumbra-testnet-(?:\w+-)*\w+)-[a-f0-9]{8}$/;

export const isTestnetPreviewChainId = (chainId: string) => {
  return TESTNET_PREVIEW_PATTERN.test(chainId);
};

export const deriveTestnetChainIdFromPreview = (previewChainId: string): string | undefined => {
  const match = previewChainId.match(TESTNET_PREVIEW_PATTERN);
  if (match) {
    return match[1];
  }
  return undefined;
};
