const path = require('path')
const WP = require('html-webpack-plugin')
const WSMP = require('@wasm-tool/wasm-pack-plugin')
const { experiments } = require('webpack')

module.exports = {
    entry: {
      index:  './public/index.js',
      pocker_room: './public/pocker_room.js'
    },
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: "[name].bundle.js"
    },
    plugins: [
        new WP({ 
            template: './public/index.html' ,
            filename: "index.html",
            chunks: ["index"]
        }),
        new WP({ 
            template: './public/pocker_room.html',
            filename: "pocker_room.html",
            chunks: ["pocker_room"]
        }),
        new WSMP({ crateDirectory: path.resolve(__dirname, '.') })
    ],

    experiments: {
      asyncWebAssembly: true 
    },

    devServer: {
        static: "./public",
        historyApiFallback: true,
        port: 8080
      },
} 