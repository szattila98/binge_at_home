import Vue from 'vue'
import App from './App.vue'
import router from './router/router.js'
import store from './store/store.js'
import vueMoment from 'vue-moment'
import vueFilterPrettyBytes from 'vue-filter-pretty-bytes'
import BootstrapVue from 'bootstrap-vue'

import 'video.js/dist/video-js.css'

Vue.config.productionTip = false

new Vue({
  router,
  store,
  render: h => h(App)
}).$mount('#app')

Vue.use(vueMoment)
Vue.use(vueFilterPrettyBytes)
Vue.use(BootstrapVue)
