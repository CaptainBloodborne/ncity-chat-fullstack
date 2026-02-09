import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import router from './router'
import { useAuth } from './composables/useAuth'

const app = createApp(App)

const { fetchUser } = useAuth()

fetchUser().then(() => {

  app.use(router)
  app.mount('#app')
})
