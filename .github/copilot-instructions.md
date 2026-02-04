# Copilot Instructions for Core Project

## Project Overview
Core is a Rust-based modular web application project built for learning and experimentation. The project focuses on creating a composable, self-contained module architecture with authentication as the MVP feature.

## Technology Stack
- **Language**: Rust (Edition 2024)
- **Web Framework**: Axum 0.8.8
- **Database**: SurrealDB 2.5.0
- **Authentication**: JWT-based with argon2 password hashing
- **Async Runtime**: Tokio with full features
- **Configuration**: Figment (supports env, json, toml)
- **Logging**: Tracing with tracing-subscriber

## Architecture & Design Patterns

### Modular Structure
The project follows a modular architecture where each module is:
- **Self-contained**: All functionality related to a feature is within its module
- **Composable**: Modules can be combined and reused
- **Encapsulated**: Other modules should only import via the module's `exports.rs` file

### Module Template Structure
Each module should follow this structure:
```
modules/<module-name>/
├── mod.rs          # Module entrypoint
├── exports.rs      # Public API for other modules
├── module.rs       # Module definition
├── services/       # Business logic services
├── guards/         # Authentication/authorization guards
├── models/         # Database models
├── dtos/           # Data Transfer Objects
├── config/         # Module-specific configuration
├── migrations/     # Database migrations
├── errors/         # Error handling
└── routes/         # HTTP route definitions
```

### Base Module
The base module provides core functionality for all other modules:
- Database connection management
- Logging and tracing setup
- Configuration management
- Application-wide utilities

## Coding Guidelines

### File Organization
- Keep related functionality together within modules
- Use `mod.rs` for module entrypoints
- Use `exports.rs` to control the public API
- Separate concerns: models, services, routes, DTOs

### Naming Conventions
- Use snake_case for file names and function names
- Use PascalCase for struct and enum names
- Suffix DTOs with `Dto` (e.g., `CreateUserDto`)
- Suffix services with `Service` (e.g., `TokenService`)
- Suffix errors with `Error` (e.g., `AuthenticationError`)

### Error Handling
- Use `thiserror` for custom error types
- Use `anyhow` for application-level error handling
- Provide meaningful error messages
- Use the error macro pattern where applicable (see `macros/error_return.rs`)

### Database
- SurrealDB is used as the database
- Define tables as SCHEMAFULL in migrations
- Use proper indexing for performance
- Follow the schema structure defined in README.md for accounts and sessions

### Configuration
- Use Figment for configuration management
- Support multiple config sources: files, environment variables
- Provide `.example` files for configuration templates
- Keep sensitive data in environment variables

## Development Workflow

### Building
```bash
cargo build
```

### Running
```bash
cargo run
```

### Testing
- Testing infrastructure is planned but not yet implemented
- When adding tests, follow Rust testing conventions
- Place unit tests in the same file as the code they test
- Place integration tests in the `tests/` directory

### Dependencies
- Use Docker Compose for local development (see `docker-compose.dev.yml`)
- SurrealDB must be running and initialized with the schema from README.md
- Configuration files should be copied from `.example` files and customized

## Current State & Goals

### Completed
- ✅ MVP Authentication module with JWT-based auth
- ✅ Basic modular architecture
- ✅ Database integration with SurrealDB

### In Progress / Planned
- 🔄 Cleanup and refactor for improved maintainability
- 🔄 Add comprehensive testing
- 🔄 Improve modularity

## Common Tasks

### Adding a New Module
1. Create the module directory under `src/modules/`
2. Follow the module template structure
3. Create `mod.rs`, `exports.rs`, and `module.rs`
4. Implement services, models, routes as needed
5. Only expose necessary items through `exports.rs`
6. Import the base module for core functionality

### Adding a New Route
1. Define the route handler in the appropriate module's `routes/` directory
2. Use Axum's router and handler patterns
3. Apply guards for authentication/authorization as needed
4. Return proper DTOs, not raw models

### Adding a Database Model
1. Create the model in the module's `models/` directory
2. Define the schema in SurrealDB migrations
3. Use proper types and derive macros (Serialize, Deserialize)
4. Add necessary indexes to the database

## Security Considerations
- Passwords are hashed using argon2
- JWT tokens are used for authentication
- Session management with refresh tokens
- Use HTTPS in production
- Validate all user inputs
- Follow Rust security best practices

## Additional Context
- This is a personal learning project
- Code quality and maintainability are priorities
- The project structure is still evolving (see PROJECT_STRUCTURE.md)
- Contributions should maintain the modular architecture
