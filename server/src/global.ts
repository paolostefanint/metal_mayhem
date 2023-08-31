interface GlobalsContainer {
    GAME_WAITING_TIME: number;
    GAME_EXIT_TIME: number;
    GAME_MAX_ROUNDS: number;
    MIN_PLAYERS_NUMBER: number;
    MAX_PLAYERS_NUMBER: number;
    viewerSocket: ViewerSocketWrapper;
    playersForThisGame: number;
}

type ViewerSocketWrapper = {
    add: (socket: any) => void;
    send: (data: any) => void;
};

const viewerSocketsWrapper = (): ViewerSocketWrapper => {
    const sockets: any[] = [];

    return {
        add: (socket: any) => {
            sockets.push(socket);
        },
        send: (data: any) => {
            sockets.forEach((socket) => {
                try {
                    socket.send(data);
                } catch (e) {
                    // console.log("Error sending data to viewer");
                    // console.log(e);
                }
            });
        },
    };
};

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
    //
    // socket.io sending seockt for field
    viewerSocket: viewerSocketsWrapper(),
    playersForThisGame: 0,
};
