# Minidump Viewer Frontend

A SvelteKit-based frontend for the minidump viewer application, featuring TypeScript and Tailwind CSS.

## Development

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview

# Type checking
npm run check

# Linting and formatting
npm run lint
npm run format
```

## Features

- **Drag & Drop**: Drop `.dmp` or `.mdmp` files directly into the browser
- **WASM Integration**: Uses the Rust WASM module for minidump parsing
- **Responsive Design**: Built with Tailwind CSS for modern styling
- **TypeScript**: Full TypeScript support for type safety
- **Local Processing**: All file processing happens in the browser

## Usage

1. Start the development server with `npm run dev`
2. Open http://localhost:5173 in your browser
3. Drag and drop a minidump file or use the file input
4. View parsed results in the Overview, System, and Threads cards
