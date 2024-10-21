# Magellanicus: Custom Vulkan Renderer for Halo Custom Edition

## Overview

Magellanicus is a custom Vulkan-based renderer written in Rust, designed as part of a larger project aimed at creating a complete replacement for the Halo Custom Edition Sapien editor. This project seeks to overhaul the graphics rendering capabilities of the Halo modding tool with a modern, high-performance engine. While still a work in progress, the ultimate goal is to enhance the modding experience for Halo Custom Edition by introducing a more powerful editor with modern rendering techniques.

Magellanicus has the potential to later serve as a renderer replacement for Halo Custom Edition itself, making it an exciting project both for developers working within the modding scene and for potential in-game improvements.

## Features

	•	Vulkan-based rendering: Leverages the Vulkan API for low-level, efficient GPU access and performance optimization.
	•	Rust implementation: Written in Rust, providing memory safety, performance, and ease of debugging.
	•	New Sapien editor renderer: Part of a larger initiative to replace the entire Sapien editor for Halo Custom Edition.
	•	Future potential for Halo CE renderer replacement: While focused on Sapien editor replacement, Magellanicus is also being developed with the possibility of functioning as a renderer for the game itself in the future.
	•	Custom shaders and lighting support: Provides advanced shader programming and improved lighting, creating a more modern visual experience. Built in interpolation and anisotropic filtering
	•	Modular design: Designed with flexibility in mind, allowing for easier future expansions and adaptations to meet the needs of both the editor and potential in-game use cases.

## Project Goals

	1.	Replace the Sapien Editor: Provide a fully modern and enhanced rendering engine as part of a custom-built Sapien editor replacement for Halo Custom Edition, improving the modding experience.
	2.	Potential Full-Game Renderer: After completion of the Sapien editor project, consider extending Magellanicus as a renderer replacement for the Halo Custom Edition game itself.
	3.	Modern Graphics Techniques: Utilize Vulkan’s advanced capabilities for efficient rendering, introducing features like custom shaders, improved lighting, and better GPU utilization.

## Installation and Usage

	1.	Clone the repository: 
 		git clone https://github.com/SnowyMouse/magellanicus


	2.	Build the project: 
 		Ensure that you have Rust and Vulkan SDK installed. Then, navigate to the project directory and build using Cargo:
		cd magellanicus
		cargo build --release


	3.	Run the renderer:
		Specific integration details with the replacement Sapien editor are still under development. Instructions will be provided as the project progresses.

## Roadmap

	•	Initial Vulkan renderer implementation
	•	Basic shader and lighting support
	•	Early integration with the Sapien replacement program
	•	Advanced rendering techniques
	•	Optimization for both editor and possible game usage
	•	Potential full-game renderer integration

## Contributing

Contributions are welcome! Please feel free to open an issue to discuss ideas, report bugs, or submit pull requests. Your input is appreciated as this project moves towards creating a fully-fledged renderer replacement.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.

By SnowyMouse and Aerocratica
