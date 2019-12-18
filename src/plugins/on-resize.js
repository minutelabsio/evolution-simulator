import _throttle from 'lodash/throttle'
import _pull from 'lodash/pull'

export default {
  install( Vue ){
    const listeners = []
    Vue.prototype.$onResize = function( fn, throttleTime = 50 ){
      const cb = _throttle(fn, throttleTime)
      listeners.push(cb)
      this.$on('hook:beforeDestroy', () => {
        _pull(listeners, cb)
      })
    }

    window.addEventListener('resize', () => {
      for (let i = 0, l = listeners.length; i < l; i++){
        listeners[i]()
      }
    })
  }
}
