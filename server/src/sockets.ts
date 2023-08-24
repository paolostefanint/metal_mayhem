import WebSocket from "ws";
import * as dotenv from "dotenv";
import {EventEmitter} from "events";


dotenv.config();

const SOCKET_TO_NODE = process.env.SOCKET_TO_NODE;
const SOCKET_FROM_NODE = process.env.SOCKET_FROM_NODE;

export interface SendingSocket {
    send: (message: string) => void;
}

function createCoreSendingSocket(): SendingSocket {
    
    let coreAddress = "ws://127.0.0.1:40010"
    let ws: WebSocket;

    const connect = () => {
    
        ws = new WebSocket(coreAddress);

        ws.on('error', (err) => {
            console.log('Sending Socket Error: ' + err);
        });
        ws.on('close', (hadErr) => {
            console.log('Sending Socket Closed: ' + hadErr);
            setTimeout(() => {
                connect();
            }, 5000)
        });
        ws.on('open', () => {
            console.log('Sending Socket Opened');
        })
    }

    connect();

    return {
        send: message => {
            
            console.log("Sending message", message);

            if (ws.readyState !== WebSocket.OPEN) {
                console.log('Sending Socket Not Ready');
                return;
            }
            ws.send(message);
        }
    }


}

function createCoreListeningSocket(): EventEmitter { 

    let messageEmitter = new EventEmitter();
    let coreAddress = "ws://127.0.0.1:42000";

    const startServer = () => {
        let ws = new WebSocket(coreAddress);

        ws.on('error', (err) => {
            console.log('Listening Socket Error: ' + err);
        });
        ws.on('close', (hadErr) => {
            console.log('Listening Socket Closed: ' + hadErr);
            messageEmitter.emit('close');
            setTimeout(() => {
                startServer();
            }, 5000)
        });
        ws.on('open', () => {
            console.log('Listening Socket Opened');
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
