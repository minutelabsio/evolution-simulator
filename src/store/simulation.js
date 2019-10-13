import createWorker from '@/workers/simulation'
const worker = createWorker()

const BEHAVIOURS = [
  { name: 'WanderBehaviour' }
  , { name: 'ScavengeBehaviour' }
  , { name: 'BasicMoveBehaviour' }
  , { name: 'HomesickBehaviour' }
  , { name: 'StarveBehaviour' }
]

const DEFAULT_CREATURE_PROPS = {
  speed: [6, 1]
  , sense_range: [50, 10]
  , reach: [5, 1]
  , life_span: [4, 1]
  , energy: 500
}

const initialState = {
  isBusy: false
  , startedAt: 0
  , computeTime: 0

  , config: {
    seed: 124
    , food_per_generation: 50
    , max_generations: 50
    , behaviours: BEHAVIOURS.concat([])
    , size: 500
  }
  , creatureCfg: {
    count: 50
    , properties: {
      speed: [6, 1]
      , sense_range: [50, 10]
      , reach: [5, 1]
      , life_span: [4, 1]
      , energy: 500
    }
  }
  , creatures: []
  , results: null
}

function getCreatureTemplate( creatureProps = DEFAULT_CREATURE_PROPS ){
  let props = Object.keys(creatureProps).reduce((p, k) => {
    let value = creatureProps[k]
    if ( Array.isArray(value) ){
      value = value.map(n => +n)
    }

    if ( typeof value === 'string' ){
      value = +value
    }

    p[k] = value
    return p
  }, {})

  return {
    state: 'ACTIVE'
    , foods_eaten: 0
    , age: 0
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

export const simulation = {
  namespaced: true
  , state: initialState
  , getters: {
    selectableBehaviours: () => BEHAVIOURS.concat([])
    , results: state => state.results
  }
  , actions: {
    run({ state, dispatch, commit }) {
      if ( state.isBusy ){ return Promise.reject(new Error('Busy')) }

      commit('start')
      return runSimulation(state.config, state.creatures)
        .then(results => {
          commit('setResults', results)
        })
        .catch(error => {
          dispatch('error', { error, context: 'while calculating simulation results' }, { root: true })
        })
        .finally(() => commit('stop'))
    }
    , setConfig({ commit }, config = {}){
      commit('setConfig', config)
    }
    , setCreatureConfig({ commit }, config = {}){
      commit('setCreatureConfig', config)
    }
    , randomizeCreatures({ state, commit, dispatch }){
      return worker.initRandomCreatures(state.config, {
        count: state.creatureCfg.count
        , template: getCreatureTemplate(state.creatureCfg.properties)
      })
        .then(creatures => {
          commit('setCreatures', creatures)
        })
        .catch(error => {
          dispatch('error', { error, context: 'while randomizing creatures' }, { root: true })
        })
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
      state.results = results
    }
    , setConfig(state, cfg){
      state.config = {
        ...state.config
        , ...cfg
      }
    }
    , setCreatureConfig(state, cfg){
      state.creatureCfg = {
        ...state.creatureCfg
        , ...cfg
      }
    }
    , setCreatures(state, creatures){
      state.creatures = creatures
    }
  }
}
