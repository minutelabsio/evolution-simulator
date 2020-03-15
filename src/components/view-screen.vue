<template lang="pug">
.viewer
  .top-controls(v-show="!showConfig")
    FloatingPanel(size="is-medium", :close-on-click="false")
      template(#activator)
        b-icon.icon-btn(icon="feature-search", size="is-medium")
      .item
        b-tooltip(label="Toggle sight range indicators", position="is-left")
          b-icon.icon-btn(icon="eye", size="is-medium", :class="{ active: showSightIndicator }", @click.native.stop="showSightIndicator = !showSightIndicator")
      .item
        b-tooltip(label="Toggle speed indicators", position="is-left")
          b-icon.icon-btn(icon="run-fast", size="is-medium", :class="{ active: showSpeedIndicator }", @click.native.stop="showSpeedIndicator = !showSpeedIndicator")
      .item
        b-tooltip(label="Toggle energy indicators", position="is-left")
          b-icon.icon-btn(icon="battery-charging", size="is-medium", :class="{ active: showEnergyIndicator }", @click.native.stop="showEnergyIndicator = !showEnergyIndicator")

    b-icon.icon-btn(:icon="followCreature ? 'video-account': 'video-image'", :class="{ active: followCreature }", size="is-medium", @click.native.stop="followCreature = !followCreature")

  .screen
    b-loading.loading-cover(:is-full-page="false", :active="isLoading")
    GenerationViewer(
      :generation-index="genIndex"
      , :step-time="stepTime"
      , :sight-indicators="showSightIndicator"
      , :speed-indicators="showSpeedIndicator"
      , :energy-indicators="showEnergyIndicator"
      , :followCreatureIndex="followCreature ? followCreatureIndex : undefined"
      , @tap-creature="followCreatureIndex = $event.index; followCreature = true"
    )
  .controls(v-show="!showConfig")
    .inner
      b-field.extras(grouped, position="is-centered", :class="{ active: playthrough }")
        b-field
          b-icon.icon-btn(title="play through", icon="redo", @click.native="playthrough = !playthrough")
      b-field(grouped, position="is-centered")
        b-field
          b-icon.icon-btn(icon="chevron-left", size="is-large", @click.native="prevGeneration()")
        b-field
          b-icon.icon-btn(:icon="paused ? 'play' : 'pause'", size="is-large", @click.native="togglePlay()")
        b-field
          b-icon.icon-btn(icon="chevron-right", size="is-large", @click.native="nextGeneration()")

      .right
        FloatingPanel(size="is-medium", direction="up", :close-on-click="false")
          template(#activator)
            b-icon.icon-btn(icon="clock-fast", size="is-medium")
          .item
            vue-slider(
              v-model="playbackSpeed"
              , :min="1"
              , :max="10"
              , :interval="1"
              , :height="200"
              , direction="btt"
              , :contained="true"
            )
    AudioScrubber(:progress="progress", @scrub="onScrub")
</template>

<script>
import { mapGetters } from 'vuex'
import Copilot from 'copilot'
import AudioScrubber from '@/components/audio-scrubber'
import GenerationViewer from '@/components/generation-viewer'
import FloatingPanel from '@/components/floating-panel'

export default {
  name: 'ViewScreen'
  , props: {
    showConfig: Boolean
  }
  , provide(){
    const self = this
    return {
      getTime(){ return self.player.time }
      , getStep(){ return self.player.time / self.stepTime }
    }
  }
  , data: () => ({
    paused: true
    , playthrough: true
    , playbackSpeed: 5
    , progress: 0
    , showSightIndicator: false
    , showSpeedIndicator: false
    , showEnergyIndicator: false
    , followCreature: false
    , followCreatureIndex: 0
  })
  , components: {
    AudioScrubber
    , GenerationViewer
    , FloatingPanel
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
      this.progress = this.player.time / this.totalTime * 100
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
    stepTime(){
      return Math.pow(10, (1 - this.playbackSpeed/5)) * 100
    }
    , totalTime(){
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
      , isLoading: 'isLoading'
    })
  }
  , watch: {
    totalTime(){
      let p = this.player.time / this.player.totalTime
      this.player.totalTime = this.totalTime
      this.player.time = this.totalTime * p
    }
    , paused(){
      this.player.togglePause(this.paused)
    }
    , generation(){
      if (this._inactive) return
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
  flex-direction: column
  .screen
    flex-grow: 1
    overflow: hidden
    display: flex
    align-items: stretch
    > *
      flex: 1

.top-controls
  position: absolute
  top: 1.5rem
  right: 1.5rem
  z-index: 1

  > *
    &:not(:first-child)
      margin-left: 1rem

  @media screen and (max-width: $tablet)
    top: 5rem

.loading-cover
  z-index: 1
.controls
  position: absolute
  bottom: 0
  left: 0
  right: 0
  align-items: baseline

  .right
    position: absolute
    right: 1.5rem
    bottom: 1.5rem

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

>>> .scrubber .inner
  background: darken($grey-light, 45)
</style>
