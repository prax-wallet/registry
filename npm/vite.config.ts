import { defineConfig } from 'vite';
import { commitInfoPlugin } from './src/utils/commit-info-vite-plugin';
import dts from 'vite-plugin-dts';

export default defineConfig({
  build: {
    lib: {
      entry: ['./src/index.ts'],
      formats: ['es', 'cjs'],
    },
  },
  plugins: [commitInfoPlugin(), dts({ rollupTypes: true })],
});
