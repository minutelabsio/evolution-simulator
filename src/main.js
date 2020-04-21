import Vue from 'vue'
import App from '@/app'
import VueAnalytics from 'vue-analytics'
import router from '@/router'
import store from '@/store'
import { sync } from 'vuex-router-sync'
import Filters from '@/plugins/filters'
import Gestures from '@/plugins/gestures'
import onResize from '@/plugins/on-resize'
import Buefy from 'buefy'
import VueCircleSlider from 'vue-circle-slider'
import Copilot from 'copilot'
import * as THREE from 'three'

import '@mdi/font/css/materialdesignicons.css'
// require styles
import './styles/main.scss'

import VueSlider from 'vue-slider-component'
import 'vue-slider-component/theme/default.css'
import labConfig from '../lab-config'


Vue.component('VueSlider', VueSlider)

// sync router and store
sync(store, router)

Copilot.registerType({
  type: 'Vector3'
  , default: new THREE.Vector3()
  , interpolator: (from, to, t) => {
    let v = new THREE.Vector3()
    v.copy( from )
    return v.lerp( to, t )
  }
})

const isProduction = process.env.NODE_ENV === 'production'
Vue.use(VueAnalytics, {
  id: 'UA-46248430-1'
  , router
  , debug: {
    enabled: !isProduction
    , trace: false
    , sendHitTask: isProduction
  }
  , autoTracking: {
    exception: true
    , pageviewTemplate (route) {
      return {
        title: `${route.name} | ${labConfig.title}`
        , page: `/${labConfig.repo}${route.path}`
        , location: window.location.href
      }
    }
  }
})

Vue.use(Buefy, {
  defaultContainerElement: '#app .below-nav'
  // , defaultIconPack: 'fas'
})

Vue.use(VueCircleSlider)
// Vue.use(ElementComponents)
Vue.use(Filters)
Vue.use(Gestures)
Vue.use(onResize)

Vue.config.productionTip = false

new Vue({
  render: h => h(App)
  , router
  , store
}).$mount('#app')
