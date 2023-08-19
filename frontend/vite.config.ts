import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react-swc';

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [react()],
    build: {
        manifest: true,
    },
    server: {
        port: 21012,
        host: '0.0.0.0',
        proxy: {
            // with options
            '/api': {
                target: 'http://localhost:9000',
                changeOrigin: true,
            },
            '/root': {
                target: 'http://localhost:9000',
                changeOrigin: true,
            },
            '/swagger-ui': {
                target: 'http://localhost:9000',
                changeOrigin: true,
            },
            '/redoc': {
                target: 'http://localhost:9000',
                changeOrigin: true,
            },
            '/rapidoc': {
                target: 'http://localhost:9000',
                changeOrigin: true,
            },
        },
    },
});
