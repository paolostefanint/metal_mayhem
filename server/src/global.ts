interface GlobalsContainer {
    GAME_WAITING_TIME: number;
    GAME_EXIT_TIME: number;
    GAME_MAX_ROUNDS: number;
    MIN_PLAYERS_NUMBER: number;
    MAX_PLAYERS_NUMBER: number;
    viewerSocket: any;
    playersForThisGame: number;
}

export const Globals: GlobalsContainer = {
    // The time in ms to wait for players to join the game
    GAME_WAITING_TIME: 6000,
    // The time in ms to wait for players to exit the game
    GAME_EXIT_TIME: 5000,

    // ALwAYS KKEP GAME_WAITING_TIME > GAME_EXIT_TIME


    GAME_MAX_ROUNDS: 300,
    // The number of players in the game
    MIN_PLAYERS_NUMBER: 1,
    MAX_PLAYERS_NUMBER: 8,
    // socket.io sending seockt for field
    viewerSocket: undefined,
    playersForThisGame: 0,
};
