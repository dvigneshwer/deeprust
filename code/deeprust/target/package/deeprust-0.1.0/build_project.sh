# Compile the library project
echo building the project
cargo build

# Creating the local documentation 
echo Creaitng the documentation
cargo doc 

# Running the examples
echo Running the testcases 
cargo test