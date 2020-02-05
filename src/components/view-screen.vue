<template lang="pug">
.viewer
  SimulationConfig.cfg(v-if="showConfig", @close="showConfig = false")
  .top-controls(v-show="!showConfig")
    router-link.config(:to="{ params: { ...$route.params, cfg: 'c' } }")
      b-icon(icon="cogs", size="is-large")
  .screen
    GenerationViewer(:generation-index="genIndex", :step-time="stepTime")
  .controls(v-show="!showConfig")
    .inner
      b-field.extras(grouped, position="is-centered", :class="{ active: playthrough }")
        b-field
          b-icon.clickable(title="play through", icon="redo", @click.native="playthrough = !playthrough")
      b-field(grouped, position="is-centered")
        b-field
          b-icon.clickable(icon="skip-previous", size="is-large", @click.native="prevGeneration()")
        b-field
          b-icon.clickable(:icon="paused ? 'play' : 'pause'", size="is-large", @click.native="togglePlay()")
        b-field
          b-icon.clickable(icon="skip-next", size="is-large", @click.native="nextGeneration()")
    AudioScrubber(:progress="progress", @scrub="onScrub")
</template>

<script>
import { mapGetters } from 'vuex'
import Copilot from 'copilot'
import AudioScrubber from '@/components/audio-scrubber'
import GenerationViewer from '@/components/generation-viewer'
import SimulationConfig from '@/components/simulation-config'

export default {
  name: 'ViewScreen'
  , props: {
    showConfig: Boolean
  }
  , provide(){
    const self = this
    return {
      getTime(){ return self.time }
      , getStep(){ return self.time / self.stepTime }
    }
  }
  , data: () => ({
    paused: true
    , playthrough: true
    , stepTime: 100 // ms
    , time: 0
    , progress: 0
  })
  , components: {
    AudioScrubber
    , GenerationViewer
    , SimulationConfig
  }
  , created(){
    this.player = Copilot.Player({ totalTime: 1 })
  }
  , deactivated(){
    this.player.togglePause(true)
  }
  , beforeDestroy(){
    this.player.togglePause(true)
    this.player.off(true)
  }
  , mounted(){
    this.player.on('update', () => {
      this.time = this.player.time
      this.progress = this.time/this.totalTime * 100
    })
    this.player.on('togglePause', () => {
      if ( this.player.paused !== this.paused ){
        this.paused = this.player.paused
      }
    })
    this.player.on('end', () => {
      if ( this.playthrough ){
        this.nextGeneration()
      }
    })
  }
  , computed: {
    totalTime(){
      if ( !this.generation ){ return 1 }
      return this.stepTime * this.generation.steps
    }
    , generation(){
      return this.getCurrentGeneration()
    }
    , genIndex: {
      get(){
        return this.generationIndex
      }
      , set(v){
        this.loadGeneration(v)
      }
    }
    , ...mapGetters('simulation', {
      getCurrentGeneration: 'getCurrentGeneration'
      , generationIndex: 'currentGenerationIndex'
      , stats: 'statistics'
    })
  }
  , watch: {
    totalTime(){
      this.player.totalTime = this.totalTime
    }
    , paused(){
      this.player.togglePause(this.paused)
    }
    , generation(){
      this.paused = true
      if ( this.player ){
        this.player.seek(0)
      }
      setTimeout(() => {
        this.paused = false
      }, 500)
    }
  }
  , methods: {
    togglePlay(){
      this.paused = !this.paused
    }
    , updateTime(time){
      if ( time !== this.player.time ){
        this.player.seek(time)
      }
    }
    , onScrub(progress){
      this.updateTime(progress * this.totalTime / 100)
    }
    , loadGeneration(v){
      this.$store.dispatch('simulation/loadGeneration', v)
    }
    , prevGeneration(){
      if ( !this.generation ){ return }
      if ( this.genIndex <= 0 ){ return }
      this.genIndex -= 1
    }
    , nextGeneration(){
      if ( !this.generation ){ return }
      if ( this.genIndex >= this.stats.num_generations - 1 ){ return }
      this.genIndex += 1
    }
  }
}
</script>

<style lang="sass" scoped>
.viewer
  position: relative
  display: flex
  height: 100%
  flex-direction: column
  .screen
    flex-grow: 1
    overflow: hidden
.cfg
  position: absolute
  top: 0
  left: 0
  right: 0
  bottom: 0
  z-index: 1
  background: rgba(0, 0, 0, 0.35)
  backdrop-filter: blur(5px)
.top-controls
  position: absolute
  top: 1em
  right: 1em
  z-index: 1
.controls
  position: absolute
  bottom: 0
  left: 0
  right: 0
  align-items: baseline

  @media(pointer: fine)
    .inner
      opacity: 0.5
      transition: opacity 0.3s ease

    .inner:hover
      opacity: 1

  >>> .field
    margin-bottom: 0.5em

  .extras
  margin-bottom: 0

.top-controls,
.controls,
>>> .cfg
  .icon
    transition: color 0.15s ease
    color: darken($grey-light, 25)
    text-shadow: 0 0 1px $black-ter
    &:hover
      color: $grey-lighter
  .active .icon
    color: $blue

>>> .scrubber .inner
  background: darken($grey-light, 45)
</style>
