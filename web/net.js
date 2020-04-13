class Matrix {
    constructor(rows, cols) {
        this.rows = rows;
        this.cols = cols;
        this.data = [[]];
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
    if (a.rows != b.cols) {
        console.log("Multiply dimensions error");
    }
    let res = {};

}