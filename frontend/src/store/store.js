import Vue from 'vue'
import Vuex from 'vuex'
import api from '@/api/video-api.js'

Vue.use(Vuex)

export default new Vuex.Store({
  state: {
    videos: []
  },
  mutations: {
    setVideos (state, videos) {
      state.videos = videos
    }
  },
  actions: {
    getVideos ({ commit }) {
      api.getAll().then((response) => {
        commit('setVideos', response.data)
      }).catch((error) => {
        console.error('Get all courses unsuccessful', error)
      })
    }
  },
  modules: {}
})
