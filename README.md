# Projects Indexer

A powerful command-line tool for indexing and organizing your projects. It can scan directories, detect project types, generate tags using Local LLM models (Ollama), and provide detailed statistics about your project collection.

## Features

- 🔍 Recursive directory scanning with configurable depth
- 📊 Project status detection (active/archived) based on git history
- 🏷️ AI-powered tag generation using Ollama
- 📁 Smart project categorization based on directory structure
- 🔎 Search functionality across projects, tags, and categories
- 📈 Detailed project statistics and insights

## Prerequisites

- Rust 1.70 or higher
- [Ollama](https://ollama.ai/) (optional, for AI tag generation)

## Installation

```bash
# Clone the repository
git clone git@github.com:RustSandbox/projets-indexer.git
cd projets-indexer

# Build the project
cargo build --release

# Optional: Install globally
cargo install --path .
```

## Usage

### Basic Commands

```bash
# Show help
projets-indexer --help

# Index projects with default settings
projets-indexer index

# Index projects with custom directory and output
projets-indexer index -d ~/my-projects -o my-index.json

# Search projects
projets-indexer search "machine learning"

# Show project statistics
projets-indexer stats

# Generate tags for a specific project
projets-indexer generate-tags -p ~/projects/my-project
```

### Index Command Options

```bash
projets-indexer index [OPTIONS]

Options:
  -d, --projects-dir <DIR>    Directory containing projects [default: ~/projects]
  -o, --output <FILE>         Output JSON file [default: projects_index.json]
  -a, --ollama               Enable Ollama for tag generation [default: true]
  -x, --max-depth <NUM>      Maximum directory depth [default: 3]
  -m, --min-depth <NUM>      Minimum directory depth [default: 3]
  -e, --exclude <DIRS>       Directories to exclude [default: .git,node_modules,...]
  -v, --verbose             Enable verbose output
  -n, --no-color           Disable color output
```

### Search Command Options

```bash
projets-indexer search [OPTIONS] <QUERY>

Options:
  -i, --index-file <FILE>    Index file to search in [default: projects_index.json]
  -t, --tags-only           Search only in project tags
  -c, --category-only       Search only in project categories
```

### Stats Command Options

```bash
projets-indexer stats [OPTIONS]

Options:
  -i, --index-file <FILE>    Index file to analyze [default: projects_index.json]
  -d, --detailed            Show detailed category breakdown
```

### Generate Tags Command Options

```bash
projets-indexer generate-tags [OPTIONS]

Options:
  -p, --project-dir <DIR>    Project directory to analyze
  -o, --output <FILE>        Output file for generated tags
```

## Example Output

```
⚙️ Configuration
════════════════
📁 Projects Directory: ~/projects
📦 Index File: projects_index.json
⚙️ Ollama Enabled: yes

🔍 Indexing Projects
════════════════════
✨ Found 42 projects
🚀 14 active projects
📚 28 archived projects
🏷️ 156 total tags generated

Projects by Category
──────────────────────────────
📁 web: 15
📁 machine-learning: 12
📁 tools: 8
📁 research: 7
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👨‍💻 About the Author

I'm Hamze Ghalebi, CTO at Remolab, passionate about building tools that improve developer workflows. This Projects Indexer is part of a collection of tools I originally built for my own use, and I've decided to open source it in case others find it helpful.

As someone who works with many projects simultaneously, I created this tool to help me index and track all my projects effectively. It helps me maintain a clear overview of my project collection, making it easier to find and manage different projects across various domains.

Many of the tools I create solve specific pain points in my daily workflow with cloud infrastructure and development environments. If you have any feedback or suggestions for improvements, please feel free to contribute!

### Connect with me:

* GitHub: [hghalebi](https://github.com/hghalebi)
* Twitter/X: [@hamzeml](https://twitter.com/hamzeml)
* LinkedIn: [Hamze Ghalebi](https://linkedin.com/in/hamzeghalebi)

### Support this project:

If you find this tool useful, please consider [sponsoring me on GitHub](https://github.com/sponsors/hghalebi) to support continued development and maintenance. 