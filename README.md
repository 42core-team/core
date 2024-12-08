# Core
This is the "real" server for the CORE game, written in Rust. It handles all game logic, processes actions from the socket, and sends the game state containing all relevant information back to the client.

## Development Setup
The preferred way to develop is by using the sandbox repository, which includes this project as a submodule. This setup allows you to run both the server and the client simultaneously with ease.

If you prefer to develop the server independently, you can follow the steps below to open the project in a devcontainer in VS Code.

## Environment Variables

| Variable Name | Description |
|---------------|-------------|
| `PORT`        | Sets the port on which the socket should listen. |
| `LOG_TO_FILE`         | Enables logging if set to true. |
| `SEED`        | Sets the random seed for the game. |
| `TICK_RATE`   | Sets how many ticks per second the server calculates. Also sets the speed through that. |

## Usage Guide

### Commands

Use the following `make` commands to manage the project:

- **run**: Runs the game binary.
- **build**: Builds the game binary.
- **clean**: Cleans the build artifacts.
- **re**: Cleans and then builds the game binary.
- **doc**: Generates the documentation for the project.
- **test**: Runs the tests for the project.

### Opening the Devcontainer in VS Code

To open the project in a devcontainer in VS Code, follow these steps:

1. Open the project folder in VS Code.
2. Click on the green button in the bottom-left corner of the window that says "Reopen in Container".
3. If the button is not visible, open the Command Palette (Ctrl+Shift+P) and type "Remote-Containers: Reopen in Container" and select it.
4. VS Code will now build and open the project in a devcontainer.

This setup ensures that you have a consistent development environment with all the necessary dependencies and tools installed.
