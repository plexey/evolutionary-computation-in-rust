import fs from "fs";
import readline from "readline";
import { createCanvas, Canvas, NodeCanvasRenderingContext2D } from "canvas";
import { Headers, Point } from "./types";
import fsPromises from "fs/promises";

type CTX = NodeCanvasRenderingContext2D;

const BACKGROUND_FILL = "hsl(220, 50%, 8%)";
const START_FILL = "hsla(200, 100%, 50%, 1)";
const END_FILL = "hsla(0, 100%, 50%, 1)";

const getCanvas = (width: number, height: number): Canvas => {
  return createCanvas(width, height);
};

const getCanvasContext = (canvas: Canvas): NodeCanvasRenderingContext2D => {
  return canvas.getContext("2d");
};

const generateColorOptions = () => {
  const colorOptions: string[] = [];
  let i = 0;
  while (i < 360) {
    const color = `hsla(${i}, 100%, 60%, 0.75)`;
    colorOptions.push(color);
    i += 20;
  }
  return colorOptions;
};

const colorOptions = generateColorOptions();

const drawStart = (ctx: CTX, point: Point) => {
  ctx.fillStyle = START_FILL;
  ctx.fillRect(point.x - 10, point.y - 10, 20, 20);
};

const drawGoal = (ctx: CTX, point: Point) => {
  ctx.fillStyle = END_FILL;
  ctx.fillRect(point.x - 10, point.y - 10, 20, 20);
};

const saveCanvasImage = async (canvas: Canvas) => {
  const buffer = canvas.toBuffer("image/png");
  fsPromises.writeFile(`../data/images/${Date.now().toString()}.png`, buffer);
};

const drawPath = (
  ctx: CTX,
  pathStart: Point,
  path: Point[],
  pathColor: string
) => {
  // DRAW EDGES
  ctx.strokeStyle = pathColor;
  ctx.beginPath();
  ctx.moveTo(pathStart.x, pathStart.y);

  path.forEach((point) => {
    ctx.lineTo(point.x, point.y);
  });

  ctx.stroke();

  // DRAWN NODES
  ctx.beginPath();
  ctx.moveTo(pathStart.x, pathStart.y);

  path.forEach((point) => {
    ctx.fillStyle = pathColor;
    ctx.beginPath();
    ctx.arc(point.x, point.y, 2.5, 0, 2.5 * Math.PI, false);
    ctx.fill();
  });
};

// const drawMetadata = (ctx: CTX, worldWidth: number, metadata: Metadata) => {
//   ctx.fillStyle = "#fff";
//   ctx.font = "16px sans serif";
//   ctx.fillText(`Generations: ${metadata.generations}`, worldWidth - 250, 50);
// };

const predraw = (ctx: CTX, worldWidth: number, worldHeight: number) => {
  ctx.fillStyle = BACKGROUND_FILL;
  ctx.fillRect(0, 0, worldWidth, worldHeight);
};

const run = async () => {
  let canvas: Canvas;
  let ctx: NodeCanvasRenderingContext2D;

  let worldHeight: number;
  let worldWidth: number;
  let start: Point;
  let goal: Point;

  const rl = readline.createInterface({
    input: fs.createReadStream("../data/paths.txt"),
    crlfDelay: Infinity,
  });

  let i = 0;

  for await (const line of rl) {
    const data = JSON.parse(line);

    if (data?.type === "headers") {
      const headers: Headers = data;

      console.log({ headers });

      worldHeight = headers.worldHeight;
      worldWidth = headers.worldWidth;
      start = headers.start;
      goal = headers.goal;

      canvas = getCanvas(headers.worldWidth, headers.worldHeight);
      ctx = getCanvasContext(canvas);

      predraw(ctx!, worldWidth!, worldHeight!);

      continue;
    }

    // if (data?.type === "metadata") {
    //   const metada: Metadata = data;
    //   drawMetadata(ctx!, worldWidth!, metada);
    //   break;
    // }

    const pathColor: string = colorOptions[i % colorOptions.length];
    const path: Point[] = data;

    drawPath(ctx!, start!, path, pathColor);
    i++;
  }

  drawStart(ctx!, start!);
  drawGoal(ctx!, goal!);

  await saveCanvasImage(canvas!);
};

run();
