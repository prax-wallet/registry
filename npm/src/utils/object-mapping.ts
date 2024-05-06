type MapFunction<T, U> = (value: T) => U;

export const mapObjectValues = <T, U>(
  obj: Record<string, T>,
  mapFn: MapFunction<T, U>,
): Record<string, U> => {
  const mappedEntries = Object.entries(obj).map(([key, value]): [string, U] => [key, mapFn(value)]);
  return Object.fromEntries(mappedEntries);
};
