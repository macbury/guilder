const path = require('path');
const { lessLoader } = require('esbuild-plugin-less');

const env = process.env.NODE_ENV || 'development';
const isProduction = env != 'development';


const watch = isProduction ? false : {
  onRebuild(error, result) {
    if (error) console.error('watch build failed:', error)
    else console.log('watch build succeeded')
  },
};
// src/index.tsx --watch --bundle '--define:process.env.NODE_ENV=\"development\"' --sourcemap --outfile=../public/bundle.js
require('esbuild').build({
  entryPoints: ['./src/index.jsx'],
  bundle: true,
  watch,
  minify: isProduction,
  sourcemap: !isProduction,
  define: {
    "process.env.NODE_ENV": `"${env}"`
  },
  outfile: path.resolve(__dirname, '../../public/bundle.js'),
  plugins: [lessLoader()],
  loader: {
    '.ts': 'ts',
    '.tsx': 'tsx',
  },
}).then(result => {
  console.log('watching...')
}).catch(() => process.exit(1))
