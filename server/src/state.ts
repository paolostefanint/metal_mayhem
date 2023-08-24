import {Schema, ArraySchema, type, MapSchema} from "@colyseus/schema";


export enum CoreStates {
    WaitingForPlayers = "WaitingForPlayers",
    RoundCountdown = "RoundCountdown",
    Running = "Running",
    RoundEnd = "RoundEnd",
}

export enum GameStates {
    GAME_OVER = "GAME_OVER", 
    RUNNING = "RUNNING",
    GAME_ERROR = "GAME_ERROR",
}

export class PlayerPosition extends Schema {    
    @type("number") x: number;
    @type("number") y: number;
}

export class Player extends Schema {
    @type("int32") id: number;
    @type("boolean") connected: boolean;
    @type("string") name: string;
    @type("string") sessionId: string;
    @type("string") sub: string;
    @type("string") color: string;
    @type("string") avatar: string;
    @type("int32") life: number;
    @type(PlayerPosition) position = new PlayerPosition();
    @type("string") direction: string;
    @type("string") score: number;
    @type("string") spriteState: string;
}

export class GameState extends Schema {
    @type("string") status: GameStates = GameStates.GAME_OVER;
    @type("number") time: number = 0;
    @type("number") remainingTime: number = 0;
}


export class ClientState extends Schema {
    @type(GameState) game = new GameState();
    @type({map: Player}) players = new MapSchema<Player>();
}

export class RelayState extends Schema {
    @type({map: Player}) players = new MapSchema<Player>();
    @type("boolean") gameRunning: boolean = false;
}
