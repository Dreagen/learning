const FPS = 60
const BACKGROUND = "#101010"
const FOREGROUND = "#50FF50"
game.width = 1200
game.height = 1200
var ctx = game.getContext("2d");
console.log(ctx);

function clear() {
    ctx.fillStyle = BACKGROUND;
    ctx.fillRect(0, 0, game.width, game.height);
}

function renderPoint({ x, y }) {
    const s = 20;
    ctx.fillStyle = FOREGROUND;
    ctx.fillRect(x - s / 2, y - s / 2, s, s);
}

function renderLine(p1, p2) {
    ctx.lineWidth = 3;
    ctx.strokeStyle = FOREGROUND;
    ctx.beginPath();
    ctx.moveTo(p1.x, p1.y);
    ctx.lineTo(p2.x, p2.y);
    ctx.stroke();
}

function adjustForScreen(point) {
    x = (point.x + 1) / 2 * game.width
    y = (1 - (point.y + 1) / 2) * game.height

    return { x: x, y: y }
}

function projectTo2d({ x, y, z }) {
    return {
        x: x / z,
        y: y / z
    }
}

function translateZ(point, dz) {
    return { x: point.x, y: point.y, z: point.z + dz }
}

function translateY(point, dy) {
    return { x: point.x, y: point.y + dy, z: point.z }
}

function translateX(point, dx) {
    return { x: point.x + dx, y: point.y, z: point.z }
}

function rotateXZ({ x, y, z }, angle) {
    newX = x * Math.cos(angle) - z * Math.sin(angle)
    newZ = x * Math.sin(angle) + z * Math.cos(angle)
    newY = x * Math.sin(angle) + y * Math.cos(angle)

    return { x: newX, y: y, z: newZ };
}

const points = [
    { x: 0.25, y: 0.25, z: 0.25 },
    { x: 0.25, y: -0.25, z: 0.25 },
    { x: -0.25, y: -0.25, z: 0.25 },
    { x: -0.25, y: 0.25, z: 0.25 },

    { x: 0.25, y: 0.25, z: -0.25 },
    { x: 0.25, y: -0.25, z: -0.25 },
    { x: -0.25, y: -0.25, z: -0.25 },
    { x: -0.25, y: 0.25, z: -0.25 },
]

const faces = [
    [0, 1, 2, 3],
    [4, 5, 6, 7],
    [0, 4],
    [3, 7],
    [1, 5],
    [2, 6],
]

let dz = 1;
let dy = 0.1;
let dx = 0.1;
let angle = 0;
let counter = 0;

function frame() {
    clear();

    if (counter % 600 < 300) {
        dz += 1 / FPS
    } else {
        dz -= 1 / FPS
    }
    counter++;

    angle += Math.PI / FPS

    for (f of faces) {
        for (let i = 0; i < f.length; ++i) {
            const a = points[f[i]]
            const b = points[f[(i + 1) % f.length]]
            p1 = adjustForScreen(projectTo2d(translateZ(rotateXZ(a, angle), dz)));
            p2 = adjustForScreen(projectTo2d(translateZ(rotateXZ(b, angle), dz)));
            renderLine(p1, p2);
        }
    }

    setTimeout(frame, 1000 / FPS);
}

setTimeout(frame, 1000 / FPS);
