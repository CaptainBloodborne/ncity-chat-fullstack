<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { fetchApi } from '../utils/http'
import { useAuth } from '../composables/useAuth'

const router = useRouter()
const email = ref('')
const password = ref('')

async function handleLogin() {
  const loginRes = await fetchApi('/api/login', {
    method: 'POST',
    body: JSON.stringify({
      email: email.value,
      password: password.value
    })
  })

  if (loginRes.ok) {

    const { fetchUser } = useAuth()

    if (!(await fetchUser())) {

      alert("Failed to fetch user data")
    }

    router.push('/')
  } else {

    alert("Invalid credentials")
  }
}
</script>

<template>
  <div class="max-w-sm mx-auto mt-20 p-6 bg-white shadow-lg rounded-lg">
    <h1 class="text-2xl font-bold mb-4">Авторизация</h1>
    <form @submit.prevent="handleLogin" class="space-y-4">
      <input v-model="email" placeholder="Email" class="w-full border p-2 rounded" />
      <input v-model="password" type="password" placeholder="Password" class="w-full border p-2 rounded" />
      <button class="w-full bg-blue-600 text-white p-2 rounded hover:bg-blue-700">Войти</button>
    </form>

    <div class="text-center mt-4">
      <p class="text-sm text-gray-600">
        <RouterLink to="/register" class="font-medium text-indigo-600 hover:text-indigo-500">
          Создать новый аккаунт
        </RouterLink>
      </p>
    </div>
  </div>
</template>