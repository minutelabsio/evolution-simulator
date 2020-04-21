<template lang="pug">
.about.scrollbars
  .container
    .section
      .content
        .has-text-centered
          .is-inline-block
            img(src="social-thumbnail.jpg", alt="Evolution Simulator by MinuteLabs.io and Primer")
          h1.is-sr-only Evolution Simulator by MinuteLabs.io and Primer

        h2.title.is-size-3 How to use this thing

        p This video will give you a good introduction to the simulator

        responsive-embed(:ratio="16/9", :max-width="560")
          iframe(width="560", height="315", src="https://www.youtube.com/embed/6nMo8T3T0L4", frameborder="0", allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture", allowfullscreen)

        br/
        p.
          The idea for this lab came from Primer. This video explains the basics of the model.

        responsive-embed(:ratio="16/9", :max-width="560")
          iframe(width="560", height="315", src="https://www.youtube.com/embed/0ZGbIKd0XrM", frameborder="0", allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture", allowfullscreen)
        br/
        p.
          Soon more complex models will be added with different rules... so stay tuned!

        h3.title.is-size-3 How to Support my Work

        p.
          I release all content online for free, so user support is the only way I can continue
          making things like this. If my work helps you as much as a cup of coffee does,
          consider supporting current and future work any way you can.

        p Besides sharing the apps with others, here are some other ways to help:

        br/
        .columns
          .column.has-text-centered
            p Donate through <br/>
              a.button.is-info.is-medium(href="https://github.com/sponsors/wellcaffeinated", target="_blank")
                b-icon(icon="heart")
                span Github Sponsors
          .column.has-text-centered
            p.
              Subscribe on Patreon <br/>
              <a href="https://www.patreon.com/bePatron?u=617966" target="_blank">
                <img src="https://c5.patreon.com/external/logo/become_a_patron_button@2x.png" width="217" height="51">
              </a>

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
  padding-top: 5rem
  max-height: 100%
  overflow: auto
  .container
    max-width: 1020px
</style>
