# System_Muscle

System_Muscle is a desktop application developed to automate gym management processes, mainly sales, inventory, calculations, and report generation.

## What problem does it solve?

The system aims to replace manual processes performed using notebooks, spreadsheets, or physical records, reducing errors and improving information management.

Main problems addressed:

- Manual sales records.
- Manual inventory management.
- Manual financial calculations.
- Manual report generation.
- Information loss and inconsistencies.

## Architecture

The project uses an **MVC (Model-View-Controller)** architecture:

- **Model:** Manages data and communication with SQLite.
- **View:** Provides the user interface using SvelteKit, TypeScript, and TailwindCSS.
- **Controller:** Processes requests and coordinates the application logic between the View and Model.

## Technologies

| Technology | Purpose |
|---|---|
| Tauri | Desktop application |
| Rust | Backend and system logic |
| SQLite | Database |
| SvelteKit | User interface |
| TypeScript | Frontend development |
| TailwindCSS | Interface styling |
| GitHub | Version control |

## Objective

System_Muscle aims to provide gyms with a centralized tool to manage their operations, reduce manual tasks, and improve control over business information.

## Project Status

Currently under development.
