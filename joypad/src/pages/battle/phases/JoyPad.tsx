import { createStore } from "solid-js/store";
import { onCleanup, createSignal, onMount } from "solid-js";
import { BattleInfoCurrentPlayer } from "../../../models/user";
import Splash from "../../../shared/icons/Splash";
import p5 from "p5";

interface JoypadState {
    movement: [number, number];
    attack: boolean;
}

const initialState: JoypadState = {
    movement: [0, 0],
    attack: false,
};

interface JoyPadProps {
    onChange: ({
        movement,
        attack,
    }: {
        movement: [number, number];
        attack: boolean;
    }) => void;
    playerStats: BattleInfoCurrentPlayer;
}

const JoyPad = ({ onChange, playerStats }: JoyPadProps) => {
    const [joyPadStore, setJoyPadStore] =
        createStore<JoypadState>(initialState);
    const [p5Instance, setP5Instance] = createSignal<p5 | null>(null);
    let canvas;

    const sketch = (p: p5) => {
        const joystickCenter = [150, 150];
        const joystickRadius = 100;

        p.setup = () => {
            canvas = p.createCanvas(300, 300);
            canvas.parent("field");
            p.background(51);
            p.frameRate(10);
        };

        p.draw = () => {
            p.background(51);

            // draw a cross on joystick center
            p.stroke(255);
            p.strokeWeight(2);
            p.line(
                joystickCenter[0] - 10,
                joystickCenter[1],
                joystickCenter[0] + 10,
                joystickCenter[1],
            );
            p.line(
                joystickCenter[0],
                joystickCenter[1] - 10,
                joystickCenter[0],
                joystickCenter[1] + 10,
            );

            // draw a dotte cirle aroung the joystick center
            p.noFill();
            p.stroke(255, 255, 255, 100);
            p.strokeWeight(1);
            p.circle(joystickCenter[0], joystickCenter[1], joystickRadius * 2);

            p.fill(255);

            for (let i = 0; i < p.touches.length; i++) {
                const touch = p.touches[i] as any;

                // get diff vector from joystick center to touch
                const diff = [
                    touch.x - joystickCenter[0],
                    touch.y - joystickCenter[1],
                ];

                // limit diff vector to joystick area using trigonometry
                const diffMax = diff.map((v) =>
                    Math.min(Math.max(v, -joystickRadius), joystickRadius),
                );

                // draw diff vector
                p.stroke(255, 0, 0);
                p.strokeWeight(2);
                p.line(
                    joystickCenter[0],
                    joystickCenter[1],
                    joystickCenter[0] + diffMax[0],
                    joystickCenter[1] + diffMax[1],
                );

                // normalize diff vector
                const diffNorm = diffMax.map((v) => v / joystickRadius);

                p.noStroke();

                p.ellipse(
                    joystickCenter[0] + diffMax[0],
                    joystickCenter[1] + diffMax[1],
                    50,
                    50,
                );

                p.fill(0);
                p.text(touch.id, touch.x, touch.y);

                setJoyPadStore("movement", () => {
                    return [diffNorm[0], diffNorm[1]];
                });

                onChange({
                    movement: joyPadStore.movement,
                    attack: joyPadStore.attack,
                });
            }
        };
    };

    onMount(() => {
        setP5Instance(new p5(sketch));
    });

    onCleanup(() => {
        p5Instance()?.remove();
    });

    return (
        <div class={"flex flex-col flex-1"}>
            <div class={"flex"}>
                <div class={"flex-1 flex flex-col items-center text-white"}>
                    <span class={"text-xl text-grey mb-1"}>Colore</span>
                    <Splash color={playerStats.color} />
                </div>
            </div>

            <div class={"flex flex-col flex-1"}>
                <div id={"field"} ref={canvas}></div>
            </div>
        </div>
    );
};

export default JoyPad;
