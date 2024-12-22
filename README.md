# Rusty Game of Life with Bevy

**Rusty Game of Life with Bevy** is a simulation of Conway's Game of Life built using the [Bevy game engine](https://bevyengine.org/). This project demonstrates the use of Rust's modern game development capabilities while offering an interactive way to explore cellular automata principles.

## Features

- **Real-time Simulation**: Watch the cells evolve according to Conway's rules.
- **Customizable Grid**: Modify the grid size and initial conditions.
- **Pause/Play Controls**: Start or pause the simulation at will.
- **Step-by-Step Simulation**: Advance the simulation one step at a time for detailed observation.
- **Dynamic Speed Adjustment**: Change the simulation speed.

## Installation

### Prerequisites
- Rust (latest stable version recommended)
- Cargo (comes with Rust)
- Bevy dependencies ([installation guide](https://bevyengine.org/learn/book/getting-started/))

### Steps
1. Clone the repository:
   ```bash
   git clone https://github.com/John-Beeping-Doe/rusty_game_of_life_with_bevy.git
   cd rusty_game_of_life_with_bevy

	2.	Build the project:

cargo build --release


	3.	Run the simulation:

cargo run



Usage
	1.	Launch the application by running cargo run.
	2.	Use the following keyboard controls:
	•	Space: Pause/Play the simulation.
	•	Arrow Keys: Navigate the grid.
	•	Enter: Advance the simulation by one step (when paused).
	•	+/-: Increase or decrease the simulation speed.
	3.	Modify initial conditions via configuration files or programmatically in the code.

Project Structure
	•	src/: Contains the source code.
	•	main.rs: Entry point of the application.
	•	game_logic.rs: Implements the rules of Conway’s Game of Life.
	•	ui.rs: Manages user interface elements.
	•	Cargo.toml: Project dependencies and metadata.
	•	assets/: Resources for the project (e.g., icons, shaders).
	•	README.md: Project documentation.

Dependencies

This project leverages the following key dependencies:
	•	Bevy - Game engine for 2D and 3D applications.
	•	serde - Serialization/deserialization.
	•	rand - Random number generation.

Contributions

Contributions are welcome! If you’d like to improve the project or add new features:
	1.	Fork the repository.
	2.	Create a new branch for your changes.
	3.	Submit a pull request with a clear description of the changes.

License

This project is licensed under the MIT License. See the LICENSE file for details.

Contact

For questions or suggestions, feel free to open an issue on GitHub or contact the maintainer through the repository.

Enjoy simulating life!

