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
