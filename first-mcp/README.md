# First MCP Server

A simple Model Context Protocol (MCP) server that provides tools to alter files.

## Features

This MCP server provides three tools:
- **read_file**: Read the contents of a file
- **write_file**: Write content to a file (creates or overwrites)
- **append_to_file**: Append content to the end of a file

## Prerequisites

- Node.js (v18 or higher recommended)
- npm (comes with Node.js)

## Installation

1. Navigate to the project directory:
```bash
cd /home/gk/FUN/ai-poc/first-mcp
```

2. Install dependencies:
```bash
npm install
```

## Build

Compile the TypeScript code to JavaScript:

```bash
npm run build
```

This creates the compiled files in the `build/` directory.

## Testing Your Server

### Method 1: Using MCP Inspector (Recommended for Testing)

The MCP Inspector provides a web UI to test your server tools interactively.

1. Make sure your server is built (`npm run build`)

2. Run the inspector:
```bash
npx @modelcontextprotocol/inspector node /home/gk/FUN/ai-poc/first-mcp/build/index.js
```

Or from within the project directory:
```bash
npx @modelcontextprotocol/inspector node build/index.js
```

3. The inspector will open in your browser (usually at `http://localhost:5173`)

4. In the UI, you can:
   - View all available tools
   - Test each tool by clicking on it
   - Provide parameters (like file paths and content)
   - See the results in real-time

**Troubleshooting Inspector Connection Errors:**

If you see "Connection Error - Check if your MCP server is running and proxy token is correct":

1. Verify the build exists:
```bash
ls build/index.js
```

2. Test the server directly:
```bash
node build/index.js
```
You should see: `First MCP server running on stdio` (in stderr)

3. Make sure you're using the correct path in the inspector command

### Method 2: Using Claude Desktop

Add this to your Claude Desktop configuration file:

**Linux**: `~/.config/Claude/claude_desktop_config.json`  
**MacOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`  
**Windows**: `%APPDATA%/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "first-mcp": {
      "command": "node",
      "args": ["/home/gk/FUN/ai-poc/first-mcp/build/index.js"]
    }
  }
}
```

After saving, restart Claude Desktop. The server will be available and Claude can use the file tools automatically.

### Method 3: Direct Execution

You can run the server directly (it will wait for stdio input):

```bash
npm start
```

Or:
```bash
node build/index.js
```

The server runs on stdio and waits for MCP protocol messages.

## Development

Run in watch mode (automatically rebuilds on file changes):

```bash
npm run dev
```

This is useful when actively developing and testing changes.

## Tool Documentation

### read_file

Reads and returns the contents of a file.

**Parameters:**
- `path` (string, required): Path to the file to read

**Example:**
```json
{
  "path": "/path/to/file.txt"
}
```

### write_file

Writes content to a file. Creates the file if it doesn't exist, overwrites if it does. Automatically creates parent directories if needed.

**Parameters:**
- `path` (string, required): Path to the file to write
- `content` (string, required): Content to write to the file

**Example:**
```json
{
  "path": "/path/to/file.txt",
  "content": "Hello, World!"
}
```

### append_to_file

Appends content to the end of a file. Creates the file if it doesn't exist. Automatically creates parent directories if needed.

**Parameters:**
- `path` (string, required): Path to the file to append to
- `content` (string, required): Content to append to the file

**Example:**
```json
{
  "path": "/path/to/file.txt",
  "content": "\nNew line of text"
}
```

## Project Structure

```
first-mcp/
├── src/
│   └── index.ts          # Main server implementation
├── build/
│   └── index.js          # Compiled JavaScript (generated)
├── node_modules/         # Dependencies (generated)
├── package.json          # Project configuration
├── tsconfig.json         # TypeScript configuration
├── .gitignore           # Git ignore rules
└── README.md            # This file
```

## How It Works

1. The server uses the MCP SDK to implement the Model Context Protocol
2. It communicates via stdio (standard input/output)
3. When a client connects, it can list available tools
4. The client can call tools with parameters
5. The server executes the tool and returns results

## Common Issues

**"npm: command not found"**
- Make sure Node.js and npm are installed: `node --version && npm --version`

**TypeScript compilation errors**
- Run `npm install` to ensure all dependencies are installed
- Check that you have TypeScript 5.x installed

**Connection errors in MCP Inspector**
- Verify the server builds successfully: `npm run build`
- Check the path to `build/index.js` is correct
- Try running the server directly first: `node build/index.js`

## Next Steps

- Extend the server with more file operation tools (delete, move, rename)
- Add validation for file paths
- Implement file watching capabilities
- Add support for reading/writing binary files
- Create tools for directory operations
