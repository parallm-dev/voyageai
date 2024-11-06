# Rust Development Conventions

## Code Style
- Follow Rust official style guide and use `rustfmt`
- Use 4 spaces for indentation
- Maximum line length of 100 characters
- Group imports by std, external crates, and internal modules
- Use clear variable/function names in snake_case
- Document public APIs with rustdoc comments

## Error Handling
- Use Result<T,E> for operations that can fail
- Create custom error types for domain-specific errors
- Avoid unwrap() except in tests or examples
- Use ? operator for error propagation
- Handle all Result and Option values explicitly

## Testing
- *Never* write unit tests in same file as code
    - always write them in `tests/` directory named `test_*.rs`
- Integration tests go in `tests/integration` directory
- Use #[test] attribute for test functions
- Follow arrange-act-assert pattern
- Aim for >80% test coverage
- Use test data builders for complex types

## Project Structure
- src/lib.rs for library crates
- src/main.rs for binary crates
- One module per file
- Group related functionality in modules
- Keep module hierarchies shallow
- Use workspaces for multi-crate projects

## Aider Conventions

### Prompts
- Be specific and explicit
- One change request per prompt
- Reference specific files/functions
- Include context when needed
- Use clear success criteria

### Code Changes
- Make atomic, focused changes
- Follow existing code style
- Include tests for changes
- Update documentation
- Add clear commit messages

### Development Flow
- Start with failing tests
- Make minimal changes to pass
- Refactor after tests pass
- Commit logical units
- Review diffs before committing

### Best Practices
- Use --edit for targeted changes
- Save work frequently
- Keep git history clean
- Document significant changes
- Test changes incrementally
