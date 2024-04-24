import { JsonValue } from '@bufbuild/protobuf';

// @ts-expect-error alias for dev only
// eslint-disable-next-line @typescript-eslint/no-unused-vars
export type Jsonified<T> = JsonValue;

// @ts-expect-error alias for dev only
// eslint-disable-next-line @typescript-eslint/no-unused-vars
export type Stringified<T> = string;

type MapFunction<T, U> = (value: T) => U;

export const mapObjectValues = <T, U>(
  obj: Record<string, T>,
  mapFn: MapFunction<T, U>,
): Record<string, U> => {
  const mappedEntries = Object.entries(obj).map(([key, value]): [string, U] => [key, mapFn(value)]);
  return Object.fromEntries(mappedEntries);
};

export const uint8ArrayToBase64 = (byteArray: Uint8Array): string => {
  const binString = String.fromCodePoint(...byteArray);
  return btoa(binString);
};

export const base64ToUint8Array = (base64: string): Uint8Array => {
  const base64Regex = /^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?$/;

  if (!base64Regex.test(base64)) {
    throw new Error('Input string is not a valid Base64 encoded string');
  }

  const binString = atob(base64);
  return Uint8Array.from(binString, byte => byte.codePointAt(0)!);
};

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
