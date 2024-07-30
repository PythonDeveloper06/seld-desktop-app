<script setup>
import { useUserData } from '~/store/authuser';

const router = useRouter()
const userData = useUserData()

const name = ref('');
const password = ref('');
const state = ref('');

const error = ref('');

async function getTokenOnRust() {
  userData.login = name.value
  userData.password = password.value
  
  state.value = await userData.getUserToken()

  if (state.value != "400") {
    error.value = ""
    router.push('/devices')
  } else {
    error.value = "Неправильный логин или пароль"
  }

}
</script>

<template>
  <div class="flex flex-col justify-center items-center px-6 py-20 lg:px-8">
    <div class="sm:mx-auto sm:w-full sm:max-w-sm">
      <img class="mx-auto h-10 w-auto" src="../assets/media/locked.png" alt="SELD" />
      <h2 class="mt-5 text-center text-2xl font-bold leading-9 tracking-tight text-gray-900">Sign in to your account</h2>
    </div>

    <div class="mt-5 sm:mx-auto sm:w-full sm:max-w-sm">
      <form class="space-y-6" action="#" method="post" @submit.prevent="getTokenOnRust()">
        <div>
          <label for="email" class="block text-sm font-medium leading-6 text-gray-900">Login</label>
          <div class="mt-2">
            <input type="text" id="email" name="email" v-model="name" required autocomplete="email" class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
          </div>
        </div>

        <div>
          <label for="password" class="block text-sm font-medium leading-6 text-gray-900">Password</label>
          <div class="mt-2">
            <input id="password" name="password" v-model="password" type="password" required autocomplete="current-password" class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
          </div>
        </div>

        <div>
          <button type="submit" class="flex w-full justify-center items-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">Sign in</button>
        </div>
      </form>
    </div>
    <div class="mt-3">
      {{ error }}
    </div>
  </div>
</template>