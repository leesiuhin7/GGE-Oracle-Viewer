import { InvalidValue, UnknownValue } from "./base";

export function BoolValue({ value }: { value: bigint | null }) {
  switch (value) {
    case null:
      return <UnknownValue />;
    case 0n:
      return <>False</>;
    case 1n:
      return <>True</>;
    default:
      return <InvalidValue />;
  }
}

export function NumericValue({ value }: { value: bigint | null }) {
  return value === null ? <UnknownValue /> : <>{value.toString()}</>;
}

export function StringValue({ value }: { value: string | null }) {
  return value === null ? <UnknownValue /> : <>{value}</>;
}

export function TimestampValue({ value }: { value: bigint | null }) {
  return value === null ?
      <UnknownValue />
    : <>{new Date(Number(value) * 1000).toLocaleString()}</>;
}

export function NotSupported() {
  return <>Not supported</>;
}
