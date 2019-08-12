// based on https://github.com/zhanziyang/v-dragged
import drag from './drag'

export default {
  install: function (Vue) {
    // registration
    Vue.directive('drag', drag)
  }

  , drag
}
