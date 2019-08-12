const ThreeWebpackPlugin = require('@wildpeaks/three-webpack-plugin')

module.exports = {
  publicPath: process.env.NODE_ENV === 'production'
    ? '/what-is-a-day/'
    : '/'
  , configureWebpack: {
    resolve: { symlinks: false }
    , node: {
      __dirname: true
    }
    ,plugins: [
  		new ThreeWebpackPlugin()
  	]
  }
  , css: {
    loaderOptions: {
      // pass options to sass-loader
      sass: {
        data: `@import '@/styles/_variables.scss'`
      }
    }
  }
}
