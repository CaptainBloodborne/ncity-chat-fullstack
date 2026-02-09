import { ref, computed } from "vue";
import { fetchApi } from "../utils/http";

const user = ref(null)
const isAuthenticated = ref(false)

export function useAuth() {
    function setUser(userData) {
        user.value = userData
    }

    async function fetchUser() {

    try {
      const res = await fetchApi('/api/user')

      if (res.ok) {
        user.value = await res.json()
        isAuthenticated.value = true
        
        return true
      }
    } catch (e) {
      console.error(e)
    }

    user.value = null
    isAuthenticated.value = false

    return false
  }

  async function logout() {
    try {

      await fetchApi('/api/logout', { method: 'POST' })
    } catch (err) {
      console.error("Logout failed", err)
    } finally {

      user.value = null
      isAuthenticated.value = false
    }

  }

  const isAdmin = computed(() => {
        return user.value?.role === 'admin'
    })

    return {
        user,
        isAuthenticated,
        isAdmin,
        setUser,
        fetchUser,
        logout
    }
}