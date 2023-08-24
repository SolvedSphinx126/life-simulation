import * as wasm from "life_simulation";

var canvas = document.getElementById("game-canvas");
var ctx = canvas.getContext("2d");

const drawBg = () => {
    ctx.fillStyle = "#111111";
    ctx.fillRect(0,0,canvas.width,canvas.height);
}

const render = () => {
    drawBg();
}

render()

