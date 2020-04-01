<template lang="pug">
.about.scrollbars
  .container
    .section
      .content
        h2.title.is-size-3 How to use this thing

        p TODO ML VIDEO TOUR

        p.
          The idea for this lab came from Primer. This video explains the basics of the model.

        responsive-embed(:ratio="16/9", :max-width="560")
          iframe(width="560", height="315", src="https://www.youtube.com/embed/0ZGbIKd0XrM", frameborder="0", allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture", allowfullscreen)
        br/
        p.
          Soon more complex models will be added with different rules... so stay tuned!



        h2.title.is-size-3 Credits
        p.
          The original idea behind this came from
          <a href="https://www.youtube.com/watch?v=0ZGbIKd0XrM" target="_blank">Primer's wonderful video.</a>
        p.
          All of the code was written by yours truly (Jasper), but I couldn't
          have made it look so good without Sarah Berman's artistic direction and
          drawings.
        p.
          And as always, this was made possible by ths support of the
          <a href="https://minuteearth.com" target="_blank">MinuteEarth team</a>.

        h3.title.is-size-4 Thank you, Patreon Supporters! (so much)
        p.
          Geeks like you are the reason I build things.

        <a href="https://www.patreon.com/bePatron?u=617966" target="_blank">
          <img src="https://c5.patreon.com/external/logo/become_a_patron_button@2x.png" width="217" height="51">
        </a>

        h3.title.is-size-3 Source Code
        p.
          As always, freely available on
          <a href="https://github.com/minutelabsio/evolution-simulator" target="_blank">github</a>.

        h3.title.is-size-3 Bug Reports
        p.
          If you find a bug, you can post it on
          <a href="https://www.patreon.com/minutelabsio" target="_blank">patreon</a>,
          <a href="mailto:bugs@minutelabs.io">email us</a>,
          or post it on <a href="https://github.com/minutelabsio/evolution-simulator" target="_blank">github</a>.

</template>

<script>
import ResponsiveEmbed from '@/components/responsive-embed'
import FlowerChart from '@/components/flower-chart'

export default {
  name: 'About'
  , components: {
    ResponsiveEmbed
    , FlowerChart
  }
}
</script>

<style lang="sass" scoped>
.about
  padding-top: 4rem
  max-height: 100%
  overflow: auto
  .container
    max-width: 1020px
</style>
