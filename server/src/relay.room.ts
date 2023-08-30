import { Client, Room } from "colyseus";
import { Queue } from "./queue";
import http from "http";
import { Player, PresenceMessages, RelayState } from "./state";
import { Globals } from "./global";
import { createPacketSizeTicker } from "./packet-size-ticker";
import { GameStates, FSM } from "./fsm";
import { getRoomLogger, LogLevel } from "./logging";

export interface PlayerInitData {
    client: Client;
    sub: string;
    name: string;
    avatar: string;
    pic: string;
}

export class DropRelayRoom extends Room<RelayState> {
    // tslint:disable-line

    fsm = new FSM();

    public allowReconnectionTime: number = 0;
    autoDispose = false;

    waitingPlayers = new Queue<Player>();
    playingPlayers = new Set<Player>();

    waitingListMissingTime = 0;
    waitingListTimeout: NodeJS.Timeout;
    waitingListTimer: NodeJS.Timer;

    packetSizeTicker = createPacketSizeTicker();

    logger = getRoomLogger("RELAY", LogLevel.DEBUG);

    public onCreate(
        _options: Partial<{
            maxClients: number;
            allowReconnectionTime: number;
            metadata: any;
        }>,
    ) {
        this.logger.log("Creating Relay Room...");

        this.setState(new RelayState());

        this.onMessage("identity", this.handleNewIdentity.bind(this));

        this.presence.subscribe(
            PresenceMessages.BATTLE_STATE,
            (state: GameStates) => {
                this.state.status = state;

                this.logger.log("RELAY: changed state to", state);

                if (state === GameStates.GAME_OVER) {
                    this.logger.log("clearing players");
                    this.playingPlayers = new Set<Player>();

                    if (
                        !this.isGameRunning() &&
                        this.hasEnoughConnectedPlayers()
                    ) {
                        this.startGameTimer();
                        this.startBroadcastTimer();
                    }
                }
            },
        );

        this.logger.log("Relay Room created");
    }

    private handleNewIdentity(client: Client, data: string) {
        const [sub, name, avatar, pic] = data.split("#");

        this.logger.log(`got player identity`, sub, name);
        const playerInitData: PlayerInitData = {
            client,
            sub,
            name,
            pic,
            avatar,
        };

        let player = DropRelayRoom.createPlayerOnJoin(playerInitData);

        if (this.playerAlreadyExists(sub)) {
            this.logger.log("Player already exists, reconnecting...");
            this.handlePlayerReconnection(player, client);

            if (this.playerShouldEnterBattleOnConnect(player)) {
                this.logger.log("Player should enter battle on connect");
                this.sendPlayerToBattle(client);
            } else {
                this.putPlayerInWaitingList(player);

                if (!this.isGameRunning() && this.hasEnoughConnectedPlayers()) {
                    if (!this.timerIsRunning()) {
                        this.startGameTimer();
                        this.startBroadcastTimer();
                    }
                }
            }
        } else {
            this.handleNewPlayerJoining(player);
            this.putPlayerInWaitingList(player);

            if (!this.isGameRunning() && this.hasEnoughConnectedPlayers()) {
                this.startGameTimer();
                this.startBroadcastTimer();
            }
        }

        this.broadcatsQueue();
    }

    private timerIsRunning() {
        return (
            this.waitingListTimeout != null &&
            this.waitingListTimer != undefined
        );
    }

    private startBroadcastTimer() {
        this.waitingListMissingTime = Globals.GAME_WAITING_TIME;
        this.waitingListTimer = setInterval(() => {
            this.waitingListMissingTime -= 1000;
            this.broadcast("timer", this.waitingListMissingTime);
            this.presence.publish(
                "round_countdown",
                this.waitingListMissingTime / 1000,
            );
        }, 1000);
    }

    private endBroadcastTimer() {
        this.logger.log("Ending waiting list timer");
        clearInterval(this.waitingListTimer);
    }

    public onAuth(client: Client, options: any, request: http.IncomingMessage) {
        return true;
    }

    public async onJoin(client: Client, options: any = {}) {
        this.logger.log("JOINED", client.sessionId);
        return true;
    }

    public async onLeave(client: Client, consented: boolean) {
        let p: Player;
        this.state.players.forEach((player, key) => {
            if (player.sessionId === client.sessionId) {
                p = player;
            }
        });

        if (p) {
            p.connected = false;
            this.state.players.set(p.sub, p);

            const leavingWaitingPlayer = this.waitingPlayers.find(
                (waitinggPlayer) => waitinggPlayer.sub === p.sub,
            );
            if (leavingWaitingPlayer) {
                leavingWaitingPlayer.connected = false;
            }

            this.broadcatsQueue();
            this.logger.log(`${p.name} left relay`);
        }
    }

    private startGameTimer() {
        if (this.waitingListTimeout) {
            this.logger.log("Clearing waiting list timeout");
            clearTimeout(this.waitingListTimeout);
            clearInterval(this.waitingListTimer);
        }

        this.logger.log("Starting new waiting list timeout");
        this.waitingListTimeout = setTimeout(() => {
            if (this.shouldStartNewGame()) {
                this.startNewGame();
            } else {
                this.startGameTimer();
            }
            this.waitingListTimeout = null;
        }, Globals.GAME_WAITING_TIME + 500);
    }

    private broadcatsQueue() {
        const toBroadcast = this.waitingPlayers
            .toArray()
            .map((p) => `${p.name}|${p.connected}|${p.avatar}|${p.pic}`);
        console.log("toBroadcast", toBroadcast);
        this.broadcast("queue", toBroadcast);
    }

    private startNewGame() {
        this.endBroadcastTimer();

        const waiting = this.waitingPlayers.toArray();
        this.logger.log(
            `players`,
            waiting.map((p) => p.name),
        );

        const newWaitingPlayersQueue = new Queue<Player>();
        // let addedPlayers = 0;

        Globals.playersForThisGame = Math.min(
            Globals.MAX_PLAYERS_NUMBER,
            waiting.length,
        );

        for (let i = 0; i < waiting.length; i++) {
            const player: Player = waiting[i];

            if (!player) {
                continue;
            }

            if (i < Globals.MAX_PLAYERS_NUMBER) {
                this.playingPlayers.add(player);

                const involvedClient = this.clients.find(
                    (client) => client.sessionId === player.sessionId,
                );
                if (involvedClient) {
                    involvedClient.send("battle_ready");
                }
            } else {
                newWaitingPlayersQueue.enqueue(player);
            }
        }

        this.logger.log(
            "this.playingPlayers",
            Array.from(this.playingPlayers).map((p) => p.name),
        );

        this.presence.publish(
            PresenceMessages.BATTLE_PLAYERS,
            this.playingPlayers,
        );

        this.waitingPlayers = newWaitingPlayersQueue;
    }

    private getPlayersThatShouldStartPlaying() {
        const waiting = this.waitingPlayers.toArray();
        const numberOfPlayers = Math.min(
            Globals.MAX_PLAYERS_NUMBER,
            waiting.length,
        );
        // kind of deep copy ??
        return waiting.slice(0, numberOfPlayers).map((p) => ({ ...p }));
    }

    private putPlayerInWaitingList(player: Player) {
        if (!this.waitingPlayers.has((p) => p.sub === player.sub)) {
            this.waitingPlayers.enqueue(player);
        }
    }

    private playerIsInWaitingList(player: Player) {
        return this.waitingPlayers.has((p) => p.sub === player.sub);
    }

    private handleNewPlayerJoining(player: Player) {
        this.state.players.set(player.sub, player);
        this.logger.log(`${player.name} joined relay`);
    }

    private sendPlayerToBattle(client: Client) {
        // if it is, tell the player client to reconnect to the game
        client.send("battle_ready", true);
    }

    private playerAlreadyExists(sub: string) {
        return this.state.players.has(sub);
    }

    private playerShouldEnterBattleOnConnect(player: Player) {
        return (
            this.isGameRunning() &&
            Array.from(this.playingPlayers).filter((p) => p.sub === player.sub)
                .length > 0
        );
    }

    private handlePlayerReconnection(player: Player, client: Client) {
        const existingPlayer = this.state.players.get(player.sub);
        existingPlayer.sessionId = client.sessionId;
        existingPlayer.connected = true;
        existingPlayer.avatar = player.avatar;
        this.logger.log(`${existingPlayer.name} reconnected`);
    }

    private static createPlayerOnJoin(playerInitData: PlayerInitData) {
        let player = new Player();
        player.connected = true;
        player.sessionId = playerInitData.client.sessionId;
        player.sub = playerInitData.sub;
        player.name = playerInitData.name;
        player.avatar = playerInitData.avatar;
        player.pic = playerInitData.pic;
        return player;
    }

    private shouldStartNewGame() {
        return !this.isGameRunning() && this.hasEnoughPlayers();
    }

    private isGameRunning() {
        return this.state.status === GameStates.RUNNING;
    }

    private hasEnoughPlayers() {
        return this.waitingPlayers.size() >= Globals.MIN_PLAYERS_NUMBER;
    }

    private hasEnoughConnectedPlayers() {
        return (
            this.waitingPlayers.toArray().filter((p) => p.connected).length >=
            Globals.MIN_PLAYERS_NUMBER
        );
    }

    onDispose() {
        this.logger.log("onDispose relay");
    }
}
