import router from '../router'
import _cloneDeep from 'lodash/cloneDeep'
// import _some from 'lodash/some'
import traitColors from '@/config/trait-colors'
import createWorker from '@/workers/simulation'
const worker = createWorker()

// const BEHAVIOURS = [
//   { name: 'WanderBehaviour' }
//   , { name: 'ScavengeBehaviour' }
//   , { name: 'BasicMoveBehaviour' }
//   , { name: 'HomesickBehaviour' }
//   , { name: 'StarveBehaviour' }
// ]

function getTraitsForPreset(){
  return [ 'age', 'speed', 'size', 'sense_range' ]
}

const DEFAULT_CREATURE_PROPS = {
  speed: [10, 0.5]
  , size: [10, 0.5]
  , sense_range: [20, 0.5]
  , reach: [1, 0]
  , flee_distance: [1e12, 0] // for now flee within sight range
  , life_span: [1e4, 0]
  , energy: 5000000
}

const initialState = {
  isBusy: false
  , isRestarting: false
  , isContinuing: false
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
  , currentGenerationIndex: -1
  , getCurrentGeneration: () => null
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
    , foods_eaten: []
    , age: 0
    , energy_consumed: 0
    // gets overridden
    , pos: [0, 0]
    , home_pos: [0, 0]
    , movement_history: [[0, 0]]
    , status_history: []
    , id: '00000000000000000000000000000000'
    , ...props
  }
}

export const simulation = {
  namespaced: true
  , state: initialState
  , getters: {
    isLoading: state => state.isRestarting
    , isContinuing: state => state.isContinuing
    , isBusy: state => state.isBusy
    , canContinue: state => state.canContinue
    , config: state => state.config
    , creatureConfig: state => state.creatureConfig
    , creatureTemplate: state => state.creatureConfig.template
    , creatureCount: state => state.creatureConfig.count
    , getCurrentGeneration: state => state.getCurrentGeneration
    , currentGenerationIndex: (state, getters, rootState) =>
        rootState.route ? +rootState.route.params.generationIndex - 1 : 0
    , statistics: state => state.statistics

    , traits: state => getTraitsForPreset(state.config.preset.name)
    , traitColors: (state, getters) => getters.getTraitColors(getters.traits)
    , getTraitColors: () => (traits) => traits.map(k => traitColors[k])
  }
  , actions: {
    async run({ state, dispatch, commit, getters }, fresh = true) {
      if ( state.isBusy ){ return Promise.reject(new Error('Busy')) }

      let preload = fresh ? 1 : getters.currentGenerationIndex + 1
      let postload = state.config.max_generations - preload

      commit('start', true)
      try {
        await worker.initSimulation(state.config, {
          count: state.creatureConfig.count | 0
          , template: getCreatureTemplate(state.creatureConfig.template)
        })

        await worker.advanceSimulation(preload)
        commit('setMeta', {
          canContinue: await worker.canContinue()
        })
        commit('setStatistics', await worker.getStatistics())

        await dispatch('loadGeneration', fresh ? 0 : getters.currentGenerationIndex)

      } catch ( error ){
        dispatch('error', { error, context: 'while calculating simulation results' }, { root: true })
      } finally {
        commit('stop')
      }

      if (postload) {
        dispatch('continue', postload)
      }
    }
    , async continue({ state, getters, dispatch, commit }, numGenerations) {
      if ( state.isBusy ){ return Promise.reject(new Error('Busy')) }
      if ( !getters.canContinue ){ return Promise.reject(new Error('No Results')) }

      commit('start')
      try {
        await worker.advanceSimulation((numGenerations | 0) || state.config.max_generations)
        commit('setMeta', {
          canContinue: await worker.canContinue()
        })
        commit('setStatistics', await worker.getStatistics())

        // await dispatch('loadGeneration', getters.currentGenerationIndex)

      } catch ( error ){
        dispatch('error', { error, context: 'while calculating simulation results' }, { root: true })
      } finally {
        commit('stop')
      }
    }
    , async loadGeneration({ state, commit, getters }, idx){
      if (!state.statistics) { return }
      idx = Math.max(0, Math.min(idx, state.statistics.num_generations - 1))

      if ( idx !== getters.currentGenerationIndex ){
        let route = router.history.current
        let params = route ? route.params : {}
        let query = route ? route.query : {}
        router.replace({ params: { ...params, generationIndex: idx + 1 }, query })
      }

      commit('setGenerationIndex', idx)
      commit('setGeneration', await worker.getGeneration(idx))
    }
    , setConfig({ commit }, config = {}){
      commit('setConfig', _cloneDeep(config))
    }
    , setCreatureConfig({ commit }, config = {}){
      commit('setCreatureConfig', _cloneDeep(config))
    }
    , setCreatureTemplate({ commit }, config = {}){
      commit('setCreatureTemplate', _cloneDeep(config))
    }
    , setCreatureCount({ commit }, count){
      commit('setCreatureCount', count | 0)
    }
  }
  , mutations: {
    start(state, isRestart){
      state.isBusy = true
      state.isRestarting = !!isRestart
      state.isContinuing = !isRestart
      state.computeTime = 0
      state.startedAt = performance.now()
    }
    , stop(state){
      state.isBusy = false
      state.isRestarting = false
      state.isContinuing = false
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
      let generation = Object.freeze(gen)
      state.getCurrentGeneration = () => generation
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
    , setCreatureTemplate(state, cfg){
      state.creatureConfig = {
        ...state.creatureConfig
        , template: {
          ...state.creatureConfig.template
          , ...cfg
        }
      }
    }
    , setCreatureCount(state, count){
      state.creatureConfig = {
        ...state.creatureConfig
        , count
      }
    }
  }
}
