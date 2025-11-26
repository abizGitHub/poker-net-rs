const path = require('path')
const WP = require('html-webpack-plugin')
const WSMP = require('@wasm-tool/wasm-pack-plugin')
const { experiments } = require('webpack')

module.exports = {
    entry: './public/main.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js'
    },
    plugins: [
        new WP({ template: './public/index.html' }),
        new WSMP({ crateDirectory: path.resolve(__dirname, '.') })
    ],

    experiments: {
      asyncWebAssembly: true 
    }

} 