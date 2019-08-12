<template lang="pug">
.jasper-tv
  .screen(v-show="wasOn", :class="{ on: screenOn, off: wasOn && !screenOn }")
    img.pose(v-if="!img", :src="poses[pose]")
    img.custom-image(v-if="img", :src="img")
    canvas.fuzz(ref="canvas", v-show="showFuzz")
</template>

<script>
function makeFuzz( canvas ){
  // From https://codepen.io/alenaksu/pen/dGjeMZ

	const context = canvas.getContext("gl") || canvas.getContext("2d")
	const scaleFactor = 1 // Noise size
	const FPS = 50
	const scanSpeed = FPS * 15 // 15 seconds from top to bottom
	const SAMPLE_COUNT = 10
  let samples = []
	let scanSize = 0
  let sampleIndex = 0
  let scanOffsetY = 0

	function init() {
		canvas.width = canvas.offsetWidth / scaleFactor
		canvas.height = canvas.width / (canvas.offsetWidth / canvas.offsetHeight)

    if (!canvas.width || !canvas.height) return
    scanSize = canvas.offsetHeight / scaleFactor / 3

		samples = []
		for (let i = 0; i < SAMPLE_COUNT; i++){
			samples.push(generateRandomSample(context, canvas.width, canvas.height))
    }
	}

	function interpolate(x, x0, y0, x1, y1) {
		return y0 + (y1 - y0) * ((x - x0) / (x1 - x0))
	}

	function generateRandomSample(context, w, h) {
		var intensity = []
		var factor = h / 50
		var trans = 1 - Math.random() * 0.05

		var intensityCurve = []
		for (let i = 0; i < Math.floor(h / factor) + factor; i++){
			intensityCurve.push(Math.floor(Math.random() * 15))
    }

		for (let i = 0; i < h; i++) {
			var value = interpolate(
				i / factor,
				Math.floor(i / factor),
				intensityCurve[Math.floor(i / factor)],
				Math.floor(i / factor) + 1,
				intensityCurve[Math.floor(i / factor) + 1]
			)
			intensity.push(value)
		}

		var imageData = context.createImageData(w, h)
		for (let i = 0; i < w * h; i++) {
			var k = i * 4
			var color = Math.floor(36 * Math.random())
			// Optional: add an intensity curve to try to simulate scan lines
			color += intensity[Math.floor(i / w)]
			imageData.data[k] = imageData.data[k + 1] = imageData.data[k + 2] = color
			imageData.data[k + 3] = Math.round(255 * trans)
		}
		return imageData
	}

	function render() {
		context.putImageData(samples[Math.floor(sampleIndex)], 0, 0)

		sampleIndex += 20 / FPS // 1/FPS == 1 second
		if (sampleIndex >= samples.length) sampleIndex = 0

		var grd = context.createLinearGradient(
			0,
			scanOffsetY,
			0,
			scanSize + scanOffsetY
		)

		grd.addColorStop(0, "rgba(255,255,255,0)")
		grd.addColorStop(0.1, "rgba(255,255,255,0)")
		grd.addColorStop(0.2, "rgba(255,255,255,0.2)")
		grd.addColorStop(0.3, "rgba(255,255,255,0.0)")
		grd.addColorStop(0.45, "rgba(255,255,255,0.1)")
		grd.addColorStop(0.5, "rgba(255,255,255,1.0)")
		grd.addColorStop(0.55, "rgba(255,255,255,0.55)")
		grd.addColorStop(0.6, "rgba(255,255,255,0.25)")
		//grd.addColorStop(0.8, 'rgba(255,255,255,0.15)')
		grd.addColorStop(1, "rgba(255,255,255,0)")

		context.fillStyle = grd
		// context.fillRect(0, scanOffsetY, canvas.width, scanSize + scanOffsetY)
		context.globalCompositeOperation = "lighter"

		scanOffsetY += canvas.height / scanSpeed
		if (scanOffsetY > canvas.height) {
      scanOffsetY = -(scanSize / 2)
    }
	}

	return {
    init
    , render
  }
}

const poses = {
  'exasperated': require('@/assets/poses/Jasper - Exasperated.png')
  , 'facepalm': require('@/assets/poses/Jasper - Facepalm.png')
  , 'greeting': require('@/assets/poses/Jasper - Greeting.png')
  , 'head-scratch': require('@/assets/poses/Jasper - Head scratch.png')
  , 'magic': require('@/assets/poses/Jasper - Magic.png')
  , 'idea': require('@/assets/poses/Jasper - Idea.png')
  , 'open-arms': require('@/assets/poses/Jasper - Open Arms.png')
  , 'point-nw': require('@/assets/poses/Jasper - Pointing NW.png')
  , 'shrug': require('@/assets/poses/Jasper - Shrug.png')
  , 'talk2': require('@/assets/poses/Jasper - Talk 2.png')
  , 'thinking': require('@/assets/poses/Jasper - Thinking.png')
  , 'thumbs-up': require('@/assets/poses/Jasper - Thumbs up.png')
}
export default {
  name: 'JasperTV'
  , props: [
    'pose'
    , 'screenOn'
    , 'img'
  ]
  , components: {
  }
  , data: () => ({
    poses
    , showFuzz: false
    , wasOn: false
  })
  , mounted(){
    const fuzz = this.fuzz = makeFuzz( this.$refs.canvas )

    let stop = false
    const draw = () => {
      if ( stop ) return
      window.requestAnimationFrame(draw)
      if ( this.showFuzz ){
        fuzz.render()
      }
    }

    fuzz.init()
    draw()
    this.$on('hook:beforeDestroy', () => {
      stop = true
    })
  }
  , watch: {
    screenOn: {
      handler( on ){
        clearTimeout(this.timeout)
        if ( on ){
          this.wasOn = true
          this.showFuzz = true
          this.$nextTick(() => { this.fuzz.init() })
          this.timeout = setTimeout(() => {
            this.showFuzz = false
          }, 1000)
        } else {
          this.showFuzz = false
        }
      }
      , immediate: true
    }
  }
  , methods: {
  }
}
</script>

<style scoped lang="sass">
$ease-out-quint: cubic-bezier(0.230, 1.000, 0.320, 1.000)
$ease-in-quint: cubic-bezier(0.755, 0.050, 0.855, 0.060)
@keyframes turn-on
  0%
    opacity: 0.4
  100%
    opacity: 1

@keyframes turn-off
  0%
    transform: scale(1,1.3) translate3d(0,0,0)
    filter: brightness(1)
    opacity: 1

  60%
    transform: scale(1.3,0.001) translate3d(0,0,0)
    filter: brightness(10)

  100%
    animation-timing-function: $ease-in-quint
    transform: scale(0.000,0.0001) translate3d(0,0,0)
    filter: brightness(50)

.jasper-tv
  position: relative
  background: black
  border: 2px solid $grey-dark
  border-radius: 5px
  overflow: hidden

  .screen
    position: relative
    width: 100%
    height: 100%
    background: $white-ter
    box-shadow: inset -1px -1px 8px rgba(0, 0, 0, 0.9)
    overflow: hidden

  .screen.off
    animation: turn-off 0.55s $ease-out-quint
    animation-fill-mode: forwards

  .screen.on
    animation: turn-on 1s linear
    animation-fill-mode: forwards

  .pose
    transform: scale(1.2) translate(0, 10%)

  .custom-image
    display: block
    max-height: 100%
    max-width: 100%
    margin: auto

  .fuzz
    position: absolute
    top: 0
    left: 0
    width: 100%
    height: 100%
</style>
