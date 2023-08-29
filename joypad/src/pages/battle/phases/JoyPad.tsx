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
        let joystickCenter: [number, number] = [150, 150];
        let joystickRadius = 100;

        let buttonCenter = [150, 400];

        let canvasWidth = 300;
        let canvasHeight = 500;

        let inputs: JoypadState = {
            movement: [0, 0],
            attack: false,
        };

        let orientation = screen.orientation.type;

        const field = document.getElementById("field") as HTMLDivElement;

        const isPortrait = () => screen.orientation.type.startsWith("portrait");
        const isLandscape = () =>
            screen.orientation.type.startsWith("landscape");

        const handleJoystickTouch = (touch: any) => {
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

            // p.fill(0);
            // p.text(touch.id, touch.x, touch.y);

            inputs.movement = [diffNorm[0], diffNorm[1]];
        };

        const handleButtonTouch = (touch: any) => {
            inputs.attack = true;
        };

        const resetInputs = () => {
            inputs = {
                movement: [0, 0],
                attack: false,
            };
        };

        const drawJoypadBase = () => {
            if (isPortrait()) {
                joystickCenter = [canvasWidth / 2, 150];
                joystickRadius = canvasWidth / 4;
                buttonCenter = [canvasWidth / 2, canvasHeight - 100];
            } else {
                // landscape
                joystickCenter = [canvasWidth / 4, canvasHeight / 2];
                joystickRadius = canvasHeight / 4;
                buttonCenter = [
                    canvasWidth / 2 + canvasWidth / 4,
                    canvasHeight / 2,
                ];
            }
        };

        p.setup = () => {
            canvasWidth = field.offsetWidth;
            canvasHeight = field.offsetHeight;

            canvas = p.createCanvas(canvasWidth, canvasHeight);
            canvas.parent("field");

            drawJoypadBase();

            p.background(51);
            p.frameRate(10);
        };

        p.windowResized = () => {
            console.log("resized");
            canvasWidth = field.offsetWidth;
            canvasHeight = field.offsetHeight;
            p.resizeCanvas(canvasWidth, canvasHeight);
            drawJoypadBase();
        };

        p.draw = () => {
            resetInputs();
            p.background(51);

            drawJoystickArea(p, joystickCenter, joystickRadius);

            drawButtons(p, buttonCenter);

            p.fill(255);

            for (let i = 0; i < p.touches.length; i++) {
                const touch = p.touches[i] as any;

                if (isPortrait()) {
                    if (touch.y < canvasHeight / 2) {
                        // joystick area
                        console.log("joystick area");
                        handleJoystickTouch(touch);
                    }

                    if (touch.y > canvasHeight / 2) {
                        // button area
                        console.log("button area");
                        handleButtonTouch(touch);
                    }
                }
                if (isLandscape()) {
                    if (touch.x < canvasWidth / 2) {
                        // joystick area
                        console.log("joystick area");
                        handleJoystickTouch(touch);
                    }

                    if (touch.x > canvasWidth / 2) {
                        // button area
                        console.log("button area");
                        handleButtonTouch(touch);
                    }
                }
            }

            onChange({
                movement: inputs.movement,
                attack: inputs.attack,
            });
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
            {/*
            <div class={"flex"}>
                <div class={"flex-1 flex flex-col items-center text-white"}>
                    <span class={"text-xl text-grey mb-1"}>Colore</span>
                    <Splash color={playerStats.color} />
                </div>
            </div>
            */}

            <div class={"flex flex-col flex-1"}>
                <div id={"field"} ref={canvas} class={"min-h-full"}></div>
            </div>
        </div>
    );
};

function drawJoystickArea(
    p: p5,
    joystickCenter: number[],
    joystickRadius: number,
) {
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

    // draw a circle to indicate joystick area
    p.noFill();
    p.stroke(255, 255, 255, 100);
    p.strokeWeight(1);
    p.circle(joystickCenter[0], joystickCenter[1], joystickRadius * 2);
}

function drawButtons(p: p5, buttonCenter: number[]) {
    // draw a circle to indicate joystick area
    p.noFill();
    p.stroke(255, 255, 255, 100);
    p.strokeWeight(1);
    p.circle(buttonCenter[0], buttonCenter[1], 50);
}

export default JoyPad;
