import { matchMaker, Server } from "colyseus";
import { BattleRoom } from "./src/battle.room";
import http from "http";
import { WebSocketTransport } from "@colyseus/ws-transport";
import { DropRelayRoom } from "./src/relay.room";
import { enhanced_logging } from "./src/logging";
import express from "express";
import { Globals } from "./src/global";
import { getRoomLogger, LogLevel } from "./src/logging";
import { Server as WebSocketServer } from "ws"
import "dotenv/config";
const multiplayerServerPort = Number(process.env.SERVER_PORT) || 7000;
const viewerServerPort = Number(process.env.VIEWER_PORT) || 7001;



async function startGameServer() {

    const app = express();
    const server = http.createServer(app);


    const gameServer = new Server({
        // server: createServer(app),
        transport: new WebSocketTransport({
            server
        }),
    });

    gameServer.define("battle", BattleRoom);
    gameServer.define("relay", DropRelayRoom);

    const relay = await matchMaker.createRoom("relay", {});
    const battle = await matchMaker.createRoom("battle", {
        /* options */
    });

    enhanced_logging(relay, battle);

    return gameServer.listen(multiplayerServerPort);
}

async function startViewerServer() {
    const logger = getRoomLogger("VIEWER_SOCKET", LogLevel.DEBUG);

    const wsServer = new WebSocketServer({
        port: viewerServerPort,
    });

    wsServer.on("connection", (socket) => {
        Globals.viewerSocket.add(socket);
        logger.log("Viewer connected");
    });

    console.log(`Viewer started on port ${viewerServerPort}`);
}

startViewerServer();

startGameServer().then(() => {
    console.log(`Server started on port ${multiplayerServerPort}`);
});
