<template lang="pug">
Player.player(:playlist="playlist", :play-index="playIndex", :music="music")
  template(slot-scope="playerProps")
    .main(ref="main")
      router-view(v-if="viewWidth", :player-loading="playerProps.loading", :view-width="viewWidth", :view-height="viewHeight")
    .panel-bottom
      PlayerControls()
</template>

<script>
// import Promise from 'bluebird'
// import PubSub from '@/lib/pubsub'
import _debounce from 'lodash/debounce'
import Player from '@/components/player'
import PlayerControls from '@/components/player-controls'

export default {
  name: 'PlayerUI'
  , props: {

  }
  , components: {
    Player
    , PlayerControls
  }
  , data: () => ({
    viewWidth: 0
    , viewHeight: 0
  })
  , created(){
    const onResize = _debounce(() => this.getViewDimensions(), 100)
    window.addEventListener( 'resize', onResize, { passive: true } )

    this.$once('hook:beforeDestroy', () => {
      window.removeEventListener( 'resize', onResize )
    })
  }
  , mounted(){
    this.getViewDimensions()
  }
  , computed: {
    playlist(){
      let playerRoute = this.$router.options.routes.find( r => r.name === 'player' )
      return playerRoute.children.map( r => ({
        routeName: r.name
        , title: r.meta.title
        , files: r.meta.audio
      }))
    }
    , playIndex(){
      let title = this.$route.meta.title
      return this.playlist.findIndex( p => p.title === title )
    }
    , music(){
      let playerRoute = this.$router.options.routes.find( r => r.name === 'player' )
      return playerRoute.meta.music
    }
  }
  , methods: {
    getViewDimensions(){
      this.viewWidth = this.$refs.main.offsetWidth
      this.viewHeight = this.$refs.main.offsetHeight
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="sass">
.player
  position: absolute
  top: 0
  left: 0
  bottom: 0
  right: 0
  display: flex
  flex-direction: column
  justify-content: flex-end
.panel-bottom
  background: $background
  padding-top: 2px
.main
  position: relative
  flex: 1
  overflow: hidden
</style>
