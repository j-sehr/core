# Core Project Structure - Version 1.3
This document outlines the structure of the Core project, detailing the organization of directories and files to ensure consistency and maintainability across the codebase.

## 1. Root Directory
- README.md: Provides an overview of the project, setup instructions, and usage guidelines.
- LICENSE: Contains the licensing information for the project.
- CHANGELOG.md: Documents the changes made in each version of the project.
- GOALS.md: Lists the objectives and goals of the project.
- PROJECT_STRUCTURE.md: This document, detailing the project's structure.
- docker-compose.[env].yaml: Docker Compose files for different environments (e.g., development, production).
- .env: Environment variable files for different environments.
- .env.example: Example environment variable file for reference.
- mise.toml: Configuration file for the Mise tool, if applicable.
- .gitignore: Specifies files and directories to be ignored by Git.
- Cargo.toml, Cargo.lock, src: Rust project files and directories.
- templates/: Directory containing project templates.

## 2. Template
- Module Template (`templates/module-template`)
  - mod.rs : Module Entrypoint.
  - exports.rs : Module Exports for other modules.
  - services/: Directory for services e.g. TokenService.
  - guards/: Directory for guards e.g. Authenticated.
  - models/ Directory for database models e.g. User.
  - dtos/: Directory for Data Transfer Objects e.g. CreateUserDto or AuthenticationResponseDTO.
  - config/: Directory for module configuration e.g. ModuleConfig.
  - migrations/: Directory for database migrations.
  - errors/: Directory for module-specific error handling.
  - routes/: Directory for route definitions.
  - module.rs: The Module Definition
  - e.g. other directories as needed for the module's functionality.

## 3. Modules
Each Module should be composable, self-contained, and follow the structure outlined in the Module Template and when importing a Module it should only import the other modules exports.rs file to ensure encapsulation.

### The Base Module is a special module that provides core functionality and should be included in every other module.
- Database Connection
- Logging
- Configuration Management
- Tracing

## 4. Src Directory (MIGRATION/RESTRUCTURE PENDING)
- main.rs: The main entry point of the application.
- lib.rs: Library entry point for shared functionality.
- config/: Directory for application-wide configuration files.
- modules/: Directory containing all project modules.
- utils/: Directory for utility functions and helpers.
- common/: Directory for common types and functions used across modules.
