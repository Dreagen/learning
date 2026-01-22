const canvas = game;
console.log(canvas);

const WIDTH = 1000;
const HEIGHT = 1000;
canvas.width = WIDTH;
canvas.height = HEIGHT;

const ctx = canvas.getContext("2d");

function clear() {
    ctx.fillStyle = "#101010"
    ctx.fillRect(0, 0, 1000, 1000);
}

function line(startPoint, endPoint) {
    ctx.lineWidth = 3;
    ctx.strokeStyle = "green";
    ctx.beginPath();
    ctx.moveTo(startPoint.x, startPoint.y);
    ctx.lineTo(endPoint.x, endPoint.y);
    ctx.closePath();
    ctx.stroke();
}

function point({ x, y }) {
    const size = 20;

    ctx.fillStyle = "green"
    ctx.fillRect(x - (size / 2), y - (size / 2), size, size);
}

function transformToCanvasLayout({ x, y }) {
    x = (x + 1) / 2; // x comes in between -1 and 1 => now is between 0 and 1
    y = (y + 1) / 2; // y comes in between -1 and 1 => now is between 0 and 1
    x = x * WIDTH; // matched to width of canvas
    y = y * HEIGHT; // matched to height of canvas

    return { x: x, y: y }
}

function parse3dTo2d({ x, y, z }) {
    const newX = x / z;
    const newY = y / z;

    return { x: newX, y: newY }
}

const points = [
    { x: -0.25, y: -0.25, z: 1 },
    { x: 0.25, y: -0.25, z: 1 },
    { x: -0.25, y: 0.25, z: 1 },
    { x: 0.25, y: 0.25, z: 1 },

    { x: -0.25, y: -0.25, z: 2 },
    { x: 0.25, y: -0.25, z: 2 },
    { x: -0.25, y: 0.25, z: 2 },
    { x: 0.25, y: 0.25, z: 2 },
]

const faces = [
    [0, 1, 2, 3],
    [4, 5, 6, 7],
    [0, 4],
    [1, 5],
    [2, 6],
    [3, 7]
]

clear();

for (p of points) {
    point(transformToCanvasLayout(parse3dTo2d({ x: p.x, y: p.y, z: p.z })));
}

for (f of faces) {
    for (i of f) {
        console.log(i)
        line(
            transformToCanvasLayout(parse3dTo2d(points[i])),
            transformToCanvasLayout(parse3dTo2d(points[(i + 1) % ]))
        );
    }
}
