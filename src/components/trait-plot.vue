<script>
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
    options: {
      responsive: true
			, hoverMode: 'index'
			, stacked: false
      , scales: {
        yAxes: [{
          type: 'linear' // only linear but allow scale type registration. This allows extensions to exist solely for log scale for instance
          , display: true
          , position: 'left'
        }]
      }
      , legend: {
        display: false
      }
      , elements: {
        line: {
          borderWidth: 1
        }
      }
    }
  })
  , computed: {
    chartdata(){
      return {
        datasets: [{
          label: this.label
          , showLine: true
          , data: this.data.map((y, x) => ({ x, y }))
          , borderColor: this.color
          , backgroundColor: this.color
          , fill: false
        }]
      }
    }
  }
  , watch: {
    chartdata(){
      this.render()
    }
  }
  , mounted() {
    this.render()
  }
  , methods: {
    render(){
      this.renderChart(this.chartdata, this.options)
    }
  }
}
</script>

<style lang="sass" scoped>

</style>
