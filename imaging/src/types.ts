export type Point = { x: number; y: number };

///////////// DATA PRESENTATION

export type Headers = {
  type: "headers";
  worldHeight: number;
  worldWidth: number;
  start: Point;
  goal: Point;
};

export type Metadata = {
  type: "metadata";
  generations: number;
};
