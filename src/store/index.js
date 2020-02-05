import Vue from 'vue'
import Vuex from 'vuex'

import { alerts } from './alerts'
import { simulation } from './simulation'

Vue.use(Vuex)

const watchGeneration = (store) => {
  store.watch((state) => state.route && state.route.params.generationIndex, (index) => {
    if (index === undefined){ return }
    store.dispatch('simulation/loadGeneration', index - 1)
  })
}

export default new Vuex.Store({
  strict: process.env.NODE_ENV !== 'production'
  , plugins: [watchGeneration]
  , modules: {
    alerts
    , simulation
  }
})
