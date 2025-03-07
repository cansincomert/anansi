//Webpack requires this to work with directories
const path = require('path');
const CircularDependencyPlugin = require('circular-dependency-plugin')
// This is main configuration object that tells Webpackw what to do. 
module.exports = {
    //path to entry paint
    // stats: {warnings:false},
    entry: './index.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: "bundle.js",
        // filename: (pathData) =>
        //     `${pathData.chunk.name}.${process.env.npm_package_version}.js`,
    },
    experiments: {
        topLevelAwait: true,
        syncWebAssembly: true
    },
    performance: {
        hints: false,
        maxEntrypointSize: 512000,
        maxAssetSize: 512000
    },
    devServer: {
        https: false,
        static: {
            directory: path.join(__dirname, 'dist'),
        },
        // we need this in order to enable SharedArrayBuffer
        headers: {
            "Access-Control-Allow-Origin": "*",
            "Cross-Origin-Embedder-Policy": "require-corp",
            "Cross-Origin-Opener-Policy": "same-origin",
        },
        compress: true,
        port: 9000,
        host: 'localhost',
    },
    plugins: [
        new CircularDependencyPlugin({
            // `onStart` is called before the cycle detection starts
            onStart({ compilation }) {
                console.log('start detecting webpack modules cycles');
            },
            // `onDetected` is called for each module that is cyclical
            onDetected({ module: webpackModuleRecord, paths, compilation }) {
                // `paths` will be an Array of the relative module paths that make up the cycle
                // `module` will be the module record generated by webpack that caused the cycle
                console.log(paths);
                compilation.errors.push(new Error(paths.join(' -> ')))
            },
            // `onEnd` is called before the cycle detection ends
            onEnd({ compilation }) {
                console.log('end detecting webpack modules cycles');
            },
        })
    ],
    module: {
        rules: [
            {
                test: /\.js$/,
                exclude: /(node_modules)/,
                use: {
                    loader: 'babel-loader',
                    options: {
                        presets: ['@babel/preset-env']
                    }
                }
            },
            {
                test: /\.css$/,
                use: ['style-loader', 'css-loader']
            }

        ],
    },


    //default mode is production
    mode: 'production'
}