const fs = require("fs");
const path = require("path");

function genEntry(config) {
    const isDebug = config.isDebug;
    isDebug && console.log(`=== start genEntry ===`);
    const views = config.views;
    const entry = {};
    for (let i = 0; i < views.length; i++) {
        let entry_path = path.join(__dirname, `${config.PATH_ROOT}/${views[i]}/index.js`);
        isDebug && console.log(`gen a entry ---> ${entry_path}`);
        entry[views[i]] = entry_path;
    }
    isDebug && console.log(`=== finish genEntry ===`);
    return entry;
}

function genHTML(config) {
    const isDebug = config.isDebug;
    isDebug && console.log(`=== start genHTML ===`);
    const views = config.views;
    const html = [];
    const temp_path = path.join(__dirname, `${config.HTML_TEMP_PATH}`);
    isDebug && console.log(`html temp path ---> ${temp_path}`);
    for (let i = 0; i < views.length; i++) {
        html.push({
            filename: `${views[i]}.html`,
            template: temp_path,
            title: config.TITLE,
            chunks: [ views[i] ]
        });
    }
    isDebug && console.log(`=== finish genHTML ===`);
    return html;
}

class WebpackTools {
    constructor(config) {
        this.entry = genEntry(config);
        this.html = genHTML(config);
    }
}
module.exports = WebpackTools;