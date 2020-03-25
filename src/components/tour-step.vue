<template lang="pug">
.tour-step(v-if="step === stepNumber")
  .content
    slot

  .controls
    slot(name="controls", :skip="skip", :next="next", :back="back")
      .level
        .level-left
          b-field(v-if="!isLast")
            p.control
              b-button.is-info(@click="skip") skip
        .level-right
          b-field(position="is-right")
            p.control(v-if="step !== 1")
              b-button.btn-dark(@click="back") back
            p.control
              b-button.is-primary(@click="next") {{ buttonText }}
</template>

<script>

export default {
  name: 'TourStep'
  , props: {
    buttonText: {
      default: 'tell me more'
    }
    , step: Number
    , isLast: Boolean
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
    skip(){
      this.$router.push({ ...this.$route, query: { intro: '' } })
    }
    , back(){
      this.$router.push({ ...this.$route, query: { intro: this.stepNumber - 1 } })
    }
    , next(){
      if (this.isLast){
        return this.skip()
      }

      this.$router.push({ ...this.$route, query: { intro: this.stepNumber + 1 } })
    }
  }
}
</script>

<style lang="sass" scoped>
.tour-step
  max-width: 480px
  pointer-events: all
  background: transparentize($background, 0.03)
  border-radius: 4px
  padding: 1.61rem
  box-shadow: 0px 0px 4px transparentize($primary, 0.8)

.content
  line-height: 1.2
.controls .button
  font-size: 0.9rem
</style>
