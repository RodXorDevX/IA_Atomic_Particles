import http from 'http';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const server = http.createServer((req, res) => {
    // Resolver la ruta del archivo
    let filePath = req.url === '/' ? 'simple.html' : req.url;
    filePath = path.join(__dirname, filePath);

    // Obtener la extensión del archivo
    const ext = path.extname(filePath).toLowerCase();

    // Definir MIME types correctos
    const mimeTypes = {
        '.html': 'text/html; charset=utf-8',
        '.js': 'application/javascript; charset=utf-8',
        '.wasm': 'application/wasm',
        '.json': 'application/json',
        '.css': 'text/css; charset=utf-8',
        '.ts': 'application/typescript',
        '.d.ts': 'application/typescript'
    };

    // Establecer Content-Type correcto
    const contentType = mimeTypes[ext] || 'application/octet-stream';
    res.setHeader('Content-Type', contentType);
    
    // CORS headers para WASM
    res.setHeader('Access-Control-Allow-Origin', '*');
    res.setHeader('Access-Control-Allow-Methods', 'GET, OPTIONS');
    res.setHeader('Access-Control-Allow-Headers', 'Content-Type');

    // Manejar OPTIONS requests
    if (req.method === 'OPTIONS') {
        res.writeHead(200);
        res.end();
        return;
    }

    // Leer y servir el archivo
    fs.readFile(filePath, (err, data) => {
        if (err) {
            console.error(`Error: ${filePath} - ${err.message}`);
            res.writeHead(404, { 'Content-Type': 'text/plain' });
            res.end('404 - Archivo no encontrado\n' + filePath);
            return;
        }

        res.writeHead(200);
        res.end(data);
        console.log(`✅ ${req.url} (${ext})`);
    });
});

const PORT = 8000;
server.listen(PORT, () => {
    console.log('\n' + '='.repeat(50));
    console.log('🚀 Servidor WebAssembly activo');
    console.log('='.repeat(50));
    console.log(`📍 URL: http://localhost:${PORT}`);
    console.log(`📁 Directorio: ${__dirname}`);
    console.log('='.repeat(50) + '\n');
});

server.on('error', (err) => {
    console.error('❌ Error del servidor:', err.message);
});
