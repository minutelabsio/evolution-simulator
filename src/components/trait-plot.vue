<script>
import chroma from 'chroma-js'
import { Scatter } from 'vue-chartjs'

function clamp(v, min, max){
  return Math.min(Math.max(v, min), max)
}

export default {
  name: 'TraitPlot'
  , extends: Scatter
  , props: {
    'data': {
      type: Array
      // [{ label: "Name", data: [] }]
    }
    , label: {
      type: String
    }
    , color: {
      type: String
      , default: '#FCFDF4'
    }
    , textColor: {
      type: String
      , default: '#FCFDF4'
    }
  }
  , components: {
  }
  , data: () => ({
  })
  , computed: {
    options(){
      let self = this
      // let titleColor = chroma(this.color).desaturate(1).css()
      return {
        responsive: true
        , maintainAspectRatio: false
        , hoverMode: 'index'
        , animation: {
          duration: 0
        }
        , tooltips: {
          intersect: false
          , mode: 'index'
          , position: 'nearest'
          , callbacks: {
            label(tooltipItem, data){
              let idx = tooltipItem.datasetIndex
              if (idx === 2){
                let stddev = self.data[tooltipItem.xLabel - 1].deviation.toFixed(2)
                return `average: ${tooltipItem.yLabel.toFixed(2)} ± ${stddev}`
              } else if (idx === 0 || idx === 4){
                let label = data.datasets[idx].label
                return `${label}: ${tooltipItem.yLabel.toFixed(2)}`
              }
            }
            , title(tooltipItem){
              let gen = tooltipItem[0].xLabel
              return 'Generation: ' + gen
            }
          }
        }
        , scales: {
          xAxes: [{
            display: true
            , ticks: {
              min: 1
              , fontColor: this.textColor
            }
            // , scaleLabel: {
            //   display: true
            //   , labelString: 'Generation'
            //   , fontColor: this.textColor
            //   , fontSize: 14
            // }
          }]
          , yAxes: [{
            type: 'linear'
            , beginAtZero: true
            , display: true
            , ticks: {
              fontColor: this.textColor
            }
            // , stacked: true
            , position: 'left'
            // , scaleLabel: {
            //   display: true
            //   , labelString: this.label
            //   , fontColor: this.textColor
            //   , fontSize: 14
            // }
          }]
        }
        // , title: {
        //   display: true
        //   // , position: 'left'
        //   , text: this.label
        //   , fontSize: 20
        //   , fontColor: titleColor
        // }
        , legend: {
          display: false
        }
        , elements: {
          line: {
            borderWidth: 1
            , tension: 0
          }
        }
      }
    }
    , chartdata(){
      let bgColor = chroma(this.color).alpha(0.1).css()
      let devColor = bgColor
      let maxMinColor = chroma(this.color).desaturate(2).alpha(0.3).css()
      if ( !this.data ){ return {} }
      let options = {
        label: this.label
        , showLine: true
        , pointRadius: 1
        , borderColor: this.color
        , backgroundColor: bgColor
        , fill: false
      }
      if ( Number.isFinite(this.data[0]) ){
        return {
          datasets: [{
            ...options
            , data: this.data.map((y, x) => ({ y, x: x + 1 }))
          }]
        }
      }
      return {
        datasets: [{
          ...options
          , pointRadius: 0
          , data: this.data.map((d, x) => ({ y: d.max, x: x + 1 }))
          // , borderDash: [5, 5]
          , borderColor: maxMinColor
          , label: 'max'
        } , {
          ...options
          , pointRadius: 0
          , data: this.data.map((d, x) => ({ y: clamp(d.mean + d.deviation, d.min, d.max), x: x + 1 }))
          , borderColor: devColor
          , backgroundColor: bgColor
          , fill: 2
          , label: 'stddev (+)'
        }, {
          ...options
          , data: this.data.map((d, x) => ({ y: d.mean, x: x + 1 }))
          , label: 'average'
        }, {
          ...options
          , pointRadius: 0
          , data: this.data.map((d, x) => ({ y: clamp(d.mean - d.deviation, d.min, d.max), x: x + 1 }))
          , borderColor: devColor
          , backgroundColor: bgColor
          , fill: 2
          , label: 'stddev (-)'
        }, {
          ...options
          , pointRadius: 0
          , data: this.data.map((d, x) => ({ y: d.min, x: x + 1 }))
          // , borderDash: [5, 5]
          , borderColor: maxMinColor
          , label: 'min'
        }]
      }
    }
  }
  , watch: {
    chartdata(){
      this.render()
    }
    , options(){
      this.render()
    }
  }
  , mounted() {
    this.render()

    const onClick = () => {
      let tooltip = this._data._chart.tooltip
      if (!tooltip._active){ return }
      let gen = tooltip._active[0]._index + 1
      this.$emit('click', gen)
    }
    this.$el.addEventListener('click', onClick)
    this.$on('hook:beforeDestroy', () => {
      this.$el.removeEventListener('click', onClick)
    })
  }
  , methods: {
    render(){
      if (this._inactive){ return }
      this.options.scales.xAxes[0].max = this.chartdata.length
      this.renderChart(this.chartdata, this.options)
    }
  }
}
</script>

<style lang="sass" scoped>

</style>
