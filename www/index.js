import { Universe , Cell} from "wasm-game-of-life";
// Import the WebAssembly memory at the top of the file.
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const fps = new (class {
    
    constructor() {
        this.fps = document.getElementById("fps");
        this.frames = [];
        this.lastFrameTimestamp = performance.now();
    }

    render() {
        const now = performance.now();
        const delta = now - this.lastFrameTimestamp;
        this.lastFrameTimestamp = now;
        const fps = (1 / delta) * 1000;

        this.frames.push(fps);
        if (this.frames.length > 100) {
            this.frames.shift();
        }

        let min = Infinity;
        let max = -Infinity;
        let sum = 0;
        for (let i = 0; i < this.frames.length; i++) {
            min = Math.min(this.frames[i], min);
            max = Math.max(this.frames[i], max);
            sum += this.frames[i];
        }
        let mean = sum / this.frames.length;

        this.fps.textContent = `
  Frames per second (fps):
           latest = ${Math.round(fps)}
  avg of last 100 = ${Math.round(mean)}
  min of last 100 = ${Math.round(min)}
  max of last 100 = ${Math.round(max)}
  `.trim();
    }
})();


const CELL_SIZE = 8; // px

const LINE_WIDTH = 1.0;
const GRID_COLOR = 'rgb(200.0, 200.0, 200.0)';
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

// Construct the universe, and get its width and height.
const universe = Universe.new();
const no_cells_wide = universe.width();
const no_cells_deap = universe.height();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * no_cells_deap + 1;
canvas.width = (CELL_SIZE + 1) * no_cells_wide + 1;

// var params = { width: canvas.width, height: canvas.height, type: Two.Types.webgl };
// var two = new Two(params).appendTo(canvas);

let animationId = null;
let cells_rects = []



const renderLoop = () => {
    fps.render();
    //debugger;
    universe.tick();

    // Draw it
    universe.render_webgl();
   
    animationId = requestAnimationFrame(renderLoop);
};

const isPaused = () => {
    return animationId === null;
};

const playPauseButton = document.getElementById("play-pause");

const play = () => {
    playPauseButton.textContent = "⏸";
    renderLoop();
};

const pause = () => {
    playPauseButton.textContent = "▶";
    cancelAnimationFrame(animationId);
    animationId = null;
};

playPauseButton.addEventListener("click", event => {
    if (isPaused()) {
        play();
    } else {
        pause();
    }
});


// const drawGrid = () => {
//     // Vertical lines.
//     console.log("cells wide",no_cells_wide);
//     for (let i = 0; i <= no_cells_wide; i++) {
//         let line = two.makeLine(i * (CELL_SIZE + 1) + 1, 0, i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * no_cells_deap + 1);
//         line.stroke = GRID_COLOR;
//         line.linewidth = LINE_WIDTH;
//     }

//     // Horizontal lines.
//     for (let j = 0; j <= no_cells_deap; j++) {
//         let line = two.makeLine(0, j * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * no_cells_wide + 1, j * (CELL_SIZE + 1) + 1);
//         line.stroke = GRID_COLOR;
//         line.linewidth = LINE_WIDTH;
//     }

//     for (let row = 0; row < no_cells_deap; row++) {
//         for (let col = 0; col < no_cells_wide; col++) {
//             const idx = getIndex(row, col);

//             var rect = two.makeRectangle(col * (CELL_SIZE + 1) + 1, row * (CELL_SIZE + 1) + 1, CELL_SIZE-1, CELL_SIZE-1);
//             rect.noStroke().fill = 'rgb(0, 200, 255)';
//             cells_rects.push(rect)
//         }
//     }
   
// }

const getIndex = (row, column) => {
    return row * no_cells_wide + column;
};

// const drawCells = () => {
//     const cellsPtr = universe.cells();
//     const cells = new Uint8Array(memory.buffer, cellsPtr, no_cells_wide * no_cells_deap);

//     //ctx.beginPath();

//     // Alive cells.
//     //ctx.fillStyle = ALIVE_COLOR;
//     for (let row = 0; row < no_cells_deap; row++) {
//         for (let col = 0; col < no_cells_wide; col++) {
//             const idx = getIndex(row, col);
//             if (cells[idx] !== Cell.Alive) {
  
//                 continue;
//             }
//             cells_rects[idx].fill = 'rgb(0, 0, 0)';
//             cells_rects[idx].rotation = 0.0;
           
//         }
//     }
//     // Dead cells.
//     //ctx.fillStyle = DEAD_COLOR;
//     for (let row = 0; row < no_cells_deap; row++) {
//         for (let col = 0; col < no_cells_wide; col++) {
//             const idx = getIndex(row, col);
//             if (cells[idx] !== Cell.Dead) {
//                 continue;
//             }
//             cells_rects[idx].fill = 'rgb(0, 200, 255)';
//             cells_rects[idx].rotation = Math.PI / 4;
//         }
//     }
// };

canvas.addEventListener("click", event => {
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), no_cells_deap - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), no_cells_wide - 1);

    universe.toggle_cell(row, col);

    universe.render_webgl();
});



  // This used to be `requestAnimationFrame(renderLoop)`.
play();