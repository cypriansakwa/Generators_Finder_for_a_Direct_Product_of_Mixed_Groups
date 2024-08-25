## Overview
This Rust program finds and prints the generators of the group $\mathbb{Z}_n \times \mathbb{Z}_m^\*$, where $\mathbb{Z}_n$ is an additive  and $\mathbb{Z}_m^\*$ is a multiplicative group. The code defines a struct ZnZmStar that represents these groups and provides methods to identify generator pairs for the combined group structure $\mathbb{Z}_n \times \mathbb{Z}_m^\*$. It identifies all pairs (a,b) that generate the entire group. 
## Features
This program contains the following;
- ZnZmStar Struct that represents the group $\mathbb{Z}_n \times \mathbb{Z}_m^\*$.
-  find_generators Method that finds all generator pairs $(a,b)$ of the group.
-  is_generator Method to check if a given pair $(a,b)$ is a generator.
-  main Function to emonstrate the usage of the ZnZm struct to find and print generators for a specific group.
-   get_multiplicative_group Method to find and return the multiplicative group $\mathbb{Z}_m^\*$ as a set of elements that are coprime with $m$.
-   gcd Function which is a helper function to compute the greatest common divisor $\gcd$ of two numbers, used to determine the elements of $\mathbb{Z}_m^\*$.

## How It Works
- Initialization: The program initializes the group $\mathbb{Z}_n \times \mathbb{Z}_m^\*$ with the provided values of $n$ and $m$.
- Finding Generators:
   - The find_generators method iterates over all possible pairs $(i,j)\in \mathbb{Z}_n \times \mathbb{Z}_m^\*$.
   - For each pair, the is_generator method checks if the pair can generate the entire group. A pair is a generator if it can visit all elements of $\mathbb{Z}_n \times \mathbb{Z}_m^\*$ exactly once before repeating.
- Output: The list of all generator pairs is printed to the console.
 ## Contributing
  - If you intend to contribute to this project, fork the repository and make a pull request.

  ## Installation

- To use this project, you need to have Rust installed on your machine.
- If Rust is not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install it.
- After installing Rust, clone this repository or copy the code into a Rust project, Compile and run the code using cargo run.
## Usage
- To use this code, you can clone the repository.
- You can change the values of $n$ and $m$ in the main function to test different cases.
- Compile the Rust code using cargo:
>```
>cargo build
>cargo run
- This will compile and run the program, printing the generators of the specified $\mathbb{Z}_n \times \mathbb{Z}_m$.
## Acknowledgments
- Rust
### Clone the repository or copy the source code into a Rust project.
```bash
git clone https://github.com/cypriansakwa/Generators_Finder_for_a_Direct_Product_of_Mixed_Groups.git 
cd Generators_Finder_for_a_Direct_Product_of_Mixed_Groups
