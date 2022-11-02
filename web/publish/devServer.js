const Webpack = require("webpack");
const WebpackDevServer = require("webpack-dev-server");
const path = require("path");
const webpackConfig = require("./webpack.config.js");
const config = require("./config");
config.isDebug = true;
const views = config.views;

const compiler = Webpack(webpackConfig);

const server = new WebpackDevServer({
    ...config.devOption
}, compiler);

const runServer = async () => {
    console.log('Starting server...');
    await server.start();
};

runServer().then(r => {
    console.log(`webpack server run on port ${config.devOption.port} successfully~`);
});