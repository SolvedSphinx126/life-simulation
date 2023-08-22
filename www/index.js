import * as wasm from "life_simulation";

var canvas = document.getElementById("game-canvas");
var ctx = canvas.getContext("2d");

const drawBg = () => {
    ctx.fillStyle = "#555555";
    ctx.fillRect(0,0,canvas.width,canvas.height);
}

const render = () => {
    drawBg();
}

render()

window.addEventListener('resize', () => {
    const parentWidth = document.querySelector('.parent-container').clientWidth;
    const parentHeight = document.querySelector('.parent-container').clientHeight;
  
    const container = document.querySelector('.container');
    let size = Math.min(parentWidth, parentHeight - 40)
    container.style.width = (size) + 'px'; // Adjust the factor as needed
    container.style.height = (size) + 'px'; // Adjust the factor as needed
});

window.dispatchEvent(new Event('resize'));