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
