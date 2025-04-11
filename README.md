# My Final Recap Project - Modern and Emerging Programming Language

This is my final recap project for the course `Modern and Emerging Programming Language` at [https://fitech101.aalto.fi](https://fitech101.aalto.fi).
![Gameplay Demo](rust_rush_game.gif)
**Game idea:** A player (represented by a yellow arrow) navigates a field, collecting hearts to increase their lives while dodging relentless aliens (represented by purple octopuses).

## How to run the game on your local machine:

**Step 1: Setting up the environment**

* **Install Rust:** If you don't have Rust installed, you'll need to download and install it using `rustup`. Visit the official Rust installation guide for detailed instructions: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install). Follow the instructions for your operating system.

* **Clone or Download the Project:**
    * **Using Git:** Open your terminal or command prompt and navigate to the directory where you want to save the project, then run:
        ```bash
        git clone <YOUR_PROJECT_GIT_REPOSITORY_URL>
        ```
        *(Replace `<YOUR_PROJECT_GIT_REPOSITORY_URL>` with the actual URL of your project's Git repository.)*
    * **Downloading as ZIP:** Alternatively, if you downloaded the project as a ZIP file, extract its contents to a directory on your computer.

* **Download Dependencies:** Navigate to the project directory in your terminal or command prompt:
    ```bash
    cd <YOUR_PROJECT_DIRECTORY>
    ```
    Then, fetch and build the necessary dependencies by running Cargo, Rust's package manager:
    ```bash
    cargo update
    ```
    This command will download all the crates (Rust libraries) specified in your `Cargo.toml` file.

* **Build and Run:** Finally, clean the previous build (if any) and build the project in release mode for optimized performance:
    ```bash
    cargo clean
    cargo run --release
    ```

**Step 2: Playing the game**

* **Terminal Output:** The game is a terminal-based application. Ensure your terminal window is sufficiently sized to display the game field clearly. You might need to adjust your terminal's font size or window dimensions for the best experience.

* **Controls:**
    * `Up Arrow`: Accelerate your player (yellow arrow).
    * `Down Arrow`: Decelerate your player.
    * `Left Arrow`: Turn the player's facing direction counter-clockwise.
    * `Right Arrow`: Turn the player's facing direction clockwise.

* **Gameplay Notes:**
    * The walls and aliens are represented by single pixel dots. Due to the simplicity of this representation, it might appear as though you can sometimes pass through them slightly. Don't be alarmed, this is an intentional aspect of the current implementation. Focus on avoiding direct and sustained contact!
    * Collect as many heart symbols as you can! Each heart collected increases your lives, giving you more chances to evade the alien octopuses. More hearts mean more fun and a higher potential score!
    * If a cute little purple octopus manages to catch you, the game will likely end. Look for options to "Restart" or "Quit" within the terminal output to play again or exit.

* **Emergency Exit:**
    * **Windows:** Press `Ctrl + C` to forcefully terminate the game.
    * **Linux/macOS:** Press `Ctrl + C` to forcefully terminate the game.

Enjoy playing your game! Good luck collecting those hearts and dodging the aliens!