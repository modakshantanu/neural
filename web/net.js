class Matrix {
    constructor(rows, cols) {
        this.rows = rows;
        this.cols = cols;
        this.data = [];
        for (let i = 0; i < rows; i++) {
            let row = [];
            for (let j = 0; j < cols; j++) {
                row.push(0);
            }
            this.data.push(row);
        }
    }
}

function matmul(a, b) {
    if (a.cols != b.rows) {
        console.log("Multiply dimensions error");
        console.log(a,b);
    }
    let res = new Matrix(a.rows, b.cols);
    for (let i = 0; i < a.rows; i++) {
        for (let j = 0; j < b.cols; j++) {
            let sum = 0;
            for (let k = 0; k < a.cols; k++) {
                sum += a.data[i][k] * b.data[k][j];
            }
            res.data[i][j] = sum;
        }
    }
    return res;
}

function matadd(a, b) {
    if (a.rows != b.rows || a.cols != b.cols) {
        console.log("Add dimension error");
        console.log(a,b);
    }
    let res = new Matrix(a.rows, a.cols);
    for (let i = 0; i < a.rows; i++) {
        for (let j = 0; j < a.cols; j++) {
            res.data[i][j] = a.data[i][j] + b.data[i][j];
        }
    }
    return res;
}

function sigmoid(a) {
    let res = new Matrix(a.rows, a.cols);
    for (let i = 0; i < a.rows; i++) {
        for (let j = 0; j < a.cols; j++) {
            res.data[i][j] = 1/(1+Math.exp(-a.data[i][j]));
        }
    }
    return res;
}

function matrixFromArray(startIndex) {
    let index = startIndex;
    let num;
    ({num, index} = toInt(index));
    let rows = num;
    ({num, index} = toInt(index));
    let cols = num;
    let res = new Matrix(rows, cols);
    for (let i = 0; i < rows; i++) {
        for (let j = 0; j < cols; j++) {
            ({num, index} = toFloat(index));
            res.data[i][j] = num;
        }
    }
    return {res, index};
}

function toInt(startIndex) {
    
    let res = 0;
    for (let i = 0; i < 4; i++) {
        res *= 256;
        res += data[startIndex + i];
    }
    return {num: res, index: startIndex + 4};
}

function toFloat(startIndex) {
    return {num: view.getFloat64(startIndex, false), index: startIndex + 8};
}


class Network {
    constructor(layers, layer_sizes, w, b) {
        this.layers = layers;
        this.layer_sizes = layer_sizes;
        this.w = w;
        this.b = b;
    }
}

// Convert an array of bytes to a network
function netFromArray() {
    let index = 0;
    let num, res, net;
    ({num, index} = toInt(index));
    let layers = num;
    let layer_sizes = [];
    for (let i = 0; i < layers; i++) {
        ({num, index} = toInt(index));
        layer_sizes.push(num);
    }
    let w = [];
    let b = [];
    for (let i = 0; i < layers - 1; i++) {
        ({res , index} = matrixFromArray(index));
        w.push(res);
        ({res , index} = matrixFromArray(index));
        b.push(res);
    }

    return new Network(layers, layer_sizes, w, b);
}
// input is a list of 784 nums
function evaluate(network, input) {
    let activation = new Matrix(input.length, 1);
    for (let i = 0; i < input.length; i++) {
        activation.data[i][0] = input[i];
    }

    for (let i = 0; i < network.layers - 1; i++) {
        activation = sigmoid(matadd(matmul(network.w[i], activation) , network.b[i]));
    }
    return activation.data.map(e => e[0]);
}

function testrun() {
    let input = [];
    for (let i = 0; i < 784; i++) input.push(Math.random() - 0.5);
    let net = netFromArray();
    return evaluate(net, input);


}