# Node Sentinel

Node Sentinel is a command-line tool written in Rust that ensures you are using the correct Node.js version specified in your project's `.nvmrc` file. It integrates with `nvm` (Node Version Manager) to check and switch Node.js versions seamlessly, helping maintain consistency across development environments.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [Options](#options)
  - [Examples](#examples)
- [Automatic Version Switching with Zsh](#automatic-version-switching-with-zsh)
- [How It Works](#how-it-works)
- [Contributing](#contributing)
- [License](#license)

## Features

- **Version Checking**: Verifies if the current Node.js version matches the one specified in `.nvmrc`.
- **Automatic Switching**: Offers an option to automatically switch to the required Node.js version.
- **User Confirmation**: Prompts for confirmation before switching versions (unless auto-switch is enabled).
- **Custom Directory Support**: Allows specifying a directory to look for the `.nvmrc` file.
- **Integration with `nvm`**: Utilizes `nvm` to manage Node.js versions.

## Installation

### Prerequisites

- **Rust**: Ensure you have Rust and Cargo installed. You can install Rust using [rustup](https://rustup.rs/).
- **nvm**: Node Sentinel relies on `nvm` to switch Node.js versions. Install `nvm` from the [official repository](https://github.com/nvm-sh/nvm).

### Build from Source

1. **Clone the Repository**

   ```bash
   git clone https://github.com/joms/node-sentinel.git
   cd node-sentinel
   ```

2. **Build the Project**

   ```bash
   cargo build --release
   ```

3. **Install the Binary**

   Copy the compiled binary to a directory in your `$PATH`:

   ```bash
   cp target/release/node-sentinel /usr/local/bin/
   ```

## Usage

```bash
Usage: node-sentinel [OPTIONS] [DIRECTORY]
```

- **DIRECTORY**: (Optional) The directory to search for the `.nvmrc` file. Defaults to the current directory if not specified.

### Options

- `-a`, `--auto-switch`: Automatically switch to the required Node.js version without prompting for confirmation.
- `-c`, `--check-only`: Only check if the current Node.js version matches the required version; do not switch.
- `-h`, `--help`: Display help information.
- `-V`, `--version`: Display version information.

### Examples

#### Check Node.js Version

Check if the current Node.js version matches the one specified in `.nvmrc`:

```bash
node-sentinel --check-only
```

#### Automatically Switch Versions

Automatically switch to the required Node.js version without confirmation:

```bash
eval "$(node-sentinel --auto-switch)"
```

> **Note**: Use `eval` to execute the version switch in your current shell environment.

#### Specify a Directory

Check or switch Node.js version based on a `.nvmrc` file in a specific directory:

```bash
node-sentinel --check-only /path/to/project
```

#### Prompt Before Switching

By default, Node Sentinel will prompt for confirmation before switching versions:

```bash
eval "$(node-sentinel)"
```

Output:

```
Do you want to switch to Node.js version v14.17.0? [y/N]:
```

Respond with `y` or `yes` to proceed.

## Automatic Version Switching with Zsh

To enhance your workflow, you can configure your Zsh shell to automatically execute Node Sentinel whenever you navigate to a directory. This ensures that you're always using the correct Node.js version specified in the `.nvmrc` file without manual intervention.

### Step-by-Step Guide

1. **Ensure Node Sentinel is in Your `$PATH`**

   Make sure that the `node-sentinel` binary is accessible from your shell. If you followed the installation steps, it should be in `/usr/local/bin/`.

2. **Add the Auto-Execution Function**

   Add the following function to your `.zshrc` file:

   ```bash
   function node_sentinel_auto_switch() {
       eval "$(node-sentinel --auto-switch)"
   }

   autoload -U add-zsh-hook
   add-zsh-hook chpwd node_sentinel_auto_switch
   ```

   **Explanation:**

   - **Function `node_sentinel_auto_switch`**: Runs `node-sentinel` with the `--auto-switch` option and evaluates its output to switch the Node.js version.
   - **Hook with `add-zsh-hook`**: The `chpwd` hook triggers the `node_sentinel_auto_switch` function every time you change directories.

3. **Reload Your `.zshrc`**

   Apply the changes by reloading your `.zshrc` file:

   ```bash
   source ~/.zshrc
   ```

### Usage

Now, whenever you navigate to a directory containing an `.nvmrc` file, Node Sentinel will automatically check and switch to the required Node.js version.

**Example:**

```bash
cd /path/to/your/project
```

Upon changing to the project directory, you should see output similar to:

```
Using Node.js version v14.17.0
```

### Notes

- **Custom Directories**: If you need Node Sentinel to look for the `.nvmrc` file in a parent directory or a specific path, you can modify the function to include the directory option.

- **Confirmation Prompt**: The `--auto-switch` flag ensures that Node Sentinel switches versions without prompting. If you prefer to be prompted before switching, remove the `--auto-switch` option:

  ```bash
  eval "$(node-sentinel)"
  ```

- **Performance Considerations**: Running a command on every directory change may have a slight impact on shell performance. In most cases, this should be negligible.

## How It Works

Node Sentinel performs the following steps:

1. **Check for `nvm` Installation**: Ensures that `nvm` is installed on your system.
2. **Read `.nvmrc` File**: Retrieves the required Node.js version from the `.nvmrc` file in the specified directory.
3. **Resolve Version**: Handles version strings like `lts` or `latest`, resolving them to actual version numbers.
4. **Compare Versions**: Checks if the current Node.js version matches the required version.
5. **Switch Version (Optional)**:
   - If the versions do not match and the `--check-only` flag is not set, it offers to switch to the required version.
   - Outputs the command `nvm use <version>` which can be executed in the current shell.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests for enhancements and bug fixes.

1. **Fork the Repository**
2. **Create a Feature Branch**

   ```bash
   git checkout -b feature/your-feature
   ```

3. **Commit Changes**

   ```bash
   git commit -am 'Add your feature'
   ```

4. **Push to the Branch**

   ```bash
   git push origin feature/your-feature
   ```

5. **Open a Pull Request**

## License

This project is licensed under the GPLv3 License. See the [LICENSE](LICENSE) file for details.

---

**Disclaimer**: This tool requires `nvm` to function correctly. Ensure that `nvm` is properly installed and configured in your environment.
