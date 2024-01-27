<template>
    <div>
        <div class="flex items-center justify-center w-full mb-8 ">
            <input type="text" class="text-lg text-center font-martianMono" :placeholder="defaultname" v-model="info.name">
        </div>
        <div class="flex flex-col items-center justify-center w-full mb-8">
            <span class="text-4xl font-bold mb-2">Score</span>
            <div class="flex flex-row-reverse items-center justify-around w-full">
                <button @click="emitEvent('score_step_event', { step: 'up', team: teamname })"
                    class="bg-blue-500 hover:bg-blue-600 active:bg-blue-900 text-white font-bold py-2 px-4 mb-2 rounded">
                    +
                </button>
                <span class="text-4xl font-martianMono">{{ info.score }}</span>
                <button @click="emitEvent('score_step_event', { step: 'down', team: teamname })"
                    class="bg-red-500 hover:bg-red-600 active:bg-red-900 text-white font-bold py-2 px-4 mb-2 rounded">
                    -
                </button>
            </div>
        </div>
        <div class="flex flex-col items-center justify-center w-full mb-8">
            <span class="text-4xl font-bold mb-2">Foul</span>
            <div class="flex flex-row-reverse items-center justify-around w-full">
                <button @click="emitEvent('foul_step_event', { step: 'up', team: teamname })"
                    class="bg-blue-500 hover:bg-blue-600 active:bg-blue-900 text-white font-bold py-2 px-4 mb-2 rounded">
                    +
                </button>
                <span class="text-4xl font-martianMono">{{ info.foul }}</span>
                <button @click="emitEvent('foul_step_event', { step: 'down', team: teamname })"
                    class="bg-red-500 hover:bg-red-600 active:bg-red-900 text-white font-bold py-2 px-4 mb-2 rounded">
                    -
                </button>
            </div>
        </div>
        <div class="flex flex-col items-center justify-center w-full mb-8">
            <span class="text-4xl font-bold mb-2">Timeout</span>
            <div class="flex flex-row-reverse items-center justify-around w-full">
                <button @click="emitEvent('timeout_step_event', { step: 'up', team: teamname })"
                    class="bg-blue-500 hover:bg-blue-600 active:bg-blue-900 text-white font-bold py-2 px-4 mb-2 rounded">
                    +
                </button>
                <span class="text-4xl font-martianMono">{{ info.timeout }}</span>
                <button @click="emitEvent('timeout_step_event', { step: 'down', team: teamname })"
                    class="bg-red-500 hover:bg-red-600 active:bg-red-900 text-white font-bold py-2 px-4 mb-2 rounded">
                    -
                </button>
            </div>
        </div>
    </div>
</template>

<script lang="ts">
import { emit } from "@tauri-apps/api/event";
import type { TeamInfo } from "~/types/TeamInfo";

export default {
    props: {
        info: {
            type: Object as () => TeamInfo,
            required: true
        },
        defaultname: {
            type: String,
            required: true
        },
        teamname: {
            type: String,
            required: true
        }
    },
    watch: {
        info: {
            handler(newValue) {
                this.emitEvent('team_name_event', { team: this.teamname, name: this.info.name })
            },
            deep: true
        }
    },
    methods: {
        emitEvent(event: string, data: any) {
            emit(event, data);
        },
    },
}
</script>

<style></style>