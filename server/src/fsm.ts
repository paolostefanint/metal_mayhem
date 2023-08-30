export enum GameStates {
    WAITING_FOR_PLAYERS = "WAITING_FOR_PLAYERS",
    GAME_OVER = "GAME_OVER",
    RUNNING = "RUNNING",
    GAME_ERROR = "GAME_ERROR",
}

export class FSM {
    private state: GameStates;

    constructor() {
        this.state = GameStates.WAITING_FOR_PLAYERS;
    }

    public to(toState: GameStates) {
        switch (this.state) {
            case GameStates.WAITING_FOR_PLAYERS:

                if (toState === GameStates.RUNNING) {
                    this.state = GameStates.RUNNING;
                }
                if (toState === GameStates.GAME_ERROR) {
                    this.state = GameStates.GAME_ERROR;
                }
                break;

            case GameStates.RUNNING:
                if (toState === GameStates.GAME_OVER) {
                    this.state = GameStates.GAME_OVER;
                }
                if (toState === GameStates.GAME_ERROR) {
                    this.state = GameStates.GAME_ERROR;
                }
                break;

            case GameStates.GAME_OVER:
                if (toState === GameStates.WAITING_FOR_PLAYERS) {
                    this.state = GameStates.WAITING_FOR_PLAYERS;
                }
                if (toState === GameStates.GAME_ERROR) {
                    this.state = GameStates.GAME_ERROR;
                }
                break;
            
               case GameStates.GAME_ERROR:
                if (toState === GameStates.WAITING_FOR_PLAYERS) {
                   this.state = GameStates.WAITING_FOR_PLAYERS;
               }
               break;

            default:
                console.error(
                    `Invalid state transition from ${this.state} to ${toState}`,
                );
                return false;
        }

        return true;
    }
}
