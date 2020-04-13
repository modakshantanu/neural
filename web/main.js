
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
    let dx = [-1,0,1,0];
    let dy = [0,1,0,-1];
    let ddx = [-1,-1,1,1];
    let ddy = [1,-1,1,-1];


    if (mouseIsPressed && mouseX < 280 && mouseX > 0 && mouseY < 280 && mouseY > 0) {
        let y = Math.floor(mouseX / 10);
        let x = Math.floor(mouseY / 10);
        canvasState[x][y] = Math.max(canvasState[x][y], Math.random() / 2 + 0.5);
        for (let i = 0; i < 2; i++) {
            let nx = x + dx[i];
            let ny = y + dy[i];
            if (nx < 0 || nx >= 28 || ny < 0 || ny >= 28) continue;
            canvasState[nx][ny] = Math.max(canvasState[nx][ny], Math.random()/2 + 0.5);
        }
        for (let i = 0; i < 1; i++) {
            let nx = x + ddx[i];
            let ny = y + ddy[i];
            if (nx < 0 || nx >= 28 || ny < 0 || ny >= 28) continue;
            canvasState[nx][ny] = Math.max(canvasState[nx][ny], Math.random()/6 + 0.2);
        }
    }
    let array = canvasToArray();
    if (network !== null) {

        let result = evaluate(network, array);
        result = result.map((v,i) => ({val: v, idx: i}));
        
        result.sort((a,b) => b.val - a.val);
        setOutput(result);
        // resultText.innerText = highestIdx;
    }

    


    // render
    for (let i = 0; i < 28; i++) {
        for (let j = 0; j < 28; j++) {
            let level = 1 - canvasState[j][i];
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

function setOutput(result) {
    let fullscale = result[0].val;
    let resultHtml = "";
    result.forEach((e) => {
        let fontsize = Math.log(fullscale) / Math.log(e.val) * 50;
        resultHtml += `<span style="font-size:${fontsize}px">${e.idx}</span>`;
    });
    resultText.innerHTML = resultHtml;
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
