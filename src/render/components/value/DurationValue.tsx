import { InvalidValue, UnknownValue } from "./base";

function durationToString(duration: bigint): string {
  const s = duration % 60n;
  const m = (duration / 60n) % 60n;
  const h = (duration / 3600n) % 24n;
  const d = duration / 86400n;

  if (duration < 60n) {
    return `${s} Seconds`;
  }
  if (duration < 3600n) {
    return `${m} Minutes ${s} Seconds`;
  }
  if (duration < 86400n) {
    return `${h} Hours ${m} Minutes ${s} Seconds`;
  }
  return `${d} Days ${h} Hours ${m} Minutes ${s} Seconds`;
}

export default function DurationValue({ value }: { value: bigint | null }) {
  if (value === null) {
    return <UnknownValue />;
  }
  if (value < 0) {
    return <InvalidValue />;
  }
  return <>{durationToString(value)}</>;
}
