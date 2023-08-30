const url = new URL(window.location.href);
const HOSTNAME = `${url.protocol}//${url.hostname}${
    url.port ? `:${url.port}` : ""
}`;
const HOSTNAME_NO_PROTOCOL = `${url.hostname}${url.port ? `:${url.port}` : ""}`;

//
// DEV INIT
//
const MULTIPLAYER_HOST = `ws://localhost:7001`;
const socket = io(`${MULTIPLAYER_HOST}`);

//
// PRODUCTION INIT
//
// const MULTIPLAYER_HOST = `${HOSTNAME}`;
// const socket = io(`${MULTIPLAYER_HOST}`, {
//     path: "/viewersocket/socket.io",
// });

let gameState = {
    game: {},
    players: [],
};
let RENDERING_SCALE = 5;

socket.on("connect", () => {
    console.log("connected");
});

socket.on("message", (data) => {
    gameState = JSON.parse(data);
    console.log(data);
});

function setup() {
    createCanvas(500, 500);
    background(0);
    frameRate(10);
}

function draw() {

    background(81);
    noStroke();

    scale()
    drawTimer();


    scale(RENDERING_SCALE);
    gameState.players.forEach((player) => {
        drawPlayer(player);
    });

}

function drawTimer() {
    fill(255);
    textSize(20);
    textAlign(CENTER);
    text(gameState.game.remainingTime, 480, 20);
}

function drawPlayer(player) {
    fill(player.color);
    noStroke();
    rect(player.position.x, player.position.y, 1, 0.5);

    // Draw a green life bar on top of the player
    fill(0, 255, 0);
    rect(player.position.x, player.position.y - 5, player.life / 10, 1);
}
