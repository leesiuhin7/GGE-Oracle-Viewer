import DeputyImg from "../../../assets/ranks/deputy.png";
import DiplomatImg from "../../../assets/ranks/diplomat.png";
import GeneralImg from "../../../assets/ranks/general.png";
import LeaderImg from "../../../assets/ranks/leader.png";
import MemberImg from "../../../assets/ranks/member.png";
import NoviceImg from "../../../assets/ranks/novice.png";
import RecruiterImg from "../../../assets/ranks/recruiter.png";
import SergeantImg from "../../../assets/ranks/sergeant.png";
import TreasurerImg from "../../../assets/ranks/treasurer.png";
import WarMarshalImg from "../../../assets/ranks/war_marshal.png";
import { InvalidValue } from "./base";

function Rank({ name, src }: { name: string; src: string }) {
  return (
    <span style={{ display: "flex", gap: 5, alignItems: "center" }}>
      <img src={src} height="16px"></img>
      {name}
    </span>
  );
}

export default function RankValue({ value }: { value: bigint | null }) {
  switch (value) {
    case 0n:
      return <Rank name="Leader" src={LeaderImg} />;
    case 1n:
      return <Rank name="Deputy" src={DeputyImg} />;
    case 2n:
      return <Rank name="War marshal" src={WarMarshalImg} />;
    case 3n:
      return <Rank name="Treasurer" src={TreasurerImg} />;
    case 4n:
      return <Rank name="Diplomat" src={DiplomatImg} />;
    case 5n:
      return <Rank name="Recruiter" src={RecruiterImg} />;
    case 6n:
      return <Rank name="General" src={GeneralImg} />;
    case 7n:
      return <Rank name="Sergeant" src={SergeantImg} />;
    case 8n:
      return <Rank name="Member" src={MemberImg} />;
    case 9n:
      return <Rank name="Novice" src={NoviceImg} />;
    case null:
      return <>No rank</>;
    default:
      return <InvalidValue />;
  }
}
