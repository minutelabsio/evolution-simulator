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
  , reach: [1, 0]
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

  , canContinue: true
  , statistics: null
  , currentGenerationIndex: 0
  , currentGeneration: null
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

export const simulation = {
  namespaced: true
  , state: initialState
  , getters: {
    isLoading: state => state.isBusy
    , canContinue: state => state.canContinue
    , config: state => state.config
    , creatureConfig: state => state.creatureConfig
    , currentGeneration: state => state.currentGeneration
    , currentGenerationIndex: state => state.currentGenerationIndex
    , statistics: state => state.statistics
  }
  , actions: {
    async run({ state, dispatch, commit }) {
      if ( state.isBusy ){ return Promise.reject(new Error('Busy')) }

      commit('start')
      try {
        await worker.initSimulation(state.config, {
          count: state.creatureConfig.count | 0
          , template: getCreatureTemplate(state.creatureConfig.template)
        })

        await worker.advanceSimulation(state.config.max_generations)
        commit('setMeta', {
          canContinue: await worker.canContinue()
        })
        commit('setStatistics', await worker.getStatistics())

        await dispatch('loadGeneration', 0)

      } catch ( error ){
        dispatch('error', { error, context: 'while calculating simulation results' }, { root: true })
      } finally {
        commit('stop')
      }
    }
    , async continue({ state, getters, dispatch, commit }) {
      if ( state.isBusy ){ return Promise.reject(new Error('Busy')) }
      if ( !getters.canContinue ){ return Promise.reject(new Error('No Results')) }

      commit('start')
      try {
        await worker.advanceSimulation(state.config.max_generations)
        commit('setMeta', {
          canContinue: await worker.canContinue()
        })
        commit('setStatistics', await worker.getStatistics())

        await dispatch('loadGeneration', state.currentGenerationIndex)

      } catch ( error ){
        dispatch('error', { error, context: 'while calculating simulation results' }, { root: true })
      } finally {
        commit('stop')
      }
    }
    , async loadGeneration({ state, commit }, idx){
      idx = Math.min(idx, state.statistics.num_generations)
      commit('setGenerationIndex', idx)
      commit('setGeneration', await worker.getGeneration(idx))
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
    , setGenerationIndex(state, idx){
      state.currentGenerationIndex = idx | 0
    }
    , setMeta(state, meta){
      Object.keys(meta).forEach(k => {
        state[k] = meta[k]
      })
    }
    , setStatistics(state, stats){
      state.statistics = Object.freeze(stats)
    }
    , setGeneration(state, gen){
      state.currentGeneration = Object.freeze(gen)
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
  }
}
