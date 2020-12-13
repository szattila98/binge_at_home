import Home from '@/views/Home'
import Video from '@/views/Video'
import NotFound from '@/views/NotFound'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home
  },
  {
    path: '/video/:name/type/:ext',
    name: 'Video',
    component: Video
  },
  {
    path: '*',
    name: 'NotFound',
    component: NotFound
  }
]

export default routes
