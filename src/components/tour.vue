<template lang="pug">
.tour-steps
  TourStep(button-text="get started", :step="1")
    .has-text-centered
      p.is-size-5 Welcome to the
      h5.title.is-size-3.has-text-primary Evolution Simulator
      h6.title.is-size-6 By MinuteLabs<br/>(concept by Primer Learning)
      br
    p.
      Before we dive into the deep end, let's take a little look around...
    template(#controls="props")
      .level
        .level-left
          b-field
            b-button.is-text.skip(@click="props.skip") skip
        .level-right
          b-field
            b-button.is-primary(@click="props.next") get started

  TourStep(:step="2")
    TypewriterTextTransition
      p This is blob world. Life is simple here.
  TourStep(:step="3")
    TypewriterTextTransition
      p If you click/tap on a blob, you can follow it and see what it's thinking.
  TourStep(:step="4")
    TypewriterTextTransition
      p.
        Blobs use up energy searching for food and smaller blobs to eat. They need one
        piece of food to survive to the next generation.
  TourStep(:step="5")
    TypewriterTextTransition
      p.
        Although they'll settle for one food and head home if they're low on energy,
        with two food they can reproduce.
  TourStep(button-text="let's watch them evolve!", is-last, :step="6")
    TypewriterTextTransition
      p.
        In the next generation, when they reproduce, their offspring will be slightly different.
        Their traits are mutated in a random way that can be controlled in the settings.
      p (Find more details in the about page)
</template>

<script>
import TourStep from '@/components/tour-step'
import TypewriterTextTransition from '@/components/typewriter-text.transition'

export default {
  name: 'TourSteps'
  , props: {
  }
  , components: {
    TourStep
    , TypewriterTextTransition
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
