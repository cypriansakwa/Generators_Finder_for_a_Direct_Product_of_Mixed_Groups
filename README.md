This Rust program finds and prints the generators of the group Z_n × Z*_m, where Z_n is an additive  and Z*_m is a multiplicative group. 
It identifies all pairs (a,b) that generate the entire group. 

This program contains the following;

1. ZnZmStar Struct that represents the group Z_n × Z*_m.

2. find_generators Method that finds all generator pairs (a,b) of the group.

3. is_generator Method to check if a given pair (a,b) is a generator.

4. main Function to emonstrate the usage of the ZnZm struct to find and print generators for a specific group.
5. get_multiplicative_group Method to find and return the multiplicative group Z*_m as a set of elements that are coprime with m.
6. gcd Function which is a helper function to compute the greatest common divisor (GCD) of two numbers, used to determine the elements of Z*_m.

git clone https://github.com/cypriansakwa/Generators_Finder_for_a_Direct_Product_of_Mixed_Groups.git 
cd Generators_Finder_for_a_Direct_Product_of_Mixed_Groups
