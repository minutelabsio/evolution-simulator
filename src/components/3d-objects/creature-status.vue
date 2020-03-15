<template lang="pug">
.creature-status(@click="open = !open")
  transition(name="boing", mode="out-in", appear, appear-class="boing-appear", appear-to-class="boing-appear-to")
    .bubble(v-if="open", key="bubble")
      .bg
        img(src="@/assets/status-icons/thought-2.svg", width="160")
      transition(name="boing", mode="out-in")
        .status-icon(v-if="status", :key="status")
          img.status(:src="iconUrl", width="160")
    .collapsed(v-else, key="collapsed")
      img(src="@/assets/status-icons/thought-1.svg", width="160")
</template>

<script>

const ICONS = {
  'wandering': require('@/assets/status-icons/search.svg')
  , 'low energy': require('@/assets/status-icons/low-energy.svg')
  , 'see food': require('@/assets/status-icons/food.svg')
  , 'see prey': require('@/assets/status-icons/chase.svg')
  , 'running away': require('@/assets/status-icons/run.svg')
  , 'satisfied': require('@/assets/status-icons/satisfied.svg')
  , 'reproduce': require('@/assets/status-icons/reproduce.svg')
}

export default {
  name: 'CreatureStatus'
  , inject: [ 'getStep', 'threeVue' ]
  , props: {
    creature: Object
  }
  , data: () => ({
    status: null
    , open: true
  })
  , created(){
    this.beforeDraw(() => {
      if (!this.creature){ return }
      let lastStep = false
      let step = this.getStep() | 0
      let history = this.creature.status_history
      if ( step >= history.length ){
        step = history.length - 1
        lastStep = true
      }

      let status = history[step]

      if ( lastStep && status == "satisfied" ){
        status = "reproduce"
      }

      this.status = status
    })
  }
  , watch: {
  }
  , computed: {
    iconUrl(){
      return ICONS[this.status]
    }
  }
  , methods: {
    beforeDraw( fn ){
      this.threeVue.$on( 'beforeDraw', fn )

      this.$on('hook:beforeDestroy', () => {
        this.threeVue.$off( 'beforeDraw', fn )
      })
    }
  }
}
</script>

<style lang="sass" scoped>
.creature-status
  position: relative
  pointer-events: all
  color: $text

  img
    max-width: none

  .bg
    opacity: 0.9

  .status-icon
    position: absolute
    z-index: 1
    top: 0
    left: 0

.boing-enter-active, .boing-leave-active
  transition: transform .1s ease-in
.boing-enter, .boing-leave-to
  transform: scale(0)

.collapsed.boing-enter, .collapsed.boing-leave-to
  transform: translate(0, 60px) scale(0)

.bubble.boing-enter, .bubble.boing-leave-to
  transform: translate(0, 60px) scale(0)

.bubble.boing-appear
  transform: translate(0, 60px) scale(0)
.boing-appear-to
  transition: transform 0.5s 0.5s ease-in
</style>
