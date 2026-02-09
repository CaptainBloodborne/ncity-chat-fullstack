<script setup>
import { ref } from 'vue'
import { useRouter, RouterLink } from 'vue-router'
import { fetchApi } from '@/utils/http' // Use your existing helper

const router = useRouter()

// STATE
const form = ref({
  name: '',
  email: '',
  password: '',
  confirmPassword: ''
})

const isSubmitting = ref(false)
const errorMessage = ref('')


async function handleRegister() {
  errorMessage.value = ''

  if (form.value.password !== form.value.confirmPassword) {
    errorMessage.value = "Passwords do not match!"
    return
  }

  isSubmitting.value = true

  try {

    const response = await fetchApi('/api/user', {
      method: 'POST',
      body: JSON.stringify({
        name: form.value.name,
        email: form.value.email,
        password: form.value.password
      })
    })

    if (!response.ok) {

      const data = await response.json().catch(() => ({}))
      throw new Error(data.message || 'Registration failed')
    }


    alert('Registration successful! Please login.')
    router.push({ name: 'login' })

  } catch (error) {
    errorMessage.value = error.message
  } finally {
    isSubmitting.value = false
  }
}
</script>

<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
    <div class="max-w-md w-full space-y-8 bg-white p-8 rounded-lg shadow-md">
      
      <div>
        <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900">
           Создайте новый аккаунт
        </h2>
        <p class="mt-2 text-center text-sm text-gray-600">
          или
          <RouterLink to="/login" class="font-medium text-indigo-600 hover:text-indigo-500">
            войдите в свой аккаунт
          </RouterLink>
        </p>
      </div>

      <form class="mt-8 space-y-6" @submit.prevent="handleRegister">
        
        <div v-if="errorMessage" class="bg-red-50 text-red-600 p-3 rounded text-sm text-center">
          {{ errorMessage }}
        </div>

        <div class="rounded-md shadow-sm -space-y-px">
          <div>
            <label for="name" class="sr-only">Имя</label>
            <input v-model="form.name" id="name" name="name" type="text" required 
              class="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-t-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm" 
              placeholder="имя" />
          </div>

          <div>
            <label for="email-address" class="sr-only">Email</label>
            <input v-model="form.email" id="email-address" name="email" type="email" required 
              class="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm" 
              placeholder="email" />
          </div>

          <div>
            <label for="password" class="sr-only">Пароль</label>
            <input v-model="form.password" id="password" name="password" type="password" required 
              class="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm" 
              placeholder="пароль" />
          </div>

          <div>
            <label for="confirm-password" class="sr-only">Подтвердите пароль</label>
            <input v-model="form.confirmPassword" id="confirm-password" name="confirm-password" type="password" required 
              class="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-b-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm" 
              placeholder="подтвердите пароль" />
          </div>
        </div>

        <div>
          <button type="submit" :disabled="isSubmitting"
            class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:bg-gray-400">
            <span v-if="isSubmitting">Создание аккаунта...</span>
            <span v-else>Войти</span>
          </button>
        </div>
      </form>
    </div>
  </div>
</template>