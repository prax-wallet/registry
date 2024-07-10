// Function to convert a byte to a hexadecimal string
const byteToHex = (byte: number): string => byte.toString(16).padStart(2, '0');

const sha256BuffHash = async (inputBuffer: Uint8Array): Promise<Uint8Array> => {
  const digestBuffer = await crypto.subtle.digest('SHA-256', inputBuffer);
  return new Uint8Array(digestBuffer);
};

export const sha256Hash = async (inputString: string): Promise<string> => {
  const encoder = new TextEncoder();
  const encodedString = encoder.encode(inputString);
  const uint8Arr = await sha256BuffHash(encodedString);
  return Array.from(uint8Arr).map(byteToHex).join('');
};
