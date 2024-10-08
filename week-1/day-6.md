### Program Overview:

This Rust program is a simple command-line calculator that takes two numbers and an arithmetic operation (like addition, subtraction, division, or multiplication) from the user. It performs the specified operation and displays the result.

### How It's Built:

1. **Importing Libraries**:
   - The program starts by importing the `std::io` library, which is used to handle input and output. This allows the program to read data from the user through the terminal.

2. **Main Function**:
   - All the logic for the program is written inside the `main` function. This is the entry point of the program where everything starts.

3. **User Input**:
   - The program asks the user for two numbers:
     - It uses `println!` to display a message asking the user to enter the first number (`input1`).
     - It reads the user’s input using `std::io::stdin().read_line()` and stores it as a `String` in the `input1` variable.
     - The same process is repeated for the second number (`input2`).

4. **Asking for Operation**:
   - Next, the program asks what arithmetic operation the user wants to perform (e.g., `+`, `-`, `/`, `*`).
   - It reads the input into a variable called `opps`.

5. **Trimming and Parsing Input**:
   - The `trim()` function is used to remove any extra spaces around the user's input for both numbers and the operation.
   - The numbers entered by the user (as strings) are converted into `f32` (floating-point numbers) using `parse()`. If the conversion fails, the program will throw an error message asking the user to input valid numbers.

6. **Performing the Operation**:
   - The program checks which operation the user requested using a series of `if-else` statements:
     - **Addition (`+`)**: If the user entered a `+`, the program adds the two numbers and prints the result.
     - **Subtraction (`-`)**: If the user entered a `-`, it subtracts the second number from the first and prints the result.
     - **Division (`/`)**: If the user entered `/`, the program checks if the second number is zero to avoid dividing by zero (which would cause an error). If the second number is valid, it divides the first number by the second and prints the result.
     - **Multiplication (`*`)**: If the user entered `*`, it multiplies the two numbers and prints the result.
   - If the user enters an invalid operation (anything other than `+`, `-`, `/`, `*`), the program prints an error message asking the user to use one of the valid operations.


