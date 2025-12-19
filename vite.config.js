export default {
  base: '/',
  root: './',
  publicDir: './',
  server: {
    port: 5173,
    strictPort: false,
    open: '/index.html'
  },
  build: {
    target: 'esnext',
    outDir: 'dist',
    rollupOptions: {
      input: {
        main: './index.html'
      }
    }
  }
}
