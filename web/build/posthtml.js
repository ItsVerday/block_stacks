const posthtml = require("posthtml");
const posthtmlInlineAssets = require("posthtml-inline-assets");
const fs = require("fs");

async function run() {
    const result = await posthtml([
        posthtmlInlineAssets({
            root: "./"
        })
    ]).process(fs.readFileSync("build/index.html"));
    console.log(result.html);
}

run();