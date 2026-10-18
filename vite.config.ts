import react from '@vitejs/plugin-react'
import { defineConfig } from 'vite'

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [react()],
  // Vite options tailored for Tauri to prevent too much magic
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // using polling since WSL2 has very slow fs events at the file system level
      // (reference: https://github.com/microsoft/WSL/issues/4739)
      usePolling: true,
      interval: 600,
    },
  },
}))
