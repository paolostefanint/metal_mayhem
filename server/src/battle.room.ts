import http from "http";
import { Client, Room } from "colyseus";
import { MapSchema } from "@colyseus/schema";
import { ClientState, Player, PlayerPosition, PresenceMessages } from "./state";
import { Globals } from "./global";
import { coreListeningSocket, coreSendingSocket } from "./sockets";
import { parseCoreMessage, CoreMessage, CorePlayer } from "./message-handling";
import { CoreStates } from "./state";
import { GameStates, FSM } from "./fsm";
import { getRoomLogger, LogLevel } from "./logging";

const SERVER_TO_CLIENT_SPEED = 1000 / 10;

export class BattleRoom extends Room<ClientState> {
    autoDispose = false;
    static playerIndex = 1;

    fsm = new FSM();

    logger = getRoomLogger("BATTLE", LogLevel.DEBUG);

    // When room is initialized
    async onCreate(options: any) {
        // init battle state
        this.setState(new ClientState());

        // init procedure to send to the viewer the status of the game
        // this is only for clients not implementing colyseus
        this.startViewerSendingInterval();

        // handling of core disconnections
        // it souldn't happen, but if it does, we need to handle it
        coreListeningSocket.on(
            "close",
            this.handleCoreConnectionClosed.bind(this),
        );

        // core message handling procedure
        coreListeningSocket.on("message", this.handleCoreMessage.bind(this));

        this.onMessage("action", this.handleUserInput.bind(this));

        this.onMessage("identity", this.handleUserIdentity.bind(this));

        this.presence.subscribe(
            PresenceMessages.BATTLE_PLAYERS,
            (players: Set<Player>) => {
                players.forEach((p) => {
                    const player = new Player();
                    player.id = BattleRoom.playerIndex++;
                    player.sessionId = p.sessionId;
                    player.name = p.name;
                    player.avatar = p.avatar;
                    player.pic = p.pic;
                    player.sub = p.sub;
                    player.connected = false;

                    this.state.players.set(p.sessionId, player);
                });

                this.startGame();
            },
        );

    }

    private handleUserIdentity(client: Client, data: string) {
        const [sub, name, avatar, pic] = data.split("#");
        this.logger.log(`got player identity`, sub, name, avatar, pic);

        // try to find a player with the same sub
        let existingPlayer: Player;
        this.state.players.forEach((p) => {
            if (p.sub === sub) {
                existingPlayer = p;
            }
        });

        if (existingPlayer) {
            // create a clone of the existing player
            const existingPlayerClone = existingPlayer.clone();

            // remove the existing player
            this.state.players.delete(existingPlayer.sessionId);

            // add the  clone with the new session id
            existingPlayerClone.connected = true;
            existingPlayerClone.sessionId = client.sessionId;
            this.state.players.set(client.sessionId, existingPlayerClone);

            client.send("battle_start");
        } else {
            // create new player
            const player = new Player();
            player.id = BattleRoom.playerIndex++;
            player.sessionId = client.sessionId;
            player.name = name;
            player.avatar = avatar|| "1";
            player.pic = pic || "";
            player.sub = sub;
            player.connected = true;

            this.state.players.set(client.sessionId, player);

            client.send(this.state.players.size);
        }
    }

    private handleUserInput(client: Client, message: string) {
        if (this.state.game.status !== GameStates.RUNNING) {
            return;
        }

        const player = this.state.players.get(client.sessionId);

        if (!player) {
            return;
        }

        const socketMsg = `input|||${player.id}|(${message})`;
        coreSendingSocket.send(socketMsg);
    }

    /**
     * handleCoreConnectionClosed.
     */
    private handleCoreConnectionClosed() {
        const stateChange = this.fsm.to(GameStates.GAME_ERROR);

        if (!stateChange) {
            console.error("OMG something very wrong happended");
        }

        this.state.game.status = GameStates.GAME_ERROR;
        this.logger.log("GAME_ERROR");
        this.broadcast("battle_end");

        // tell relay that the game is over
        this.presence.publish(
            PresenceMessages.BATTLE_STATE,
            GameStates.GAME_OVER,
        );

        setTimeout(() => {
            this.logger.log("ENDGAME");
            this.state.game.status = GameStates.WAITING_FOR_PLAYERS;
            this.broadcast("endgame");

            this.state.players.clear();
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
                const state: any = this.state.toJSON();
                const players = Object.values(state.players);
                state.players = players.map((p: any) => {
                    return {
                        ...p,
                        conn: p.connected,
                        position: {
                            // trim psition to 1 decimal
                            x: Math.round(p.position.x * 10) / 10,
                            y: Math.round(p.position.y * 10) / 10,
                        },
                    };
                });
                Globals.viewerSocket.send(JSON.stringify(state));
            }, SERVER_TO_CLIENT_SPEED);
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
        const elapsedTime = message.elapsed_time;
        const remainingTime = message.remaining_time;

        this.state.game.remainingTime = remainingTime;

        switch (coreGameState) {
            case CoreStates.WaitingForPlayers:
                this.changeStateToWaitingForPlayers();
                break;

            case CoreStates.Running:
                this.changeStateToRunning(corePlayers, remainingTime);
                break;

            case CoreStates.RoundEnd:
                this.changeStateToRoundEnd(corePlayers, remainingTime);
                break;

            default:
                this.logger.log("UNKNOWN_GAME_STATE");
                break;
        }
    }

    private changeStateToWaitingForPlayers() {
        // this.logger.log("BATTLE_WAITING_FOR_PLAYERS");
        // do nothing
        const stateChange = this.fsm.to(GameStates.WAITING_FOR_PLAYERS);
        if (!stateChange) {
            console.error("OMG something very wrong happended");
        }
        this.state.game.status = GameStates.WAITING_FOR_PLAYERS;
    }

    private changeStateToRunning(corePlayers: any[], remainingTime: number) {
        this.state.game.time = +new Date();
        this.state.game.remainingTime = remainingTime;

        const stateChange = this.fsm.to(GameStates.RUNNING);
        if (!stateChange) {
            console.error("OMG something very wrong happended");
        }

        if (this.state.game.status !== GameStates.RUNNING) {
            // ensure relay in on the same state
            this.presence.publish("battle_state", GameStates.RUNNING);
        }

        this.state.game.status = GameStates.RUNNING;

        this.state.players.forEach((p) => {
            const corePlayer = corePlayers.find(
                (corePlayer) => corePlayer.id === p.id,
            ) as CorePlayer;

            if (!corePlayer) {
                this.logger.log("WTF server players != core players?", p);
                return;
            }

            p.life = corePlayer.health;
            p.direction = corePlayer.dir;
            p.position = new PlayerPosition();
            p.position.x = corePlayer.p[0];
            p.position.y = corePlayer.p[1];
            p.spriteState = corePlayer.sprite_state;
            p.damaged = corePlayer.damaged;
        });
    }

    private changeStateToRoundEnd(corePlayers: any[], remainingTime: number) {
        if (!this.shouldHandleRoundEnd()) {
            return;
        }

        const stateChange = this.fsm.to(GameStates.GAME_OVER);
        if (!stateChange) {
            console.error("OMG something very wrong happended");
        }

        this.state.game.status = GameStates.GAME_OVER;
        this.logger.log("BATTLE_END");
        this.broadcast("battle_end");

        // tell relay that the game is over
        this.presence.publish(
            PresenceMessages.BATTLE_STATE,
            GameStates.GAME_OVER,
        );

        setTimeout(() => {
            this.logger.log("Back to waiting for players");
            this.state.game.status = GameStates.WAITING_FOR_PLAYERS;

            // tell relay that the game is over
            this.presence.publish(
                PresenceMessages.BATTLE_STATE,
                GameStates.WAITING_FOR_PLAYERS,
            );
            // detstroy players here so client can see results
            this.state.players = new MapSchema<Player>();

            this.broadcast("endgame");
        }, Globals.GAME_EXIT_TIME);
    }

    private shouldHandleRoundEnd() {
        return (
            this.state.game.status !== GameStates.GAME_OVER &&
            this.state.game.status !== GameStates.WAITING_FOR_PLAYERS
        );
    }

    private startGame() {
        this.broadcast("battle_start");
        this.presence.publish("battle_state", GameStates.RUNNING);

        this.state.game.roundCountdown = 0;

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

                const avatar = parseInt(player.avatar) || 1; 

                const startingPlayer = {
                    player_id: player.id,
                    name: player.name,
                    avatar,
                    pic: player.pic,
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
        this.logger.log("JOIN", client.sessionId, options, auth);
    }

    // When a client leaves the room
    async onLeave(client: Client, consented: boolean) {
        const player = this.state.players.get(client.sessionId);

        if (!player) {
            this.logger.log("player not found", client.sessionId);
            this.state.players.forEach((p) => {
                this.logger.log("player in room", p.sessionId);
            });
            return;
        }

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
            this.logger.log(
                `client disconnected and removed`,
                player.sessionId,
            );
        }
    }

    // Cleanup callback, called after there are no more clients in the room. (see `autoDispose`)
    onDispose() {
        this.logger.log("onDispose battle");
        coreSendingSocket.send("stop");
    }
}
