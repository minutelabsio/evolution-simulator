<script>
import _get from 'lodash/get'
import { Scatter } from 'vue-chartjs'

export default {
  name: 'OverviewPlot'
  , extends: Scatter
  , props: {
    'data': {
      type: Array
    }
    , label: {
      type: String
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
              let label = data.datasets[idx].label
              return `${label}: ${tooltipItem.yLabel.toFixed(2)}`
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
            id: 'traits'
            , type: 'linear'
            , beginAtZero: true
            , display: true
            , ticks: {
              fontColor: this.textColor
            }
            , stacked: true
            , position: 'left'
            , scaleLabel: {
              display: true
              , labelString: 'Traits'
              , fontColor: this.textColor
              , fontSize: 16
            }
          }, {
            id: 'population'
            , type: 'linear'
            , beginAtZero: true
            , display: true
            , ticks: {
              fontColor: this.textColor
            }
            , position: 'right'
            , scaleLabel: {
              display: true
              , labelString: 'Population'
              , fontColor: this.textColor
              , fontSize: 16
            }
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
      if ( !this.data ){ return {} }
      let options = {
        showLine: true
        , pointRadius: 1
        , fill: false
      }

      let datasets = this.data.map(d => {
        return {
          ...options
          , ...d
          , borderColor: d.color
          , backgroundColor: d.color
          , data: d.data ? d.data.map((d, x) => ({ y: _get(d, 'mean', d), x: x + 1 })) : []
        }
      })

      return { datasets }
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
      this.options.scales.xAxes[0].max = this.chartdata.length
      this.renderChart(this.chartdata, this.options)
    }
  }
}
</script>

<style lang="sass" scoped>

</style>
