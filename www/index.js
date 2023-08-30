import { greet } from "life_simulation";

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
    plants.
    console.log(plants);
}));
