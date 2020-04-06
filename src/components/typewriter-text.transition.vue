<template lang="pug">
transition(
  appear
  , :css="false"
  , @appear="transitionText"
  , @appear-cancelled="cancelTransition"
)
  div(ref="content")
    slot
</template>

<script>
import _map from 'lodash/map'
import _sum from 'lodash/sum'
import Copilot from 'copilot'

export default {
  name: 'TypewriterTextTransition'
  , props: {
    // letters per second
    lps: {
      type: Number
      , default: 50
    }
  }
  , data: () => ({
  })
  , created(){
    this.player = Copilot.Player({ totalTime: 1 })
  }
  , beforeDestroy(){
    this.cancelTransition()
  }
  , watch: {
  }
  , computed: {
  }
  , methods: {
    transitionText(el, done){
      let ps = el.getElementsByTagName('p')
      let nps = ps.length
      let psText = _map(ps, p => p.innerText)
      let psLetters = psText.map(t => t.length)
      let letterCount = _sum(psLetters)

      Array.prototype.forEach.call(ps, (p) => {
        let { width, height } = p.getBoundingClientRect()
        p.style.display = 'inline-block'
        p.style.width = width + 'px'
        p.style.height = height + 'px'
        p.innerText = ''
      })

      this.player.totalTime = letterCount * 1000 / this.lps
      this.player.on('update', () => {
        let progress = this.player.time / this.player.totalTime
        let letterProgress = Math.round(progress * letterCount)
        let i = 0
        let l = letterProgress - psLetters[0]
        while ( l > 0 ){
          ps[i].innerText = psText[i]
          i++
          l -= psLetters[i]
        }

        let p = ps[i]
        let lettersVisible = psLetters[i] + l
        let visibleText = psText[i].substr(0, lettersVisible)
        let pad = psText[i].substr(lettersVisible).replace(/\s+.*/, '')

        p.innerHTML = visibleText + `<span class="hide">${pad}</span>`

        for (i += 1; i < nps; i++){
          ps[i].innerHTML = '<span class="hide">x</span>'
        }
      })

      this.player.on('end', () => {
        for (let i = 0; i < nps; i++){
          ps[i].innerHTML = psText[i]
        }
        done()
        this.player.off(true)
      })

      this.player.seek(0)
      this.player.togglePause(false)
    }
    , cancelTransition(){
      this.player.togglePause(true)
      this.player.off(true)
    }
  }
}
</script>

<style lang="sass" scoped>
>>> .hide
  color: transparent
</style>
