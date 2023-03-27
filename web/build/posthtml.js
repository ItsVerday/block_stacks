const posthtml = require("posthtml");
const fs = require("fs");
const uglify = require("uglify-js");

const html = fs.readFileSync("build/index.html");

function minifyJS(tree) {
    tree.match({ tag: "script" }, node => {
        node.content = [
            uglify.minify(node.content).code
        ];

        return node;
    });
}


const result = posthtml()
    .use(minifyJS)
    .process(html, { sync: true }).html;

fs.writeFileSync("build/index.html", result)