import Vue from 'vue'
import Router from 'vue-router'
import PlayerUI from '@/pages/player-ui'
import About from '@/pages/about'
import Playground from '@/pages/playground'

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
      , name: 'player'
      , component: PlayerUI
      , redirect: { name: 'welcome' }
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
      path: '/playground'
      , name: 'playground'
      , component: Playground
    }
    , {
      path: '/about'
      , name: 'about'
      , component: About
    }
    , {
      path: '*'
      , redirect: 'welcome'
    }
  ]
})
