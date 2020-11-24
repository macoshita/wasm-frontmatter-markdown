const parser = require("./pkg");

const testStr = `---
test: hoge
test2: fuga
test3: piyo
---

:innocent:

\`\`\`ts
// :innocent:
import path from "path";
import mkdirp from "mkdirp";
import puppeteer from "puppeteer";
import { fetchPostContent, PostContent } from "./src/lib/posts";

(async () => {
  const ogImageDir = path.resolve(process.cwd(), "./dist/og-image/");
  await mkdirp(ogImageDir);

  const browser = await puppeteer.launch();
  const page = await browser.newPage();
  page.setViewport({ width: 1200, height: 630 });
\`\`\`
`;

console.time("1st");
console.log(parser.parse(testStr, { frontmatter: true, content: true }));
console.timeEnd("1st");

console.time("2nd");
console.log(parser.parse(testStr, { frontmatter: false, content: true }));
console.timeEnd("2nd");

console.time("3rd");
console.log(parser.parse(testStr, { frontmatter: true, content: false }));
console.timeEnd("3rd");
