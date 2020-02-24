<script>
import chroma from 'chroma-js'
import { Scatter } from 'vue-chartjs'

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
      , default: '#bc80bd'
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
        , scales: {
          xAxes: [{
            display: true
            // , beginAtZero: true
            , ticks: {
              min: 1
              , fontColor: '#aaa'
            }
            // , scaleLabel: {
            //   display: true
            //   , labelString: 'Generation'
            //   , fontColor: '#aaa'
            //   , fontSize: 14
            // }
          }]
          , yAxes: [{
            type: 'linear'
            , beginAtZero: true
            , display: true
            , ticks: {
              fontColor: '#aaa'
            }
            // , stacked: true
            , position: 'left'
            // , scaleLabel: {
            //   display: true
            //   , labelString: this.label
            //   , fontColor: '#aaa'
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
            , line: {
              tension: 0.000001
            }
          }
        }
      }
    }
    , chartdata(){
      let devColor = chroma(this.color).desaturate(2).alpha(0.5).css()
      let bgColor = chroma(this.color).alpha(0.1).css()
      if ( !this.data ){
        return {}
      }
      if ( Number.isFinite(this.data[0]) ){
        return {
          datasets: [{
            label: this.label
            , showLine: true
            , pointRadius: 1
            , data: this.data.map((y, x) => ({ y, x: x + 1 }))
            , borderColor: this.color
            , backgroundColor: bgColor
            , fill: false
          }]
        }
      }
      return {
        datasets: [{
          label: this.label
          , showLine: true
          , pointRadius: 0
          , data: this.data.map((d, x) => ({ y: d[0] - d[1], x: x + 1 }))
          , borderColor: devColor
          , backgroundColor: bgColor
          , fill: 1
        }, {
          label: this.label
          , showLine: true
          , pointRadius: 1
          , data: this.data.map((d, x) => ({ y: d[0], x: x + 1 }))
          , borderColor: this.color
          , backgroundColor: bgColor
          , fill: false
        }, {
          label: this.label
          , showLine: true
          , pointRadius: 0
          , data: this.data.map((d, x) => ({ y: d[0] + d[1], x: x + 1 }))
          , borderColor: devColor
          , backgroundColor: bgColor
          , fill: 1
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
