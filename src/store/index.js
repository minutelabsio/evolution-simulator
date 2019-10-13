import Vue from 'vue'
import Vuex from 'vuex'

import { alerts } from './alerts'
import { simulation } from './simulation'

Vue.use(Vuex)

export default new Vuex.Store({
  strict: process.env.NODE_ENV !== 'production'
  , plugins: []
  , modules: {
    alerts
    , simulation
  }
})
