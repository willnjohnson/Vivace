import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  
  // Tauri expects a fixed port for the dev server
  server: {
    port: 3000,
    strictPort: true,
  },
  
  // Build configuration for Tauri
  build: {
    // Tauri uses Chromium on Windows and WebKit on macOS and Linux
    target: process.env.TAURI_PLATFORM == 'windows' ? 'chrome105' : 'safari13',
    // Don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    // Produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG,
  },
  
  // Prevent vite from obscuring rust errors
  clearScreen: false,
  
  // Environment variables
  envPrefix: ['VITE_', 'TAURI_'],
})
