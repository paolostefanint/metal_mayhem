import {RoomListingData} from "colyseus";
import chalk from "chalk";

export function enhanced_logging(relay: RoomListingData<any>, battle: RoomListingData<any>) {
    let ui = {
        relay: {
            clients: relay.clients
        },
        battle: {
            clients: battle.clients
        }
    }

    setInterval(() => {

        const newUi = {
            relay: {
                clients: relay.clients
            },
            battle: {
                clients: battle.clients,
            }
        }

        if (JSON.stringify(newUi) !== JSON.stringify(ui)) {

            ui = newUi;

            console.log('----------------------------------------------------');
            console.table(newUi);
            console.log('----------------------------------------------------');
        }


    }, 500)
}

export enum LogLevel {
    DEBUG = 0,
    INFO = 1,
    ERROR = 2
}

export type RoomLogger = {
    log: (message: string, level?: LogLevel) => void
}
export function getRoomLogger(room: string, generalLevel: LogLevel = LogLevel.INFO) {

    
    const chalkOut = (message: string) => {
        switch (room.toLowerCase()) {
            case 'relay':
                return chalk.blue(message);
            case 'battle':
                return chalk.red(message);
            default:
                return message;
        }

    }

    return {
        log: (...args) => {
            console.log(chalkOut(`[${room}]`), ...args);
        }
    }
}
