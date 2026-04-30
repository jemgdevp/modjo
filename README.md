# modjo
The ultimate TUI Client for API Exploration &amp; Testing. Postman’s power, Apidog’s workflow, Terminal’s speed.

## Docs 
### Explore our comprehensive documentation to get the most out of modjo:
- Site Documentation: [modjo.jemg.dev](https://modjo.jemg.dev/docs)

## Features
- **Intuitive TUI Interface**: Navigate your API projects with ease using a sleek terminal interface.
- **Comprehensive API Testing**: Create, manage, and execute API requests with support for various HTTP methods, headers, and body types.
- **Environment Management**: Define and switch between multiple environments to test your APIs in different contexts.
- **Collaboration**: Share your API projects with your team and collaborate in real-time.
- **Extensive Plugin Ecosystem**: Extend modjo’s functionality with a wide range of plugins for authentication, data generation, and more.
- **Cross-Platform Support**: Available on Windows, macOS, and Linux.

## Why Choose modjo?
- **Speed**: Experience lightning-fast performance with a terminal-based interface that eliminates the overhead of graphical applications.
- **Flexibility**: Tailor your API testing workflow with customizable plugins and environments.
- **Collaboration**: Work seamlessly with your team, sharing projects and insights without leaving the terminal.
- **Open Source**: Join a vibrant community of developers contributing to the growth and improvement of modjo.

## Installation
1. Download the latest release from our [GitHub Releases](https://github.com/jemgdevp/modjo/releases) page.
2. Follow the installation instructions for your operating system.
## Usage
### Current MVP (Ratatui)
Run:

```bash
cargo run
```

Main flow:
1. Edit `Metodo`, `URL`, `Headers` and `Body` directly in the TUI.
2. Press `s` to send the request.
3. Review status, time, size and response body in the response panel.
4. Press `c` to save current request in collections.
5. Press `Enter` over sidebar items to load history/collection requests.

Keyboard shortcuts:
- `q`: Quit
- `Tab`: Move focus between panels
- `s`: Send request
- `c`: Save current request to collections
- `h`: Open history in sidebar
- `l`: Open collections in sidebar
- `Up` / `Down`: Navigate sidebar items
- `Enter`: Load selected sidebar item

Mouse support:
- Left click selects the active panel (sidebar, URL, headers, body, response).

Persistence:
- App data is stored in `.modjo/` in your current project folder:
  - `.modjo/history.json`
  - `.modjo/collections.json`
  - `.modjo/env.toml`

Environment variables:
- You can interpolate variables in URL/body/headers with `{{var_name}}`.
- Variables are loaded from `.modjo/env.toml`.
## Contributing
We welcome contributions from the community! Please read our [CONTRIBUTING](https://github.com/jemgdevp/modjo/blob/main/CONTRIBUTING.md) for guidelines

## License
modjo is licensed under the MIT License. See the [LICENSE](https://github.com/jemgdevp/modjo/blob/main/LICENSE) file for details.