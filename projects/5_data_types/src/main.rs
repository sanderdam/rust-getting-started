mod characters;
mod numeric_operations;
mod booleans;
mod compound_types;
mod numeric_types;

fn main() {
    numeric_types::numeric_types();

    numeric_operations::numeric_operations();    

    booleans::booleans();

    characters::characters();

    compound_types::compound_types();
    compound_types::arrays();
}

