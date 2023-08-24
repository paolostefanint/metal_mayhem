import http from "http";
import { Client, Room } from "colyseus";
import { ClientState, Player, PlayerPosition } from "./state";
import { Globals } from "./global";
import { coreListeningSocket, coreSendingSocket } from "./sockets";
import { parseCoreMessage, CoreMessage, CorePlayer } from "./message-handling";
import { GameStates, CoreStates } from "./state";

export class BattleRoom extends Room<ClientState> {
    autoDispose = false;
    static playerIndex = 1;

    // When room is initialized
    async onCreate(options: any) {
        // init battle state
        this.setState(new ClientState());

        // init procedure to send to the viewer the status of the game
        // this is only for clients not implementing colyseus
        this.startViewerSendingInterval();

        // handling of core disconnections
        // it souldn't happen, but if it does, we need to handle it
        coreListeningSocket.on("close", this.handleCoreConnectionClosed.bind(this));

        // core message handling procedure
        coreListeningSocket.on("message", this.handleCoreMessage.bind(this));



            // lastRemainingToken = restoreTruncatedMessage(
            //     incomingMessages,
            //     lastRemainingToken,
            // );

            // // console.log(`incomingMessage`, incomingMessages)

            // incomingMessages
            //     .filter((message) => message.length > 0)
            //     .forEach((message) => {
            //         const viewerSocket = Globals.viewerSocket;

            //         // remove trailing |
            //         message = message.slice(0, -1);

            //         // console.log(`message`, message)

            //         if (message.startsWith("*players:")) {
            //             this.state.game.time = this.state.game.time - 1;

            //             const playersString = message.substring(
            //                 "*players:".length,
            //             );
            //             playersString
            //                 .split("/")
            //                 .forEach((playerString: string) => {
            //                     const parsedPlayer: any =
            //                         JSON.parse(playerString);
            //                     let player: Player;
            //                     this.state.players.forEach((p, _key) => {
            //                         if (p.id === parsedPlayer.id) {
            //                             player = p;
            //                         }
            //                     });

            //                     if (player) {
            //                         // fill player state with info from core
            //                     }
            //                 });

            //             // send to viewwer
            //             const viewerSocket = Globals.viewerSocket;
            //             if (!viewerSocket) {
            //                 return;
            //             }

            //             const playersList = Object.values(
            //                 this.state.players.toJSON(),
            //             );
            //             viewerSocket.emit("players", playersList);
            //             viewerSocket.emit("time", this.state.game.time);

            //             return;
            //         }

            //         if (message.startsWith("*field:")) {
            //             if (!viewerSocket) {
            //                 return;
            //             }
            //             viewerSocket.emit(
            //                 "field",
            //                 message.substring("*field:".length),
            //             );
            //             return null;
            //         }

            //         if (message.startsWith("*endgame")) {
            //             this.state.game.status = GameStates.GAME_OVER;
            //             console.log("BATTLE_END");
            //             viewerSocket.emit("battle_end");
            //             this.broadcast("battle_end");

            //             setTimeout(() => {
            //                 console.log("ENDGAME");
            //                 this.state.game.status = GameStates.GAME_OVER;
            //                 this.broadcast("endgame");

            //                 this.state.players.clear();

            //                 // tell relay that the game is over
            //                 this.presence.publish("battle_state", "endgame");
            //             }, Globals.GAME_EXIT_TIME);
            //         }
            //     });
        // });

        this.onMessage("action", (client: Client, message: String) => {

            if (this.state.game.status !== GameStates.RUNNING) {
                return;
            }

            const player = this.state.players.get(client.sessionId);

            const socketMsg = `input|||${player.id}|(${message})`;
            coreSendingSocket.send(socketMsg);

        });

        this.onMessage("identity", (client, data) => {
            const [sub, name, avatar] = data.split("#");
            console.log(`BATTLE: got player identity`, sub, name, avatar);

            let existingPlayer: Player;
            this.state.players.forEach((p) => {
                if (p.sub === sub) {
                    existingPlayer = p;
                }
            });

            if (existingPlayer) {
                existingPlayer.connected = true;
                // console.log(`existingPlayer`, existingPlayer)
                this.state.players.set(
                    client.sessionId,
                    existingPlayer.clone(),
                );
                if (client.sessionId !== existingPlayer.sessionId) {
                    this.state.players.delete(existingPlayer.sessionId);
                }

                // if (!this.state.gameOver) {
                client.send("battle_start");
                // }
            } else {
                const player = new Player();
                player.id = BattleRoom.playerIndex++;
                player.sessionId = client.sessionId;
                player.name = name;
                player.avatar = avatar;
                player.sub = sub;
                player.connected = true;

                this.state.players.set(client.sessionId, player);

                client.send(this.state.players.size);
            }
        });

        this.presence.subscribe("battle_start", (players: Set<Player>) => {
            players.forEach((p) => {
                const player = new Player();
                player.id = BattleRoom.playerIndex++;
                player.sessionId = p.sessionId;
                player.name = p.name;
                player.avatar = p.avatar;
                player.sub = p.sub;
                player.connected = false;

                // console.log(`player`, player)

                this.state.players.set(p.sessionId, player);
            });

            this.startGame();
        });
    }

    private handleCoreConnectionClosed() {
        this.state.game.status = GameStates.GAME_ERROR;
        console.log("GAME_ERROR");
        this.broadcast("battle_end");

        setTimeout(() => {
            console.log("ENDGAME");
            this.state.game.status = GameStates.GAME_OVER;
            this.broadcast("endgame");

            this.state.players.clear();

            // tell relay that the game is over
            this.presence.publish("battle_state", "endgame");
        }, Globals.GAME_EXIT_TIME);
    }

    /**
     * startViewerSendingInterval.
     */
    private startViewerSendingInterval() {
        setTimeout(() => {
            setInterval(() => {
                if (!Globals.viewerSocket) {
                    return;
                }
                Globals.viewerSocket.send(JSON.stringify(this.state.toJSON()));
            }, 500);
        }, 1000);
    }

    /**
     * 
     *
     *
     *
     *
     * @param {string} data
     */
    private handleCoreMessage(data: string) {
        const message = parseCoreMessage(data);

        const coreGameState = message.current_state;
        const corePlayers = message.players;

        switch (coreGameState) {
            case CoreStates.WaitingForPlayers:
                // console.log("BATTLE_WAITING_FOR_PLAYERS");
                // do nothing

                break;

            case CoreStates.Running:
                this.state.game.time = +new Date();
                this.state.game.remainingTime = 0;
                this.state.game.status = GameStates.RUNNING;

                this.state.players.forEach((p) => {
                    const corePlayer = corePlayers.find(
                        (corePlayer) => corePlayer.id === p.id,
                    ) as CorePlayer;
                    if (!corePlayer) {
                        console.log(
                            "WTF players from server do no match core players?",
                            p,
                            corePlayers,
                        );
                        return;
                    }

                    p.life = corePlayer.health;
                    p.direction = corePlayer.dir;
                    p.position = new PlayerPosition();
                    p.position.x = corePlayer.p[0];
                    p.position.y = corePlayer.p[1];
                    p.spriteState = corePlayer.sprite_state;
                });

                break;

            case CoreStates.RoundEnd:

                if (this.state.game.status === GameStates.GAME_OVER) {
                    break;
                }

                this.state.game.status = GameStates.GAME_OVER;
                console.log("BATTLE_END");
                this.broadcast("battle_end");

                setTimeout(() => {
                    console.log("ENDGAME");
                    this.state.game.status = GameStates.GAME_OVER;
                    this.broadcast("endgame");

                    this.state.players.clear();

                    // tell relay that the game is over
                    this.presence.publish("battle_state", "endgame");
                }, Globals.GAME_EXIT_TIME);

                break;

            default:
                console.log("UNKNOWN_GAME_STATE");
                break;
        }
    }

    private startGame() {
        this.broadcast("battle_start");

        if (!(this.state.game.status === GameStates.RUNNING)) {
            this.state.game.status = GameStates.RUNNING;

            const colors = [
                "#4EC3CB",
                "#F2C94C",
                "#FF9457",
                "#FF6694",
                "#9F0B76",
                "#9896A5",
                "#3c4fd2",
                "#7fcc58",
            ];

            let index = 0;
            this.state.players.forEach((p, key) => {
                p.color = colors[index % colors.length];
                index++;
            });

            const playerIds = [];
            this.state.players.forEach((player) => {
                playerIds.push(player.id);
            });

            let startGameMessage = `start|||`;
            let playersToSend = [];
            this.state.players.forEach((player) => {
                const startingPlayer = {
                    player_id: player.id,
                    name: player.name,
                    avatar: player.avatar,
                    color: player.color,
                    sub: player.sub,
                    initial_position: [10.0, 10.0],
                };
                playersToSend.push(startingPlayer);
            });

            coreSendingSocket.send(
                `${startGameMessage}${JSON.stringify(playersToSend)}`,
            );
        }
    }

    onAuth(client: Client, options: any, request: http.IncomingMessage) {
        return true;
    }

    async onJoin(client: Client, options: any, auth: any) {
        console.log("JOIN", client.sessionId, options, auth);
    }

    // When a client leaves the room
    async onLeave(client: Client, consented: boolean) {
        const player = this.state.players.get(client.sessionId);
        player.connected = false;

        try {
            if (consented) {
                throw new Error("consented leave");
            }

            // allow disconnected client to reconnect into this room until 20 seconds
            await this.allowReconnection(client, 60);

            // client returned! let's re-activate it.
            player.connected = true;
        } catch (e) {
            console.log(`client disconnected nd removed`, player.sessionId);
        }
    }

    // Cleanup callback, called after there are no more clients in the room. (see `autoDispose`)
    onDispose() {
        console.log("onDispose battle");
        coreSendingSocket.send("stop");
    }
}
