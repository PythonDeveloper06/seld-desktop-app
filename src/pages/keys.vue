<script setup>
import { useUserData } from '~/store/authuser';
import Header from '../components/Header.vue';
import { invoke } from '@tauri-apps/api/core';

const router = useRouter();

const userData = useUserData();

definePageMeta({
  middleware: 'auth',
})

const keys = ref([]);
const setVar = ref('');

const params = {
    year: 'numeric',
    month: 'numeric',
    day: 'numeric',
    hour: 'numeric',
    minute: 'numeric',
    second: 'numeric',
}

const form = reactive({
  key: generateKey(),
  used: "T",
  time_end: new Date().toLocaleString('ru-RU', params),
  selection: "-",
  device: parseInt(userData.id)
})

const options = ref([
    { text: 'Constant', value: 'C' },
    { text: 'Temporary', value: 'T' },
    { text: 'One use', value: 'O' }
])

const choices = ref([
    { value: '-', text: '0' },
    { value: '+1h', text: '1 hour' },
    { value: '+1d', text: '1 day' },
    { value: '+1w', text: '1 week' },
    { value: '+2w', text: '2 week' },
])

keys.value = await invoke('get_keys', { serialNum: userData.serial_num });

onBeforeUnmount(() => {
    clearInterval(setVar.value)
});

setVar.value = setInterval(async() => {
    keys.value = await invoke('get_keys', { serialNum: userData.serial_num });
}, 1000);

function generateKey() {
  let number = Math.floor(Math.random() * 12) + 4;
  var chars = "0123456789"
  var password = ""
  for (var i = 0; i < number; i++) {
    var randomNumber = Math.floor(Math.random() * chars.length);
    password += chars.substring(randomNumber, randomNumber + 1);
  }
  return parseInt(password)
}

async function createKey() {
  form.time_end = new Date(form.time_end).toISOString().split('.')[0]+"Z";
  await invoke('create_key', { form: form, serialNum: userData.serial_num });
  form.time_end = new Date().toLocaleString('ru-RU', params)
}

async function deleteKey(pk) {
  await invoke('delete_key', { pk: pk, serialNum: userData.serial_num });
}

</script>

<template>
  <Header>
    <button type="submit" 
            class="col-start-1 rounded-lg text-center p-1 btn-device m-2 text-white"
            aria-controls="hs-scale-animation-modal" data-hs-overlay="#hs-scale-animation-modal">
            Добавить ключ
    </button>
    <button type="submit" 
            class="col-end-10 rounded-lg text-center p-1 btn-device m-2 text-white"
            @click="router.back()">
            Вернуться к устройствам
    </button>
  </Header>


<div id="hs-scale-animation-modal" class="hs-overlay hidden size-full fixed top-0 start-0 z-[80] overflow-x-hidden overflow-y-auto pointer-events-none" role="dialog" tabindex="-1" aria-labelledby="hs-scale-animation-modal-label">
  <div class="hs-overlay-animation-target hs-overlay-open:scale-100 hs-overlay-open:opacity-100 scale-95 opacity-0 ease-in-out transition-all duration-300 sm:max-w-lg sm:w-full m-3 sm:mx-auto min-h-[calc(100%-3.5rem)] flex items-center">

    <div class="w-full flex flex-col bg-white border shadow-sm rounded-xl pointer-events-auto dark:bg-neutral-800 dark:border-neutral-700 dark:shadow-neutral-700/70">

      <div class="grid grid-cols-12 items-center py-3 px-3 border-b dark:border-neutral-700 border-solid">
        <h3 id="hs-scale-animation-modal-label" class="col-span-11 text-center font-bold text-gray-800 dark:text-white">
          Add new keys
        </h3>
        
        <div class="flex justify-end col-end-13">
          <button type="button" class="size-8 inline-flex justify-center items-center gap-x-2 rounded-full border border-transparent bg-gray-100 text-gray-800 hover:bg-gray-200 focus:outline-none focus:bg-gray-200 disabled:opacity-50 disabled:pointer-events-none dark:bg-neutral-700 dark:hover:bg-neutral-600 dark:text-neutral-400 dark:focus:bg-neutral-600" aria-label="Close" data-hs-overlay="#hs-scale-animation-modal">
            <span class="sr-only">Close</span>
            <svg class="shrink-0 size-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M18 6 6 18"></path>
              <path d="m6 6 12 12"></path>
            </svg>
          </button>
        </div>

      </div>

      <form class="space-y-5 px-24 py-7" method="post" @submit.prevent="createKey()">
        <div>
          <label for="key" class="text-center block font-medium text-gray-900">Key</label>
          <div class="mt-2">
            <input type="text" id="key" name="key" 
              v-model="form.key" required autocomplete="key" 
              maxlength="16" minlength="4"
              class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
          </div>
        </div>

        <div>
          <label for="used" class="text-center block font-medium text-gray-900">Used</label>
          <div class="mt-2">
            <select v-model="form.used" class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6">
              <option v-for="option in options" :value="option.value">
                {{ option.text }}
              </option>
            </select>
          </div>
        </div>

        <div>
          <label for="time_end" class="text-center block font-medium text-gray-900">Time end</label>
          <div class="mt-2">
            <input type="text" id="time_end" name="time_end" 
              v-model="form.time_end" required autocomplete="time_end"
              class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
          </div>
        </div>

        <div>
          <label for="selection" class="text-center block font-medium text-gray-900">Selection</label>
          <div class="mt-2">
            <select v-model="form.selection" class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6">
              <option v-for="choice in choices" :value="choice.value">
                {{ choice.text }}
              </option>
            </select>
          </div>
        </div>

        <div class="grid grid-rows-2 items-center gap-2 py-3 dark:border-neutral-700">
        
          <button type="submit" class="py-2 px-3 inline-flex justify-center items-center gap-x-2 text-sm font-medium rounded-lg border border-transparent bg-blue-600 text-white hover:bg-blue-700 focus:outline-none focus:bg-blue-700 disabled:opacity-50 disabled:pointer-events-none">
            Add
          </button>

          <button type="button" class="border-solid py-2 px-3 inline-flex justify-center items-center gap-x-2 text-sm font-medium rounded-lg border border-gray-200 bg-white text-gray-800 shadow-sm hover:bg-gray-50 focus:outline-none focus:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none dark:bg-neutral-800 dark:border-neutral-700 dark:text-white dark:hover:bg-neutral-700 dark:focus:bg-neutral-700" data-hs-overlay="#hs-scale-animation-modal">
            Close
          </button>

        </div>

      </form>
    </div>
  </div>
</div>


  <div class="grid grid-cols-1 sm:grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4 p-2 pt-5">
      <div v-for="key in keys">
          <div class="max-w-sm rounded-lg overflow-hidden shadow-xl w-50 lg:w-100 p-4 border border-black-500 box-device">
              <div class="grid grid-cols-2 py-0.5">
                  <label class="text-center">Key</label>
                  <div class="text-center">
                      {{ key.key }}
                  </div>
              </div>
              <div class="grid grid-cols-2 py-0.5">
                  <label class="text-center">Used</label>
                  <div class="text-center">
                      {{ key.used }}
                  </div>
              </div>
              <div class="grid grid-cols-2 py-0.5">
                  <label class="text-center">Time end</label>
                  <div class="text-center">
                      {{ new Date(key.time_end).toLocaleString('ru-RU') }}
                  </div>
              </div>
              <div class="mt-2">
                  <button type="button" @click="deleteKey(key.id)" class="items-center w-full p-1 rounded-lg btn-device">Удалить</button>
              </div>
          </div>            
      </div>
  </div>
</template>


