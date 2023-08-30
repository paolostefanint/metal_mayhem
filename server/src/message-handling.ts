import { CoreStates } from "./state";

export type CorePlayer = {
    id: string;  
    p: [number, number];
    dir: string;
    attack: boolean;
    health: number;
    sprite_state: string;
};

export type CoreMessage = {
    current_state: CoreStates;
    elapsed_time: number;
    remaining_time: number;
    players: any[];
};

export function parseCoreMessage(message: string): CoreMessage {
    try {
        return JSON.parse(message) as CoreMessage;
    } catch (e) {
        console.error("Error parsing core message", e);
        console.error("Message was", message);
    }
}

export type StateToRawClient = {
};
