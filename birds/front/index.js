import * as sim from "lib-simulation-wasm";

const simulation = new sim.Simulation();
const world = simulation.world();

document.getElementById('train').onclick = function () {
    console.log(simulation.train());
};

const viewport = document.getElementById("viewport");
const context = viewport.getContext("2d");
// ---
// | Determines color of the upcoming shape.
// - v-------v
// context.fillStyle = 'rgb(255, 0, 0)';
// ------------------ ^-^ -^ -^
// | Each of those three parameters is a number from range 0 up to 255:
// |
// | rgb(0, 0, 0) = black
// |
// | rgb(255, 0, 0) = red
// | rgb(0, 255, 0) = green
// | rgb(0, 0, 255) = blue
// |
// | rgb(255, 255, 0) = yellow
// | rgb(0, 255, 255) = cyan
// | rgb(255, 0, 255) = magenta
// |
// | rgb(128, 128, 128) = gray
// | rgb(255, 255, 255) = white
// ---
// context.fillRect(10, 10, 100, 50);
// ---------- X   Y   W    H
// | Draws rectangle filled with color determined by `fillStyle`.
// |
// | X = position on the X axis (left-to-right)
// | Y = position on the Y axis (top-to-bottom)
// | W = width
// | X = height
// |
// | (unit: pixels)
// ---
const viewportScale = window.devicePixelRatio || 1;

const viewportWidth = viewport.width;
const viewportHeight = viewport.height;

viewport.width = viewportWidth * viewportScale;
viewport.height = viewportHeight * viewportScale;

viewport.style.width = viewportWidth + 'px';
viewport.style.height = viewportHeight + 'px';

context.scale(viewportScale, viewportScale);

context.fillStyle = 'rgb(0, 0, 0)';

CanvasRenderingContext2D.prototype.drawTriangle = function ({ x, y, size, rotation }) {
    this.beginPath();
    this.moveTo(
        x + Math.cos(rotation) * size * 1.5,
        y + Math.sin(rotation) * size * 1.5
    );
    this.lineTo(
        x + Math.cos(rotation + 2.0 / 3.0 * Math.PI) * size,
        y + Math.sin(rotation + 2.0 / 3.0 * Math.PI) * size
    );
    this.lineTo(
        x + Math.cos(rotation + 4.0 / 3.0 * Math.PI) * size,
        y + Math.sin(rotation + 4.0 / 3.0 * Math.PI) * size
    );
    this.lineTo(
        x + Math.cos(rotation) * size * 1.5,
        y + Math.sin(rotation) * size * 1.5
    );

    this.fillStyle = 'rgb(255,255,255)';
    this.fill();
    this.fillStyle = 'rgb(0, 0, 0)';
    this.stroke();
};

CanvasRenderingContext2D.prototype.drawCircle =
    function (x, y, radius) {
        this.beginPath();
        this.arc(x, y, radius, 0, 2.0 * Math.PI);

        this.fillStyle = 'rgb(0, 255, 128)';
        this.fill();
    };

function drawFrame() {
    for (const animal of simulation.world().animals) {
        context.drawTriangle({
            x: animal.x * viewportWidth,
            y: animal.y * viewportHeight,
            size: 0.015 * viewportWidth,
            rotation: animal.rotation
        });
    }

    for (const food of simulation.world().foods) {
        context.drawCircle(
            food.x * viewportWidth,
            food.y * viewportHeight,
            (0.01 / 2.0) * viewportWidth,
        );
    }
}

function redraw() {
    context.clearRect(0, 0, viewportWidth, viewportHeight);
    simulation.step();
    drawFrame();
    window.requestAnimationFrame(redraw);
}

redraw();