import WebSocket from "ws";
import * as dotenv from "dotenv";
import {EventEmitter} from "events";
import {getRoomLogger, LogLevel} from "./logging";

dotenv.config();

export interface SendingSocket {
    send: (message: string) => void;
}

const CORE_SENDING_ADDRESS = process.env.CORE_SENDING_ADDRESS || "ws://0.0.0.0:40010";
const CORE_RECEIVING_ADDRESS = process.env.CORE_RECEIVING_ADDRESS || "ws://0.0.0.0:42000";

function createCoreSendingSocket(): SendingSocket {
    
    let coreAddress = CORE_SENDING_ADDRESS; 
    let ws: WebSocket;

    const logger = getRoomLogger("CORE_SENDING_SOCKET", LogLevel.DEBUG)

    const connect = () => {
    
        ws = new WebSocket(coreAddress);

        ws.on('error', (err) => {
            logger.log("Sending Socket Error: " + err)
        });
        ws.on('close', (hadErr) => {
            logger.log("Sending Socket Closed: " + hadErr)
            setTimeout(() => {
                connect();
            }, 5000)
        });
        ws.on('open', () => {
            logger.log("Sending Socket Opened")
        })
    }

    connect();

    return {
        send: message => {
            if (ws.readyState !== WebSocket.OPEN) {
                logger.log("Sending Socket Not Ready")
                return;
            }
            ws.send(message);
        }
    }


}

function createCoreListeningSocket(): EventEmitter { 

    let messageEmitter = new EventEmitter();
    let coreAddress = CORE_RECEIVING_ADDRESS;
    const logger = getRoomLogger("CORE_LISTENING_SOCKET", LogLevel.DEBUG)

    const startServer = () => {
        let ws = new WebSocket(coreAddress);

        ws.on('error', (err) => {
            logger.log("Listening Socket Error: " + err)
        });
        ws.on('close', (hadErr) => {
            logger.log("Listening Socket Closed: " + hadErr)
            messageEmitter.emit('close');
            setTimeout(() => {
                startServer();
            }, 5000)
        });
        ws.on('open', () => {
            logger.log('Listening Socket Opened');
        });
        ws.on('message', (message) => {
            messageEmitter.emit('message', message)
        });
        
    };

    startServer();

    return messageEmitter;

}


export const coreListeningSocket = createCoreListeningSocket()
export const coreSendingSocket = createCoreSendingSocket()


const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));
