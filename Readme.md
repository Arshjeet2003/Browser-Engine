# 🖥️ Minimal Browser Engine (Rust)

This project is a simplified browser engine implemented in Rust. It parses HTML and CSS, builds a styled representation of the document, computes layout information, and renders display commands using GPU acceleration via `wgpu`.


## 📌 Features Implemented

### 1. Parsing HTML  
A custom HTML parser reads and constructs a **DOM-like node tree** from HTML source input.

- Elements and text nodes are identified.
- Nesting and hierarchy are preserved.
- Tree structures are printed in a readable format for debugging.


### 2. Parsing CSS  
A CSS parser reads and interprets CSS rules:

- CSS selectors, properties, and values are parsed.
- Stylesheets are represented as collections of rules.
- Debug print functionality displays the parsed stylesheet.


### 3. Building a Style Tree  
Combines the parsed HTML node tree with the parsed CSS rules to create a **Style Tree**:

- Each node in the HTML tree is paired with its computed style.
- Cascading and inheritance are applied where applicable.
- Pretty print available for visualization.


### 4. Computing Layout  
A layout system computes positions and dimensions for styled elements:

- Uses a simple block-based layout model.
- Computes a layout tree with absolute positions and sizes.
- Viewport dimensions are considered during layout calculation.
- Layout tree can be pretty-printed for inspection.


### 5. Building Display Commands  
The layout tree is traversed to produce a list of **Display Commands**:

- Commands include instructions for drawing rectangles with specific colors.
- These commands act as the rendering instructions for the GPU renderer.


### 6. Rendering Display Commands  
The display commands are passed into a `render::render_loop()` function which:

- Initializes a `wgpu` window and rendering context.
- Converts display commands into GPU draw calls.
- Renders solid-colored rectangles to the screen.


## 📦 Crates Used

- `wgpu` — GPU rendering backend.
- `winit` — Window creation and event handling.
- `pollster` — Simple async executor for wgpu setup.


## 📝 How to Run

Use command - cargo run
