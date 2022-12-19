import { resolve } from 'path';
import HtmlWebpackPlugin from 'html-webpack-plugin';
import { ProvidePlugin } from 'webpack';
import WasmPackPlugin from '@wasm-tool/wasm-pack-plugin';

export default {
  entry: resolve(__dirname, 'src', 'index.ts'),
  output: {
    path: resolve(__dirname, 'dist'),
    filename: 'index.js',
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: resolve(__dirname, 'public', 'index.html')
    }),
    new WasmPackPlugin({
      crateDirectory: resolve(__dirname, "wasm")
    }),
    // Have this example work in Edge which doesn't ship `TextEncoder` or
    // `TextDecoder` at this time.
    new ProvidePlugin({
      TextDecoder: ['text-encoding', 'TextDecoder'],
      TextEncoder: ['text-encoding', 'TextEncoder']
    })
  ],
  module: {
    rules: [
      {
        test: /\.css$/,
        use: ['css-loader']
      }
    ]
  },
  mode: 'development',
  experiments: {
    asyncWebAssembly: true
  }
};


// export default {
//   entry: resolve(__dirname, 'src', 'index.ts'),
//   output: {
//     filename: 'main.js',
//     path: resolve(__dirname, 'build')
//   },
//   plugins: [
//     new HtmlWebpackPlugin({
//       template: resolve(__dirname, 'public', 'index.html')
//     }),
//     new WasmPackPlugin({
//       crateDirectory: resolve(__dirname, 'wasm'),
//     }),
//     new ProvidePlugin({
//       TextDecoder: ['text-encoding', 'TextDecoder'],
//       TextEncoder: ['text-encoding', 'TextEncoder']
//     })
//   ],
//   resolve: {
//     modules: [__dirname, 'src', 'node_modules'],
//     extensions: ['*', '.js', '.jsx', '.tsx', '.ts']
//   },
//   module: {
//     rules: [
//       {
//         test: /\.ts$/,
//         exclude: /node_modules/,
//         use: ['ts-loader']
//       },
//       {
//         test: /\.css$/,
//         use: ['css-loader']
//       }
//     ]
//   },
//   devServer: {
//     port: 3000,
//     historyApiFallback: true
//   },
//   experiments: {
//     asyncWebAssembly: true
//   }
// };
