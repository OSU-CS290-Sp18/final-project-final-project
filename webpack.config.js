const path = require('path');

module.exports = {
  entry: ['babel-polyfill', './web/app.js'],

  output: {
    filename: 'unify.js',
    path: path.resolve(__dirname, 'public')
  },

  devServer: {
    contentBase: path.join(__dirname, 'public'),
    compress: true,
    port: 8000,
    proxy: {
      '/api': 'http://127.0.0.1:8080'
    }
  },

  module: {
    rules: [
      {
        test: /\.js$/,
        exclude: /node_modules/,
        use: {
          loader: 'babel-loader'
        }
      },
      {
        test: /\.scss$/,
        use: [
          {
            loader: 'style-loader',
          },
          {
            loader: 'css-loader',
          },
          {
            loader: 'sass-loader'
          }
        ]
      }
    ]
  }
};
