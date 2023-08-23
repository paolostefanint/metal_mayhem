import http from "http";
import { Client, Room } from "colyseus";
import { ClientState, Player } from "./state";
import { Globals } from "./global";
import { coreListeningSocket, coreSendingSocket } from "./sockets";

import { restoreTruncatedMessage } from "./message-handling";
import { GameStates } from "./state";

export class BattleRoom extends Room<ClientState> {
    autoDispose = false;
    static playerIndex = 1;

    // When room is initialized
    async onCreate(options: any) {
        
        this.setState(new ClientState());

        let lastRemainingToken = "";

        coreListeningSocket.on("data", (data) => {
            const incomingMessages = data.toString().split("\n");

            lastRemainingToken = restoreTruncatedMessage(
                incomingMessages,
                lastRemainingToken,
            );

            // console.log(`incomingMessage`, incomingMessages)

            incomingMessages
                .filter((message) => message.length > 0)
                .forEach((message) => {
                    const viewerSocket = Globals.viewerSocket;

                    // remove trailing |
                    message = message.slice(0, -1);

                    // console.log(`message`, message)

                    if (message.startsWith("*players:")) {
                        this.state.game.time = this.state.game.time - 1;

                        const playersString = message.substring(
                            "*players:".length,
                        );
                        playersString
                            .split("/")
                            .forEach((playerString: string) => {
                                const parsedPlayer: any =
                                    JSON.parse(playerString);
                                let player: Player;
                                this.state.players.forEach((p, _key) => {
                                    if (p.id === parsedPlayer.id) {
                                        player = p;
                                    }
                                });

                                if (player) {
                                    // fill player state with info from core
                                }
                            });

                        // send to viewwer
                        const viewerSocket = Globals.viewerSocket;
                        if (!viewerSocket) {
                            return;
                        }

                        const playersList = Object.values(
                            this.state.players.toJSON(),
                        );
                        viewerSocket.emit("players", playersList);
                        viewerSocket.emit("time", this.state.game.time);

                        return;
                    }

                    if (message.startsWith("*field:")) {
                        if (!viewerSocket) {
                            return;
                        }
                        viewerSocket.emit(
                            "field",
                            message.substring("*field:".length),
                        );
                        return null;
                    }

                    if (message.startsWith("*endgame")) {
                        this.state.game.status = GameStates.GAME_OVER;
                        console.log("BATTLE_END");
                        viewerSocket.emit("battle_end");
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
                });
        });

        this.onMessage("action", (client: Client, message: String) => {
            const player = this.state.players.get(client.sessionId);

            const socketMsg = `|${player.id}|(${message})`;
            coreSendingSocket.send(socketMsg);

            // coreSendingSocket.then(socket => {
            //   const toSend = `|${player.id}|(${message})`;
            //   console.log(`toSend`, toSend)
            //   socket.write(`${toSend}\n`);
            // })
        });

        this.onMessage("identity", (client, data) => {
            const [sub, name, avatar] = data.split("#");
            console.log(`BATTLE: got player identity`, sub, name, avatar);

            let existingPlayer: Player;
            this.state.players.forEach((p, key) => {
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

            console.log("coreSendingSocket", coreSendingSocket);

            let startGameMessage = `start|`;
            this.state.players.forEach((player) => {
                const startingPlayer = {
                    id: player.id,
                    name: player.name,
                    avatar: player.avatar,
                    color: player.color,
                    sub: player.sub,
                };
                startGameMessage += JSON.stringify(startingPlayer) + "$$";
            });

            coreSendingSocket.send(startGameMessage);
        }
    }

    onAuth(client: Client, options: any, request: http.IncomingMessage) {
        return true;
    }

    async onJoin(client: Client, options: any, auth: any) {}

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
