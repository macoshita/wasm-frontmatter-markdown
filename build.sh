#!/usr/bin/env bash
wasm-pack build --scope macoshita -t nodejs
node -e "
const fs = require('fs');
const f = fs.readFileSync('pkg/package.json', 'utf8');
const j = {...JSON.parse(f), homepage: 'https://github.com/macoshita/wasm-frontmatter-markdown'};
fs.writeFileSync('pkg/package.json', JSON.stringify(j), 'utf8');
"
