# This is a project implementing a simple shell in Rust.

## Objective

The goal of this project is to create a basic shell that can execute commands, handle input/output redirection, and manage simple pipelines.
We will focus on building a functional shell with essential features while keeping the implementation straightforward.
The puropose of this project is to learn about rust programming, system calls, and process management.

## Features

The process of this shell is as follows:

1. **Reading Input**: The shell reads a line of input from the user.
2. **Lexering Input**: The input line is tokenized into commands and arguments.
3. **Parsing Commands**: The tokens are parsed to identify commands, arguments, and operators. In this step we will build an Recursive Descent Parse tree to represent the structure of the command.
4. **Executing Commands**: The parsed commands are executed using system calls. This includes handling built-in commands, external commands, input/output redirection, and pipelines.
5. **Handling Errors**: The shell will handle errors gracefully, providing feedback to the user when commands fail or when syntax errors are detected.
6. **Looping**: The shell will continue to prompt the user for input until an exit command is issued.

```
## Example command

-> ls -l | grep "txt" > output.txt
```

This command lists all files in long format, filters the results to show only those containing "txt", and redirects the output to a file named "output.txt".

1. The shell reads the input line.
2. The input is tokenized into commands and arguments like `ls`, `-l`, `|`, `grep`, `"txt"`, `>`, and `output.txt`.
3. The tokens are parsed to identify the structure of the command, recognizing the pipeline and redirection. In this step we will build an Recursive Descent Parse tree to represent the structure of the command.

Example Parse Tree:

```

         [Pipeline]
        /          \
    [Command]    [Redirection]
     /   \          /      \

ls -l [Command] output.txt
/ \
 grep "txt"

```

4. The shell executes the commands in sequence, setting up the pipeline between `ls` and `grep`, and redirecting the output of `grep` to `output.txt`.
5. If any errors occur during execution, the shell provides feedback to the user.
6. The shell prompts the user for the next command.

## Usage Techniques

| Language | Rust |
| Build System | Cargo |
| DevEnv | rustup |
| Testing | cargo test |
