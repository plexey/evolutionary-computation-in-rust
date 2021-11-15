# Evolutionary Computation in Rust

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

Once each Dot from the current generation has expressed each of their genes, the program evaluates the fitness of each Dot.

In this world, fitness is based upon proxmity to the `GOAL` - another x,y coordinate located somewhere within the grid. 

The closer a Dot is to the `GOAL` the higher its fitness score.

Once all Dots from the current generation have been ranked by their fitness scores, the program proceeds to the selection phase, selecting breeding pairs from the current generation. Selection is biased toward Dots with higher fitness scores.

Once a breeding pair has been selected, the program breeds a child Dot via crossover & mutation. 

A random 50% portion of each parents genes is selected and randomly recombined to form the child's genes. Some artificial mutation is introduced at this point whereby there is a chance a random child gene will be subsituted with a new randomly generated gene.

Once crossover & mutation have finished, the program spawns a new Dot containing the newly fashioned child genes. 

This process is repeated until the next generation has been assembled.

Once the new generation is complete, the old generation is discarded and the cycle repeats for the new generation at the `START` position.

By logging out the path the fittest Dot from each generation took from `START`, it is possible to visualize the evolution of paths across multiple generations.

In order to visualize these paths, the Rust portion of this project writes the path data for the fittest Dot of each generation to `data/paths.txt` file. This path data is encoded as JSONL.

The data contained in this text file is then read by a Node application and rendered into a PNG using a Node implementation of the Canvas API. This image is saved to `/data/images`.


## Further thoughts

Fitness evaluation could be expanded to include criteria such as steps taken.

The mutation phase could be expanded to encompass additional types of mutation beyond substitution such as:
- deletion (randomly delete a gene)
- insertion (randomly insert a new gene)
