const path = require('path');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

const distPath = path.resolve(__dirname, 'dist');

module.exports = {
    mode: 'production',
    devServer: {
        port: 8000,
    },
    entry: './bootstrap.js',
    output: {
        path: distPath,
        filename: 'rust-yew-websocket.js',
        webassemblyModuleFilename: 'rust-yew-websocket_bg.wasm',
    },
    plugins: [
        new CopyWebpackPlugin({
            patterns: [{ from: './static', to: distPath }],
        }),
        new WasmPackPlugin({
            crateDirectory: '.',
            extraArgs: '-- --features wee_alloc',
            outName: 'rust-yew-websocket',
        }),
    ],
    experiments: {
        asyncWebAssembly: true,
    },
};