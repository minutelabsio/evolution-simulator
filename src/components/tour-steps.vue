<template lang="pug">
.tour-steps
  TourStep(button-text="get started", :step="1")
    p.is-size-5 Welcome to the Evolution Simulator
    template(#controls="props")
      .level
        .level-left
          b-field
            b-button.is-text.skip(@click="props.skip") skip
        .level-right
          b-field
            b-button.is-primary(@click="props.next") get started

  TourStep(:step="2")
    p This is blob world. Life is simple here.
  TourStep(:step="3")
    p If you click/tap on a blob, you can follow it and see what it's thinking.
  TourStep(:step="4")
    p.
      Blobs use up energy searching for food and smaller blobs to eat. They need one
      piece of food to survive to the next generation.
  TourStep(:step="5")
    p.
      Although they'll settle for one food and head home if they're low on energy,
      with two food they can reproduce.
  TourStep(button-text="let's watch them evolve!", is-last, :step="6")
    p.
      In the next generation, when they reproduce, their offspring will be slightly different.
      Their traits are mutated in a random way that can be controlled in the settings.
    p (Find more details in the about page)
</template>

<script>
import TourStep from '@/components/tour-step'

export default {
  name: 'TourSteps'
  , props: {
  }
  , components: {
    TourStep
  }
  , data: () => ({
  })
  , mounted(){
  }
  , watch: {
  }
  , computed: {
    stepNumber(){
      return this.$route.query.intro | 0
    }
  }
  , methods: {
  }
}
</script>

<style lang="sass" scoped>
.button.skip
  color: darken($grey-light, 10)
</style>
