import { defineConfig } from 'vite';
import { resolve } from 'path';

export default defineConfig({
  root: 'src',
  build: {
    outDir: '../dist',
    emptyOutDir: true,
    rollupOptions: {
      input: {
        index: resolve(__dirname, 'src/index.html'),
        reminder: resolve(__dirname, 'src/reminder.html'),
        pet: resolve(__dirname, 'src/pet.html'),
      },
    },
  },
  server: {
    strictPort: true,
    port: 5174,
  },
});
