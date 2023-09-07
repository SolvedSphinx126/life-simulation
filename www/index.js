import {Map} from "life_simulation";

var canvas = document.getElementById("game-canvas");
var ctx = canvas.getContext("2d");

const drawBg = () => {
    ctx.fillStyle = "#111111";
    ctx.fillRect(0,0,canvas.width,canvas.height);
}

const render = () => {
    drawBg();
}

const map = Map.new();

map.set_width(10);
console.log(map.get_width())
map.set_width(11);
console.log(map.get_width())

render()

