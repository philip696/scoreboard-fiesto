import type { PlayerInfo } from "~/types/PlayerInfo";
export interface TeamInfo {
  id: string;
  name: string;
  picture: string;
  score: number;
  foul: number;
  timeout: number;
  member: PlayerInfo[];
}
