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

var fileInputElement = document.getElementById("file-input");
fileInputElement.addEventListener("change", e => fileInputElement.files[0].text().then((xmlText) => {
    console.log(xmlText);
    var xmlText = xmlText.replace(/\s/g,"");
    var parser = new DOMParser();
    var xmlDoc = parser.parseFromString(xmlText,"text/xml");
    var simulation = xmlDoc.getRootNode().childNodes[0];
    var landBounds = simulation.childNodes[0];
    var plants = simulation.childNodes[1];
    //plants.getElementsByTagName("INITIAL_PLANT_COUNT")[0].childNodes[0].nodeValue
    //always returns a string
    var grazers = simulation.childNodes[2];
    var predators = simulation.childNodes[3];
    var obstacles = simulation.childNodes[4];
    console.log(plants);
}));
