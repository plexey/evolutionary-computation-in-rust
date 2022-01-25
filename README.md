# Evolutionary Computation in Rust

![Dot paths example](example_images/example_1.png?raw=true)

This project is an experiment in implementing evolutionary algorithms in Rust. It features:
- selection
- reproduction
- recombination
- mutation

## Program

The program simulates evolution of a set of creatures called Dots living in a 2D grid. 

Every generation of Dots begins life at the same x,y coordinate located within the grid called `START`.

A Dot has a set of Genes where each gene is a random x,y offset value within some range. 

Dots express their genes sequentially, applying the x,y offset contained within each gene to their current position in the 2D grid. In this way, Dots can move within the grid from the `START` position.

Below is an example of the path a typical dot might take:

![Dot path](example_images/sample_dot_path_1.png?raw=true)

The blue square in the above image represents the `START` position of the dot, while the red squigly line represents the path the dot took from the `START` position.

Once each Dot from the current generation has expressed each of their genes, the program evaluates the fitness of each Dot.

In this world, fitness is based upon proxmity to the `GOAL` - another x,y coordinate located somewhere within the grid. 

The closer a Dot is to the `GOAL` the higher its fitness score.

With this measure of fitness, consider the following two dot paths:

![Comparison of Dot paths](example_images/dot_path_comparison.png?raw=true)

In the above image, the `START` position is represented by a blue square, while the `GOAL` position is represented by a red square. The dot path on the left is here considered 'fitter' than its counterpart on the right because it's final position in the grid is closer to the red sqaure.

Once all Dots from the current generation have been ranked by their fitness scores, the program proceeds to the selection phase, selecting breeding pairs from the current generation of dots. Selection is biased toward Dots with higher fitness scores.

When a breeding pair is selected, the program breeds a new child Dot via crossover of its parents genes - a random 50% portion of each parents genes is selected and recombined to form the child's genes. Once the child genes have been assembled, there is a chance a random child gene will be mutated via substitution with a new randomly generated gene.

Once crossover & mutation have finished, the program spawns a new Dot containing the newly fashioned child genes. 

This process is repeated until the next generation has been assembled.

Once the new generation is complete, the old generation is discarded and the cycle repeats for the new generation at the `START` position.

By logging out the path the fittest Dot from each generation took from the `START` position, it is possible to visualize the evolution of paths across multiple generations.

In order to visualize these paths, the Rust portion of this project writes the path data for the fittest Dot of each generation to `data/paths.txt` file. This path data is encoded as JSONL.

The data contained in this text file is then read by a Node application and rendered into a PNG using a Node implementation of the Canvas API. This image is saved to `/data/images`. I'm quite certain there's some excellent Rust crates out there for visualizing data, but I've gone with the Canvas API as it's dead simple to use and fulfils my needs.


## Setup

To setup the project, run the `setup.sh` script located in the `scripts` folder.

## Running the project

To run the project, run the `run.sh` script located in `scripts`. 

This will re-compile and execute the Rust portion of the project responsible for simulating evolution of dots over multiple generations and outputting paths data to the `data` directory. 

This paths data will then be picked up by the Node `imaging` portion of the project and processed into PNG image output to `data/images`.

The simulation can be configured via the `algo/src/constants.rs` file containing various constants used to control, for instance, how many dots there are per generation, how many genes each dot has, the mutation rate and so on. Modifiying these values will have a profound influence on the types of images generated.

For instace, here's an image 


## Further thoughts

Fitness evaluation could be expanded to include criteria such as steps taken.

The mutation phase could be expanded to encompass additional types of mutation beyond substitution such as:
- deletion (randomly delete a gene)
- insertion (randomly insert a new gene)