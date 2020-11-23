# wasm-frontmatter-markdown

## About

This repository is for [my blog](https://macoshita.me).

- Parse frontmatter markdown
- Convert frontmatter to json
- Convert content to html
    - Syntax highlighting

## Attention

When Syntax hilighting of TypeScript at first is too slow.
Maybe Syntect and fancy regex is slow.

## Usage

```sh
npm i wasm-frontmatter-markdown
```

```js
const parser = require("@macoshita/wasm-frontmatter-markdown");

const options = {
  frontmatter: true,
  content: true,
};

parser.parse(
  `---
hello: world
---

# hoge

\`\`\`js
const hello = 'world'
\`\`\`
`,
  options
);
```

## Build

Install nodejs and [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) and build

```
./build.sh
```

## Publish

```
wasm-pack publish -a public
```
