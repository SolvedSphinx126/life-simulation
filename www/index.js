import * as wasm from "life_simulation";

var canvas = document.getElementById("game-canvas");
var ctx = canvas.getContext("2d");
canvas.width = 500;
canvas.height = 500;

const drawBg = () => {
    ctx.fillStyle = "#EEEEEE";
    ctx.fillRect(0,0,canvas.width,canvas.height);
}

const render = () => {
    drawBg();
}

render()