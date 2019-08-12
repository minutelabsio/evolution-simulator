<template lang="pug">
.player
  b-loading(:is-full-page="false", :active="loading")
  slot(:loading="loading")
</template>

<script>
// import PubSub from '@/lib/pubsub'
import Copilot from 'copilot'
import { Howl, Howler } from 'howler'
import NoSleep from 'nosleep.js'
import * as Visibility from '@/lib/visibility'
import _throttle from 'lodash/throttle'

export default {
  name: 'Player'
  , props: {
    playlist: {
      type: Array
      , required: true
    }
    , music: {
      type: Object
    }
    , itemType: {
      type: String
      , default: 'Chapter'
    }
    , playIndex: {
      type: Number
      , default: 0
    }
  }
  , provide(){
    return {
      player: this
    }
  }
  , components: {
  }
  , data: () => ({
    time: 0
    , totalTime: -1
    , paused: true
    , ended: false
    , playlistIndex: 0
    , scrubbing: false

    , volume: 1
    , musicVolume: 1
  })
  , created(){
    Howler.volume( 1 )

    let cleanup = Copilot.Syncher({
      getPlaybackRate: () => this.howl && this.howl.rate()
      , isPlaying: () => this.howl && this.howl.playing()
      , getTime: () => this.howl && this.howl.seek() * 1000
      , onFrame: ( time, now, { isPlaying } ) => {
        this.$emit('frame', now)

        if ( !this.howl || this.scrubbing ){ return }

        if ( isPlaying ){
          this.time = time
        }
      }
    })

    this.$once('hook:beforeDestroy', () => {
      cleanup()
    })

    this.$on('end', () => {
      this.paused = true
      this.ended = true
    })

    this.$on('seek', () => { this.ended = false })
    this.$on('togglePause', () => { this.ended = false })

    const noSleep = new NoSleep()

    this.$watch('paused', isPaused => {
      if ( isPaused ){
        noSleep.disable()
      } else {
        noSleep.enable()
      }
    })

    let wasPlayingMusic = false
    this.$once('play', () => {
      if ( this.musicHowl ){
        wasPlayingMusic = true
        this.musicHowl.play()
      }
    })

    const pauseOnWindowHidden = () => {
      if ( Visibility.isHidden() ){
        this.pause()

        if ( this.musicHowl ){
          this.musicHowl.pause()
        }
      } else {
        if ( this.musicHowl && wasPlayingMusic ){
          this.musicHowl.play()
        }
      }
    }

    Visibility.onChange( pauseOnWindowHidden )
    this.$on('hook:beforeDestroy', () => {
      Visibility.offChange( pauseOnWindowHidden )
    })
  }
  , destroyed(){
    if ( this.howls ){
      this.howls.forEach( h => h.unload() )
    }
    if ( this.musicHowl ){
      this.musicHowl.unload()
    }
  }
  , watch: {
    howl: {
      handler( newHowl, oldHowl ){
        this.totalTime = -1
        this.time = 0

        if ( oldHowl ){
          oldHowl.pause()
          // remove all events
          oldHowl.off()
        }

        newHowl.on('load', this.setTotalTime.bind(this))
        newHowl.on('loaderror', this.announceError.bind(this))
        newHowl.on('play', () => { this.paused = false })
        newHowl.on('pause', () => { this.paused = true })
        newHowl.on('end', () => { this.$emit('end') })
        // fix for dumb bug with seeking
        newHowl.on('fade', () => { this._mlio_volume = null })
        newHowl.load().seek(0)
      }
      , immediate: true
    }
    , howls( newHowls, oldHowls ){
      if (oldHowls){
        oldHowls.forEach( h => h.unload() )
      }
    }
    , volume( val ){
      Howler.volume( val )
    }
    , musicVolume( val ){
      let m = this.musicHowl
      if ( !m ) return

      m.volume( val * this.music.maxVolume )
    }
  }
  , computed: {
    howls(){
      if ( !this.playlist ){ return [] }
      return this.playlist.filter( e => e.files ).map( entry => {
        return new Howl({
          src: entry.files
          , preload: false
        })
      })
    }
    , howl(){
      if ( !this.howls.length ){ return null }
      return this.howls[ this.playIndex ]
    }
    , musicHowl(){
      if ( !this.music ){ return }
      return new Howl({
        src: this.music.audio
        , preload: true
        , loop: true
        , volume: this.music.maxVolume
      })
    }
    , nowPlaying(){
      if ( !this.playlist ){ return { title: '' } }
      return this.playlist[ this.playIndex ]
    }
    , loading(){
      return this.totalTime === -1
    }
  }
  , methods: {
    announceError( err ){
      console.log( err ) // eslint-disable-line no-console
      const exception = err.message || err
      this.$ga.exception(exception)
    }
    , setTotalTime(){
      this.totalTime = this.howl.duration() * 1000
    }
    , togglePlay(){
      if ( !this.howl ){ return }
      if ( !this.paused ){
        this.pause()
      } else {
        this.play()
      }
    }
    , pause(){
      this.howl.pause()
      this.$emit('pause')
      this.$emit('togglePause', true)
    }
    , play(){
      this.howl.play()
      this.$emit('play')
      this.$emit('togglePause', false)
    }
    , previous(){
      this.setTrack( this.playIndex - 1 )
    }
    , next(){
      this.setTrack( this.playIndex + 1 )
    }
    , setTrack( index ){
      if ( index < 0 ){ return }
      if ( index >= this.playlist.length ){ return }

      let name = this.playlist[ index ].routeName

      this.pause()
      this.$router.push({ name })
    }
    , seek( time ){
      this.time = time
      this.seekAudio()
      this.$emit('seek')
    }
    , seekAudio: _throttle(function(){
      let vol = this.howl._mlio_volume || (this.howl._mlio_volume = this.howl.volume())
      this.howl.fade( vol, 0, 10 )
      setTimeout( () => {
        this.howl.seek( this.time / 1000 )
        this.howl.fade( 0, vol, 100 )
      }, 10)
    }, 200)
  }
}
</script>

<style scoped lang="sass">
.loading-overlay
  z-index: 100
</style>
