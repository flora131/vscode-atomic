# PHP Extension File Locations

## Implementation Files

- `extensions/php/package.json` - Extension manifest declaring language registration, grammar, and snippet contributions
- `extensions/php/language-configuration.json` - Language-specific configuration (comments, bracket matching, indentation, auto-closing pairs, folding markers)
- `extensions/php/syntaxes/php.tmLanguage.json` - TextMate grammar for PHP language syntax highlighting (4191 lines)
- `extensions/php/syntaxes/html.tmLanguage.json` - TextMate grammar for HTML with embedded PHP (97 lines)
- `extensions/php/snippets/php.code-snippets` - Code snippet definitions (50+ snippets for PHP constructs)

## Build/Tooling

- `extensions/php/build/update-grammar.mjs` - Grammar update utility using vscode-grammar-updater, manages adaptation of injection scopes and regex fixes for PHP grammar synchronization with external grammar source

## Configuration

- `extensions/php/package.nls.json` - Localization strings (display name and description)
- `extensions/php/cgmanifest.json` - Component governance manifest declaring external dependency (language-php v0.49.0 from GitHub)

## Development Configuration

- `extensions/php/.vscode/launch.json` - VS Code debug launch configuration for extension host testing
- `extensions/php/.vscode/tasks.json` - Build task configuration (npm compile)
- `extensions/php/.vscodeignore` - Package exclusion patterns (test/, build/, src/, tsconfig files, etc.)

## Notable Clusters

**Grammar and Syntax Architecture:**
- Two TextMate grammar files provide PHP syntax highlighting: standalone PHP and PHP embedded in HTML with language injections for SQL, XML, JavaScript, JSON, and CSS

**Snippet Coverage:**
- Comprehensive snippet set covering PHP 8+ features (attributes, enums, match expressions, spaceship operator) and classic constructs (classes, functions, loops, try-catch, traits)

**Grammar Synchronization:**
- Automated grammar update pipeline pulls from external KapitanOczywisty/language-php repository and applies custom patches (scope resolution, HTML grammar derivatives, regex case-sensitivity fixes)

## Directory Structure

```
extensions/php/
├── .vscode/                    # VS Code development configuration
├── build/                      # Build utilities (grammar updater)
├── snippets/                   # PHP code snippets
├── syntaxes/                   # TextMate grammar files
├── package.json                # Extension manifest
├── language-configuration.json # Language configuration
├── package.nls.json            # Localization
├── cgmanifest.json             # Component governance
└── .vscodeignore               # Package exclusions
```

## Summary

The PHP extension is a grammar-only language contribution providing syntax highlighting, bracket matching, and code snippets. It contains no implementation code or tests. The extension relies entirely on external TextMate grammar definitions (sourced from KapitanOczywisty/language-php) with automated synchronization and custom patches. Snippets cover modern PHP syntax including PHP 8+ features like attributes, enums, and match expressions alongside traditional PHP constructs.

