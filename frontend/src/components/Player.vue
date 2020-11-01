<template lang="pug">
  video(ref="videoPlayer" class="video-js vjs-default-skin vjs-big-play-centered")
</template>

<script>
import videojs from 'video.js'
import hotkeys from 'videojs-hotkeys'

export default {
  name: 'Player',
  data () {
    return {
      player: null,
      options: {
        autoplay: false,
        controls: true,
        preload: 'auto',
        playbackRates: [0.5, 1.0, 2.0],
        sources: [{
          src: 'http://localhost:8080/api/video/' + this.$route.params.name, // TODO no prefix
          type: `video/${this.$route.params.name.substr(this.$route.params.name.length - 3)}`
        }],
        fluid: true,
        // poster: "/static/images/author.jpg", TODO poster on backend
        plugins: { // TODO moar plugins
          hotkeys: {
            customClass: hotkeys
          }
        }
      }
    }
  },
  mounted () {
    this.player = videojs(this.$refs.videoPlayer, this.options, function onPlayerReady () {
      console.log('onPlayerReady', this)
    })
  },
  beforeDestroy () {
    if (this.player) {
      this.player.dispose()
    }
  }
}
</script>
