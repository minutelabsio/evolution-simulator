<template lang="pug">
.creature-status
  transition(name="boing", mode="out-in")
    .status(v-if="status", :key="status") {{ status }}
</template>

<script>
export default {
  name: 'CreatureStatus'
  , inject: [ 'getStep', 'threeVue' ]
  , props: {
    creature: Object
  }
  , data: () => ({
    status: null
  })
  , created(){
    this.beforeDraw(() => {
      if (!this.creature){ return }
      let step = this.getStep() | 0
      let history = this.creature.status_history
      if ( step >= history.length ){ step = history.length - 1 }
      this.status = history[step]
    })
  }
  , watch: {
  }
  , computed: {
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
  line
  background: $background
  color: $text

.boing-enter-active, .boing-leave-active
  transition: transform .1s .3s ease-in
.boing-enter, .boing-leave-to
  transform: scale(0)
</style>
