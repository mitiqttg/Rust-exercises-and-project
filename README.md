# My Final Recap Project - Modern and Emerging Programming Language

This is my final project for the course `Rust - Modern and Emerging Programming Language`
![Gameplay Demo](rust_rush_game.gif)
**Game idea:** A player (represented by a yellow arrow) navigates a field, collecting hearts to increase their health points while dodging relentless aliens (represented by cute purple octopuses).

## How to run the game on your local machine:

**Step 1: Setting up the environment**

* **Install Rust:** If you don't have Rust installed, you'll need to download and install it using `rustup`. Visit the official Rust installation guide for detailed instructions: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install). Follow the instructions for your operating system.

* **Clone or Download the Project:**
    * **Using Git:** Open your terminal or command prompt and navigate to the directory where you want to save the project, then run:
        ```bash
        git clone https://github.com/mitiqttg/Rust-game-project.git your-folder-name
        ```
    * **Downloading as ZIP:** Alternatively, if you downloaded the project as a ZIP file, extract its contents to a directory on your computer.

* **Download Dependencies:** Navigate to the project directory in your terminal or command prompt, make sure it contain `Cargo.toml` file ():
    ```bash
    cd your-folder-name
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

**Step 2: Enjoying the game**

* **Terminal Output:** The game is a terminal-based application (in my case, I'm running on VScode terminal). Ensure your terminal window is sufficiently sized to display the game field clearly. You might need to adjust your terminal's font size or window dimensions for the best experience.

* **Controls:**
    * `Up Arrow`: Accelerate your player 
    * `Down Arrow`: Decelerate your player
    * `Left Arrow`: Turn counter-clockwise ↪️
    * `Right Arrow`: Turn clockwise ↩️

* **Gameplay Notes:**
    * The walls and aliens are represented by single pixel dots. Due to the simplicity of this representation, it might appear as though you can sometimes pass through them slightly. Don't be alarmed, this is an intentional aspect of the current implementation. Focus on avoiding direct and sustained contact!
    * Collect as many heart symbols as you can! Each heart collected increases your health, giving you more chances to escape the alien octopuses. More hearts mean more fun and a higher potential score!
    * If an octopus manages to catch you, -1 health point until you got 0 left. Look for options to "Restart" or "Quit" within the terminal output to play again or exit.

* **Emergency Exit:**
    Press `Ctrl + C` to forcefully terminate the game.

Enjoy!
-from miti with ❤️