<template>
    <div class="flex flex-col items-center justify-around py-12 h-full w-full">
        <span class="text-4xl font-bold text-slate-50 mb-2">{{ info?.name }}</span>
        <NuxtImg v-if="info?.picture" :src="info?.picture" class="w-52 h-w-52 bg-white p-2 rounded-sm" alt="" />
        <div class="flex items-center justify-center bg-slate-300 mt-4 p-2 rounded-sm">
            <span class="text-9xl font-bold m-0">{{ info?.score.toString().padStart(2, '0') }}</span>
        </div>
        <div class="flex items-center justify-center">
            <span class="text-6xl font-bold m-0">{{ info?.foul }}</span>
        </div>
        <div class="flex items-center justify-center">
            <div v-for="(n, index) in [1, 2, 3, 4, 5]" :key="index" @click="console.log(index)"
                :class="n <= info?.timeout ? 'bg-red-700 mt-4 mx-1 p-2 rounded-lg' : 'bg-red-200 mt-4 mx-1 p-2 rounded-lg'">
            </div>
        </div>
    </div>
</template>

<script lang="ts">
import type { TeamInfo } from "~/types/TeamInfo";

export default {
    props: {
        info: {
            type: Object as () => TeamInfo,
            required: true
        }
    },
    computed: {
    }
}
</script>

<style></style>