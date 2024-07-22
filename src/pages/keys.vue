<script setup>
import { useUserData } from '~/store/authuser';
import Header from '../components/Header.vue';
import { invoke } from '@tauri-apps/api/core';

const router = useRouter();

const userData = useUserData();

console.log(userData.serial_num)

const keys = ref([]);
const setVar = ref('');

onBeforeUnmount(() => {
    clearInterval(setVar.value)
});

onBeforeMount(() => {
  setVar.value = setInterval(async() => {
      keys.value = await invoke('get_keys', { serialNum: userData.serial_num });
  }, 1000);
})

</script>

<template>
  <Header>
    <button type="submit" 
            class="col-end-10 rounded-lg text-center p-1 border border-black-200 m-2 hover:bg-gray-200"
            @click="router.push('/devices')">
            Вернуться к устройствам
        </button>
  </Header>
  <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4 p-10 pt-5">
      <div v-for="key in keys">
          <div class="max-w-sm rounded-lg overflow-hidden shadow-xl w-50 lg:w-100 p-4 border border-black-500">
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
                      {{ new Date(key.time_end).toLocaleString('ru', { timeZone: "UTC" }) }}
                  </div>
              </div>
              <div class="mt-2">
                  <button type="button" @click="" class="items-center w-full p-1 border border-black-1000 hover:bg-gray-200 rounded">Удалить</button>
              </div>
          </div>            
      </div>
  </div>
</template>


