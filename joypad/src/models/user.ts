export interface BattleInfoCurrentPlayer {
  id: number
  connected: boolean
  name: string
  sessionId: string
  sub: string
  avatar: string
  color: string
  life: number
  spriteState: string
}

export interface PlayerDetail {
  name: string
  connected: boolean
  avatar: string
}
