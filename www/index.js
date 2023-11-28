import {Map} from "life_simulation";

var canvas = document.getElementById("game-canvas");
var ctx = canvas.getContext("2d");


const drawCircle = (x, y, d, color) => {
    x = (x / map.get_width()) * ctx.canvas.width;
    y = (y / map.get_height()) * ctx.canvas.height;
    x = x;
    y = ctx.canvas.height - y;
    ctx.beginPath();
    ctx.fillStyle = color;
    ctx.arc(x, y, d, 0, 2 * Math.PI);
    ctx.fill();
    ctx.stroke();
}

const drawMover = (x, y, d, orientation, color) => {
    x = (x / map.get_width()) * ctx.canvas.width;
    y = (y / map.get_height()) * ctx.canvas.height;
    x = x;
    y = ctx.canvas.height - y;
    ctx.beginPath();
    ctx.save();
    ctx.translate(x, y);
    ctx.rotate(-(orientation - Math.PI / 2));
    ctx.translate(-x, -y);
    ctx.fillStyle = color;
    ctx.arc(x, y, d, 0, Math.PI);
    ctx.lineTo(x, y - (1.1 * d));
    ctx.closePath();
    ctx.fill();
    ctx.stroke();
    ctx.restore();
}

const drawRocks = () => {
    for (var rock of map.get_rocks()) {
        drawCircle(rock.get_entity().get_x(), rock.get_entity().get_y(), rock.get_diameter(), 'grey')
    }
}

const drawPlants = () => {
    for (var plant of map.get_plants()) {
        drawCircle(plant.get_entity().get_x(), plant.get_entity().get_y(), plant.get_diameter(), 'green')
    }
}

const drawPredators = () => {
    for (var pred of map.get_predators()) {
        drawMover(pred.get_entity().get_x(), pred.get_entity().get_y(), 10, pred.get_mover().get_orientation(), 'red')
    }
}

const drawGrazers = () => {
    for (var grazer of map.get_grazers()) {
        drawMover(grazer.get_entity().get_x(), grazer.get_entity().get_y(), 10, grazer.get_mover().get_orientation(), 'blue')
    }
}

var msToWait = 1000;
function setWait(msWait) {
    msToWait = msWait;
}

var completedSimulationList = [];

const render = async () => {
    ctx.canvas.width = ctx.canvas.clientWidth
    ctx.canvas.height = ctx.canvas.clientHeight
    ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height)
    // perform calculation for movement functions before any entities are drawn

    drawPlants();
    drawPredators();
    drawGrazers();
    drawRocks();
    map.tick();
    let predCount = document.getElementById("predCount");
    predCount.innerHTML = map.get_predators().length;
    let grazerCount = document.getElementById("grazerCount");
    grazerCount.innerHTML = map.get_grazers().length;
    let plantCount = document.getElementById("plantCount");
    plantCount.innerHTML = map.get_plants().length;

    if (map.get_predators().length == 0 || map.get_grazers().length == 0 || map.get_plants().length == 0) {
        document.getElementById("generate report").click();
        var score = map.score();
        var filename = map.generate_report_file_name();
        map = undefined;
        document.getElementById("file-input").value = null;

        var leaderboardList = document.getElementById("leaderboard-list");

        completedSimulationList.push([score, filename]);

        completedSimulationList.sort((a, b) => b[0] - a[0]);

        // Clear existing list items
        leaderboardList.innerHTML = "";

        // Create list items for the leaderboard
        for (let i = 0; i < Math.min(completedSimulationList.length, 10); i++) {
            var listItem = document.createElement("li");
            listItem.textContent = `${completedSimulationList[i][1]}: ${completedSimulationList[i][0]}`;
            leaderboardList.appendChild(listItem);
        }
    } else {
        await new Promise(r => setTimeout(r, msToWait));
        requestAnimationFrame(render);
    }
}

var map;

var fileInputElement = document.getElementById("1x");
fileInputElement.addEventListener("click", e => {
    setWait(1000)
})

var fileInputElement = document.getElementById("10x");
fileInputElement.addEventListener("click", e => {
    setWait(100);
})

var fileInputElement = document.getElementById("50x");
fileInputElement.addEventListener("click", e => {
    setWait(20);
})

var fileInputElement = document.getElementById("100x");
fileInputElement.addEventListener("click", e => {
    setWait(10);
})

var genReportButton = document.getElementById("generate report");
genReportButton.addEventListener("click", e => {
    //get filename
    var filename = map.generate_report_file_name();

    //get data from sim
    var mapData = map.generate_report();

    //create file trying blob
    var blob = new Blob([mapData], {type: "text/plain",});

    const blobURL = URL.createObjectURL(blob);

    const link = document.createElement("a");

    link.href = blobURL;
    link.download = filename;

    document.body.appendChild(link);

    link.dispatchEvent(
        new MouseEvent('click',{
            bubbles: true,
            cancelable: true,
            view: window
        })
    );

   document.body.removeChild(link);

})

var fileInputElement = document.getElementById("file-input");
fileInputElement.addEventListener("change", e => fileInputElement.files[0].text().then((xmlText) => {
    map = Map.new();
    var xmlText = xmlText.replace(/\s/g,"");
    var parser = new DOMParser();
    var xmlDoc = parser.parseFromString(xmlText,"text/xml");
    var simulation = xmlDoc.getRootNode().childNodes[0];
    var landBounds = simulation.childNodes[0];
    var plants = simulation.childNodes[1];
    var grazers = simulation.childNodes[2];
    var predators = simulation.childNodes[3];
    var obstacles = simulation.childNodes[4];

    // map inputs
    map.set_width(parseFloat(landBounds.getElementsByTagName("WIDTH")[0].childNodes[0].nodeValue));
    map.set_height(parseFloat(landBounds.getElementsByTagName("HEIGHT")[0].childNodes[0].nodeValue));

    // plant metadata
    map.set_init_plant_count(parseInt(plants.getElementsByTagName("INITIAL_PLANT_COUNT")[0].childNodes[0].nodeValue));
    map.set_growth_rate(parseFloat(plants.getElementsByTagName("GROWTH_RATE")[0].childNodes[0].nodeValue));
    map.set_max_size(parseInt(plants.getElementsByTagName("MAX_SIZE")[0].childNodes[0].nodeValue));
    map.set_max_seed_cast_distance(parseInt(plants.getElementsByTagName("MAX_SEED_CAST_DISTANCE")[0].childNodes[0].nodeValue));
    map.set_max_seed_number(parseInt(plants.getElementsByTagName("MAX_SEED_NUMBER")[0].childNodes[0].nodeValue));
    map.set_max_size(parseFloat(plants.getElementsByTagName("MAX_SIZE")[0].childNodes[0].nodeValue));
    map.set_seed_viability(parseFloat(plants.getElementsByTagName("SEED_VIABILITY")[0].childNodes[0].nodeValue));

    plants = plants.getElementsByTagName("PLANT");
    for (var plant of plants) {
        var plantX = parseInt(plant.getElementsByTagName("X_POS")[0].childNodes[0].nodeValue);
        var plantY = parseInt(plant.getElementsByTagName("Y_POS")[0].childNodes[0].nodeValue);
        var plantD = parseInt(plant.getElementsByTagName("P_DIAMETER")[0].childNodes[0].nodeValue);
        map.add_plant(plantX, plantY, plantD)
    }

    // grazer metadata
    map.set_init_grazer_count(parseInt(grazers.getElementsByTagName("INITIAL_GRAZER_COUNT")[0].childNodes[0].nodeValue));
    map.set_grazer_energy_input(parseInt(grazers.getElementsByTagName("G_ENERGY_INPUT")[0].childNodes[0].nodeValue));
    map.set_grazer_energy_output(parseInt(grazers.getElementsByTagName("G_ENERGY_OUTPUT")[0].childNodes[0].nodeValue));
    map.set_grazer_energy_to_reproduce(parseInt(grazers.getElementsByTagName("G_ENERGY_TO_REPRODUCE")[0].childNodes[0].nodeValue));
    map.set_grazer_maintain_speed(parseFloat(grazers.getElementsByTagName("G_MAINTAIN_SPEED")[0].childNodes[0].nodeValue));
    map.set_grazer_max_speed(parseFloat(grazers.getElementsByTagName("G_MAX_SPEED")[0].childNodes[0].nodeValue));

    grazers = grazers.getElementsByTagName("GRAZER");
    for (var grazer of grazers) {
        var grazerX = parseInt(grazer.getElementsByTagName("X_POS")[0].childNodes[0].nodeValue);
        var grazerY = parseInt(grazer.getElementsByTagName("Y_POS")[0].childNodes[0].nodeValue);
        var grazerEnergy = parseInt(grazer.getElementsByTagName("G_ENERGY_LEVEL")[0].childNodes[0].nodeValue);
        map.add_grazer(grazerX, grazerY, grazerEnergy)
    }

    // predator metadata
    map.set_init_predator_count(parseInt(predators.getElementsByTagName("INITIAL_PREDATOR_COUNT")[0].childNodes[0].nodeValue));
    map.set_max_speed_hod(parseFloat(predators.getElementsByTagName("MAX_SPEED_HOD")[0].childNodes[0].nodeValue));
    map.set_max_speed_hed(parseFloat(predators.getElementsByTagName("MAX_SPEED_HED")[0].childNodes[0].nodeValue));
    map.set_max_speed_hor(parseFloat(predators.getElementsByTagName("MAX_SPEED_HOR")[0].childNodes[0].nodeValue));
    map.set_predator_maintain_speed(parseFloat(predators.getElementsByTagName("P_MAINTAIN_SPEED")[0].childNodes[0].nodeValue));
    map.set_predator_energy_output(parseInt(predators.getElementsByTagName("P_ENERGY_OUTPUT")[0].childNodes[0].nodeValue));
    map.set_predator_energy_to_reproduce(parseInt(predators.getElementsByTagName("P_ENERGY_TO_REPRODUCE")[0].childNodes[0].nodeValue));
    map.set_predator_max_offspring(parseInt(predators.getElementsByTagName("P_MAX_OFFSPRING")[0].childNodes[0].nodeValue));
    map.set_predator_gestation(parseFloat(predators.getElementsByTagName("P_GESTATION")[0].childNodes[0].nodeValue));
    map.set_predator_offspring_energy(parseInt(predators.getElementsByTagName("P_OFFSPRING_ENERGY")[0].childNodes[0].nodeValue));

    predators = predators.getElementsByTagName("PREDATOR");
    for (var predator of predators) {
        var predatorX = parseInt(predator.getElementsByTagName("X_POS")[0].childNodes[0].nodeValue);
        var predatorY = parseInt(predator.getElementsByTagName("Y_POS")[0].childNodes[0].nodeValue);
        var predatorEnergy = parseInt(predator.getElementsByTagName("P_ENERGY_LEVEL")[0].childNodes[0].nodeValue);
        var genes = predator.getElementsByTagName("GENOTYPE")[0].childNodes[0].nodeValue;
        map.add_predator(predatorX, predatorY, predatorEnergy, genes);
    }

    var rocks = obstacles.getElementsByTagName("OBSTACLE");
    for (var rock of rocks) {
        var rockX = parseInt(rock.getElementsByTagName("X_POS")[0].childNodes[0].nodeValue);
        var rockY = parseInt(rock.getElementsByTagName("Y_POS")[0].childNodes[0].nodeValue);
        var rockD = parseInt(rock.getElementsByTagName("O_DIAMETER")[0].childNodes[0].nodeValue);
        var rockH = parseInt(rock.getElementsByTagName("O_HEIGHT")[0].childNodes[0].nodeValue);
        map.add_rock(rockX, rockY, rockD, rockH);
    }

    console.log(map)
    console.log(map.get_grazers())
    console.log(map.get_rocks())
    console.log(map.get_predators())
    console.log(map.get_plants())

    render()
}));
