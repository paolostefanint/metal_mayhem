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


socket.on("connect", () => {
    console.log("connected");
})

socket.on("message", (data) => {
    console.log(data);
});
