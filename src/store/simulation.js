import _cloneDeep from 'lodash/cloneDeep'
import _some from 'lodash/some'
import createWorker from '@/workers/simulation'
const worker = createWorker()

// const BEHAVIOURS = [
//   { name: 'WanderBehaviour' }
//   , { name: 'ScavengeBehaviour' }
//   , { name: 'BasicMoveBehaviour' }
//   , { name: 'HomesickBehaviour' }
//   , { name: 'StarveBehaviour' }
// ]

const DEFAULT_CREATURE_PROPS = {
  speed: [10, 1]
  , sense_range: [20, 0.5]
  , reach: [5, 0]
  , life_span: [1e4, 0]
  , energy: 5000
}

const initialState = {
  isBusy: false
  , startedAt: 0
  , computeTime: 0

  , config: {
    seed: 118
    , food_per_generation: 50
    , max_generations: 50
    , size: 500
    , preset: {
      name: 'default'
      , foods_before_home: 2
    }
  }
  , creatureConfig: {
    count: 50
    , template: DEFAULT_CREATURE_PROPS
  }
  , creatures: []
  , results: null
  , getResults: () => {}
}

function strTypeToNumber( val ){
  if ( typeof val === 'string' ){
    return +val
  }

  return val
}

// parses all strings as numbers
function sanitizeConfig( cfg ){
  return Object.keys(cfg).reduce((p, k) => {
    let value = cfg[k]
    if ( Array.isArray(value) ){
      value = value.map(strTypeToNumber)
    } else {
      value = strTypeToNumber( value )
    }

    p[k] = value
    return p
  }, {})
}

function getCreatureTemplate( creatureProps = DEFAULT_CREATURE_PROPS ){
  let props = sanitizeConfig(creatureProps)

  return {
    state: 'ACTIVE'
    , foods_eaten: 0
    , age: 0
    , energy_consumed: 0
    // gets overridden
    , pos: [0, 0]
    , home_pos: [0, 0]
    , movement_history: [[0, 0]]
    , ...props
  }
}

function runSimulation( config = {}, creatures = [] ){
  return worker.runSimulation({
    ...config
  }, creatures)
}

function continueSimulation( config = {}, creatures = [] ){
  return worker.continueSimulation({
    ...config
  }, creatures)
}

export const simulation = {
  namespaced: true
  , state: initialState
  , getters: {
    isLoading: state => state.isBusy
    , results: state => state.getResults()
    , lastGeneration: (state, getters) => {
      if (!getters.results) { return null }
      let g = getters.results.generations
      return g[g.length - 1]
    }
    , canContinue: (state, getters) => {
      if ( !getters.results ){ return false }
      let g = getters.results.generations
      return _some(g[g.length - 1].creatures, c => c.state !== 'DEAD')
    }
    , config: state => state.config
    , creatureConfig: state => state.creatureConfig
  }
  , actions: {
    run({ state, dispatch, commit }) {
      if ( state.isBusy ){ return Promise.reject(new Error('Busy')) }

      commit('start')
      return runSimulation(state.config, {
        count: state.creatureConfig.count | 0
        , template: getCreatureTemplate(state.creatureConfig.template)
      })
        .then(results => {
          commit('setResults', results)
        })
        .catch(error => {
          dispatch('error', { error, context: 'while calculating simulation results' }, { root: true })
        })
        .finally(() => commit('stop'))
    }
    , continue({ state, getters, dispatch, commit }) {
      if ( state.isBusy ){ return Promise.reject(new Error('Busy')) }
      if ( !getters.canContinue ){ return Promise.reject(new Error('No Results')) }

      commit('start')
      return continueSimulation(state.config, getters.lastGeneration.creatures)
        .then(results => {
          commit('appendResults', results)
        })
        .catch(error => {
          dispatch('error', { error, context: 'while calculating simulation results' }, { root: true })
        })
        .finally(() => commit('stop'))
    }
    , setConfig({ commit }, config = {}){
      commit('setConfig', _cloneDeep(config))
    }
    , setCreatureConfig({ commit }, config = {}){
      commit('setCreatureConfig', _cloneDeep(config))
    }
  }
  , mutations: {
    start(state){
      state.isBusy = true
      state.computeTime = 0
      state.startedAt = performance.now()
    }
    , stop(state){
      state.isBusy = false
      state.computeTime = performance.now() - state.startedAt
      state.startedAt = 0
    }
    , setResults(state, results){
      // state.results = results
      state.getResults = () => results
    }
    , appendResults(state, results){
      // state.results.generations = state.results.generations.concat(results.generations)
      let prevResults = state.getResults()
      prevResults.generations = prevResults.generations.concat(results.generations)
      state.getResults = () => prevResults
    }
    , setConfig(state, cfg){
      state.config = {
        ...state.config
        , ...sanitizeConfig(cfg)
      }
    }
    , setCreatureConfig(state, cfg){
      state.creatureConfig = {
        ...state.creatureConfig
        , ...cfg
      }
    }
    , setCreatures(state, creatures){
      state.creatures = creatures
    }
  }
}
