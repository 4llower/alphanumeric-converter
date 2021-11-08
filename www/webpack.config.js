const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin(['./public/index.html', './public/favicon.png', './public/swap.png'])
  ],
  module: {
    rules: [
      {
        test: /\.css$/i,
        use: ["style-loader", "css-loader"],
      },
      {
        loader: 'file-loader',
        test: /\.(eot|otf|svg|ttf|woff|woff2|png|pdf|ico|gif)$/,
      },
    ],
  },
};
