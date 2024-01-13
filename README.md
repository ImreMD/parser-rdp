My plan here is to write a parser for arithmetic expression 
I will start with small and simplistic/primitive steps.

My main idea is to handle the parsing of the expression to an "engineer" 
its function will be to read the text, spawn node_builders and collect the results of their work (sub nodes) and assemble everything on the go. 
With time as a next step once the core functionnalities will be operational I'm thinking of adding concurrency to handle each node_builder in a different thread.

Step by step I would like to create my implementation of a parser
and learn Rust along the way. 

After finishing the parser I will try to implement an automatic differentiation algorithm to better understand Neural Network fundamental building block.

Wish me good luck!
