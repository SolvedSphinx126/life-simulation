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

var r = map.add_rock(1,10,15,7,9);
map.add_rock(2,10,15,7,8);

map.set_width(10);
map.set_height(10);

for(var rock of map.get_rocks()) {
    console.log(rock.get_height());
}

render()

