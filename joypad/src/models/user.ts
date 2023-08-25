export interface BattleInfoCurrentPlayer {
  id: number
  connected: boolean
  name: string
  sessionId: string
  sub: string
  avatar: string
  pic: string
  color: string
  life: number
  spriteState: string
}

export interface PlayerDetail {
  name: string
  connected: boolean
  avatar: string
  pic: string
}
