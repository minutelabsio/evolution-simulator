import stringify from 'csv-stringify'
import _get from 'lodash/get'

const app = import(/* webpackPreload: true */ '@/wasm/pkg/app')
app.then( mod => {
  console.log('setting debug hook')
  mod.browser_debug()
})

// Helpers
// ---------------------------------------
// function log( ...args ){
//   if ( process.env.NODE_ENV !== 'production' ){
//     console.log.apply(console, args)
//   }
// }

// function logErr( ...args ){
//   if ( process.env.NODE_ENV !== 'production' ){
//     console.error.apply(console, args)
//   }
// }

// async function run( method, ...args ){
//   const wasm = await app

//   const fn = wasm[method]
//   if ( !fn ){
//     throw new Error(`Method ${method} not defined`)
//   }

//   try {
//     log(`Worker: Running ${method}`, args)
//     return fn.apply( wasm, args )
//   } catch( msg ){
//     // wasm bingen doesn't throw true js errors....
//     let err = new Error( msg )
//     logErr(`Worker: ERROR from ${method}`, err)
//     throw err
//   }
// }

// API
// ---------------------------------------
let simulation = null
export async function initSimulation( cfg, creatureCfgs = [] ){
  const wasm = await app
  simulation = wasm.SquareWorld.new(
    cfg.size
    , cfg.seed
    , cfg.food_per_generation
    , cfg.preset
  )

  creatureCfgs.forEach(cfg => {
    simulation.add_creatures(cfg)
  })
}

export function advanceSimulation( numGens ){
  return simulation.run( numGens )
}

export function canContinue(){
  return simulation.can_continue()
}

export function getResults(){
  return simulation.get_results()
}

export function getGeneration(index){
  return simulation.get_generation(index)
}

export function getStatistics(species){
  return simulation.get_statistics(species)
}

const CSVCOLUMNS = [
  { key: 'generation', label: 'Generation' }
  , { key: 'population', label: 'Population' }
  , { key: 'age.mean', label: 'Mean Age' }
  // , { key: 'life_span.mean', label: 'Mean Lifespan' }
  , { key: 'age_at_death.mean', label: 'Mean Age at death' }
  , { key: 'births', label: 'Births' }
  , { key: 'deaths', label: 'Deaths' }
  , { key: 'creatures_eaten', label: 'Creatures Eaten' }
  , { key: 'food_balls_available', label: 'Food Balls Available' }
  , { key: 'food_balls_eaten', label: 'Food Balls Eaten' }
  , { key: 'sense_range.mean', label: 'Mean Sense Range' }
  , { key: 'size.mean', label: 'Mean Size' }
  , { key: 'speed.mean', label: 'Mean Speed' }
]

export function getCSV(species){
  let stats = simulation.get_statistics(species)
  let data = stats.generation_statistics.map((entry, i) => {
    let row = {}
    entry.generation = i + 1
    CSVCOLUMNS.forEach(({ key, label }) => {
      row[label] = _get(entry, key)
    })
    return row
  })
  let columns = CSVCOLUMNS.map(({ label }) => label)
  return new Promise((resolve, reject) => {
    stringify(
      data
      , { columns, header: true }
      , (err, data) => {
        if (err){ return reject(err) }
        resolve( data )
      }
    )
  })
}
