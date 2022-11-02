const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const json5 = require('json5');
const { VueLoaderPlugin } = require('vue-loader');
const WebpackTools = require("./WebpackTools");
const config = require("./config");

const webpackTools = new WebpackTools(config);
// console.log(path.resolve(__dirname, "../src"), 'jijiji');

module.exports = Object.assign({
    mode: config.isDebug ? "development" : "production",
    entry: webpackTools.entry,
    output: {
        filename: "js/[name].js",
        path: path.resolve(__dirname, "../dist"),
        clean: true,
        chunkFilename: `js/[id].bundle.js?[hash]`
    },
    resolve: {
        alias: {
            "@": path.resolve(__dirname, "../src"),
        }
    },
    module: {
        rules: [
            {
                test: /\.json5$/,
                parser: {
                    parse: json5.parse
                },
                generator: {
                    filename: 'data/[name][ext]' // 局部指定输出位置
                }
            },
            {
                test: /\.vue$/,
                use: [
                    {
                        loader: 'vue-loader'
                    }
                ]
            },
            {
                test: /\.s[ac]ss$/i,
                use: [
                    'style-loader', // 将 JS 字符串生成为 style 节点
                    'css-loader',   // 将 CSS 转化成 CommonJS 模块
                    'sass-loader',  // 将 Sass 编译成 CSS
                ],
            },
            {
                test: /\.(png|svg|jpg|gif|jpeg|ttf|woff|woff2|eot)$/i,
                type: 'asset/resource',
                generator: {
                    filename: 'images/[hash][ext]' // 局部指定输出位置
                }
            },
            {
                test: /\.template$/i,
                use: [ "html-loader" ]
            }
        ]
    },
    performance: {
        hints: false,
        maxEntrypointSize: 512000,
        maxAssetSize: 512000
    },
    watchOptions: {
        ignored: [ "**/node_modules" ],
    },
    plugins: [
        ...webpackTools.html.map(item => new HtmlWebpackPlugin(item)),
        new VueLoaderPlugin()
    ]
}, config.isDebug && {
    devtool: 'inline-source-map'
} || {});