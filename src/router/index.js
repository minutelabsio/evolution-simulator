import Vue from 'vue'
import Router from 'vue-router'
import PlayerUI from '@/pages/player-ui'
import About from '@/pages/about'
import Simulation from '@/pages/simulation'
import SimulationStatistics from '@/components/simulation-statistics'
import ViewScreen from '@/components/view-screen'

// const CDN = 'https://cdn.minutelabs.io/what-is-a-day/audio'
//
// function getTracks(name){
//   return [
//     `${CDN}/${name}.mp3`
//     , `${CDN}/${name}.ogg`
//   ]
// }

Vue.use(Router)

export default new Router({
  routes: [
    {
      path: '/'
      , name: 'home'
      , component: PlayerUI
      , redirect: { name: 'simulation' }
      , meta: {
        // music: {
        //   maxVolume: 0.7
        //   , audio: [
        //     `${CDN}/Candlepower.mp3`
        //     , `${CDN}/Candlepower.ogg`
        //   ]
        // }
      }
      , children: [
        // {
        //   path: 'welcome'
        //   , name: 'welcome'
        //   , component: () => import('@/components/chapters/welcome')
        //   , meta: {
        //     title: 'Welcome'
        //     , audio: getTracks('welcome')
        //   }
        // }
      ]
    }
    , {
      path: '/s/:generationIndex?'
      , name: 'simulation'
      , redirect: { name: 'viewscreen' }
      , component: Simulation
      , props(route){
        return {
          ...route.params
          , showConfig: !!route.query.cfg
          , generationIndex: +route.params.generationIndex
        }
      }
      , children: [
        {
          path: 'viewer'
          , name: 'viewscreen'
          , component: ViewScreen
          , props: true
        }
        , {
          path: 'stats'
          , name: 'stats'
          , component: SimulationStatistics
          , props: true
        }
      ]
    }
    , {
      path: '/about'
      , name: 'about'
      , component: About
    }
    , {
      path: '*'
      , redirect: { name: 'viewscreen' }
    }
  ]
})
