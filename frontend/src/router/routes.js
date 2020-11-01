import Home from '@/views/Home'
import Video from '@/views/Video'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home
  },
  {
    path: '/video/:name',
    name: 'Video',
    component: Video
  }
]

export default routes
