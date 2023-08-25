import { matchMaker, Server } from "colyseus";
import { BattleRoom } from "./src/battle.room";
import http from "http";
import { WebSocketTransport } from "@colyseus/ws-transport";
import { DropRelayRoom } from "./src/relay.room";
import { enhanced_logging } from "./src/logging";
import express from "express";
import { Server as SocketIoServer } from "socket.io";
import { Globals } from "./src/global";
import { getRoomLogger, LogLevel } from "./src/logging";

const multiplayerServerPort = Number(process.env.SERVER_PORT) || 7000;
const viewerServerPort = Number(process.env.VIEWER_PORT) || 7001;

async function startGameServer() {
    const gameServer = new Server({
        // server: createServer(app),
        transport: new WebSocketTransport({}),
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
    const app = express();
    app.use(express.json());
    const logger = getRoomLogger("VIEWER_SOCKET", LogLevel.DEBUG);

    const server = http.createServer(app);

    const socketPath =
        process.env.NODE_ENV === "production" ? "/viewersocket/socket.io" : "";

    const io = new SocketIoServer(server, {
        path: socketPath,
        cors: {
            origin: "*",
            methods: ["GET", "POST"],
        },
    });

    io.on("connection", (socket) => {
        Globals.viewerSocket = socket;
        logger.log("Viewer connected");
    });

    return server.listen(viewerServerPort);
}

startViewerServer().then(() => {
    console.log(`Viewer started on port ${viewerServerPort}`);
});

startGameServer().then(() => {
    console.log(`Server started on port ${multiplayerServerPort}`);
});
