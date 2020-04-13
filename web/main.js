
var oReq = new XMLHttpRequest();
oReq.open("GET", "https://cors-anywhere.herokuapp.com/https://github.com/modakshantanu/neural/blob/master/new.ncf?raw=true", true);
oReq.responseType = "arraybuffer";
var data,view;
var network;

oReq.onload = function (oEvent) {
    data = oReq.response; // Note: not oReq.responseText
    if (data) {
        view = new DataView(data);
        data = new Uint8Array(data);
        network = netFromArray();
        loading = false;
    }
};

let canvasState = [];

function initCanvasState() {
    for (let i = 0; i < 28; i++) {
        let row = [];
        for (let j = 0; j < 28; j++) {
            row.push(0.0);
        }
        canvasState.push(row);
    }

}

function setup() {
    
    oReq.send(null);
    createCanvas(280, 280).parent('canvas');
    stroke(255);
    strokeWeight(0);	
    initCanvasState();
    
}
let loading = true;

function draw() {
    if (loading) return;
    // update
    let dx = [-1,1,0,0];
    let dy = [0,0,1,-1];
    let ddx = [-1,-1,1,1];
    let ddy = [1,-1,1,-1];


    if (mouseIsPressed && mouseX < 280 && mouseX > 0 && mouseY < 280 && mouseY > 0) {
        let x = Math.floor(mouseX / 10);
        let y = Math.floor(mouseY / 10);
        canvasState[x][y] = 1;
        for (let i = 0; i < 4; i++) {
            let nx = x + dx[i];
            let ny = y + dy[i];
            if (nx < 0 || nx >= 28 || ny < 0 || ny >= 28) continue;
            canvasState[nx][ny] = Math.max(canvasState[nx][ny], Math.random()/2 + 0.5);
        }
        for (let i = 0; i < 4; i++) {
            let nx = x + ddx[i];
            let ny = y + ddy[i];
            if (nx < 0 || nx >= 28 || ny < 0 || ny >= 28) continue;
            canvasState[nx][ny] = Math.max(canvasState[nx][ny], Math.random()/4 + 0.25);
        }
    }
    let array = canvasToArray();

    if (network !== null) {

        let result = evaluate(network, array);
    
        let highestIdx = 0;
        for (let i = 0; i < 10; i++) {
            if (result[i] > result[highestIdx]) {
                highestIdx = i;
            }
        }
        resultText.innerText = highestIdx;
    }

    


    // render
    for (let i = 0; i < 28; i++) {
        for (let j = 0; j < 28; j++) {
            let level = 1 - canvasState[i][j];
            fill(level * 255);
            rect(10*i, 10*j, 10, 10);
        }
    }
}

function canvasToArray() {
    let array = [];
    for (let i = 0; i < 28; i++) {
        for (let j = 0; j < 28; j++) {
            array.push(canvasState[i][j]);
        }
    }
    return array;
}

let resultText;
window.onload = () => {
    let clearButton = document.getElementById('clear-button');
    resultText = document.getElementById('answer');
    clearButton.onclick = () => {
        for (let i = 0; i < 28; i++) {
            for (let j = 0; j < 28; j++) {
                canvasState[i][j] = 0;
            }
        }
    }

}
